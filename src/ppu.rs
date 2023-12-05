use std::iter;

use crate::{
    cpu::interrupt::{self, Interrupts},
    lcd::LCD,
};

pub const LCD_WIDTH: usize = 160;
pub const LCD_HEIGHT: usize = 144;
pub const LCD_PIXELS: usize = LCD_WIDTH * LCD_HEIGHT;

const LCDC_ADDR: u16 = 0xFF40;
const STAT_ADDR: u16 = 0xFF41;
const SCY_ADDR: u16 = 0xFF42;
const SCX_ADDR: u16 = 0xFF43;
const LY_ADDR: u16 = 0xFF44;
const LYC_ADDR: u16 = 0xFF45;
const DMA_ADDR: u16 = 0xFF46;
const BGP_ADDR: u16 = 0xFF47;
const OBP0_ADDR: u16 = 0xFF48;
const OBP1_ADDR: u16 = 0xFF49;
const WY_ADDR: u16 = 0xFF4A;
const WX_ADDR: u16 = 0xFF4B;
// TODO: 以下 bus にも記載しているので DRYじゃない
const VRAM_ADDR_START: u16 = 0x8000;
const VRAM_ADDR_END: u16 = 0x9FFF;
const OAM_ADDR_START: u16 = 0xFE00;
const OAM_ADDR_END: u16 = 0xFE9F;

// 0xFF40 LCDC register
const PPU_ENABLE: u8 = 1 << 7; //0b1000_0000
const WINDOW_TILE_MAP: u8 = 1 << 6;
const WINDOW_ENABLE: u8 = 1 << 5;
const TILE_DATA_ADDRESSING_MODE: u8 = 1 << 4;
const BG_TILE_MAP: u8 = 1 << 3;
const SPRITE_SIZE: u8 = 1 << 2;
const SPRITE_ENABLE: u8 = 1 << 1;
const BG_WINDOW_ENABLE: u8 = 1 << 0;

// 0xFF41 STAT register
const LYC_LY_INTERRUPT: u8 = 1 << 6;
const OAM_INTERRUPT: u8 = 1 << 5;
const VBLANK_INTERRUPT: u8 = 1 << 4;
const HBLANK_INTERRUPT: u8 = 1 << 3;
const LYC_LY_COINCIDENCE: u8 = 1 << 2;

// sprite flags
const OBJ2BG_PRIORITY: u8 = 1 << 7;
const Y_FLIP: u8 = 1 << 6;
const X_FLIP: u8 = 1 << 5;
const PALETTE: u8 = 1 << 4;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    HBlank = 0,
    VBlank = 1,
    OAMScan = 2,
    Drawing = 3,
}

pub struct Ppu {
    mode: Mode,
    lcdc: u8,                // lcd control
    stat: u8,                // lcd status
    scy: u8,                 // scroll y
    scx: u8,                 // scroll x
    ly: u8,                  // line y
    lyc: u8,                 // line y compare
    bgp: u8,                 // bg palette
    obp0: u8,                // object palette 0
    obp1: u8,                // object palette 1
    wy: u8,                  // window y
    wx: u8,                  // window x
    vram: Box<[u8; 0x2000]>, // 8 KiB video ram
    oam: Box<[u8; 0xA0]>,    // 160 B object attribute memory
    pub oam_dma: Option<u16>,
    buffer: Box<[u8; LCD_PIXELS * 4]>,
    cycles: u16,
    lcd: LCD, // TODO: 一般化
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Sprite {
    pub y: u8,
    pub x: u8,
    pub tile_idx: u8,
    pub flags: u8,
}

impl Ppu {
    pub fn new(lcd: LCD) -> Self {
        Self {
            mode: Mode::OAMScan,
            lcdc: 0,
            stat: 0,
            scy: 0,
            scx: 0,
            ly: 0,
            lyc: 0,
            bgp: 0,
            obp0: 0,
            obp1: 0,
            wy: 0,
            wx: 0,
            vram: Box::new([0; 0x2000]),
            oam: Box::new([0; 0xA0]),
            oam_dma: None,
            buffer: Box::new([0; LCD_PIXELS * 4]),
            cycles: 20, // OAM scan mode needs 20 cycles
            lcd,
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            LCDC_ADDR => self.lcdc,
            STAT_ADDR => 0x80 | self.stat | self.mode as u8,
            SCY_ADDR => self.scy,
            SCX_ADDR => self.scx,
            LY_ADDR => self.ly,
            LYC_ADDR => self.lyc,
            DMA_ADDR => {
                if self.oam_dma.is_some() {
                    0xFF
                } else {
                    self.oam[addr as usize & 0xFF]
                }
            }
            BGP_ADDR => self.bgp,
            OBP0_ADDR => self.obp0,
            OBP1_ADDR => self.obp1,
            WY_ADDR => self.wy,
            WX_ADDR => self.wx,
            VRAM_ADDR_START..=VRAM_ADDR_END => {
                if self.mode == Mode::Drawing {
                    0xFF // can not read vram during drawing
                } else {
                    self.vram[(addr - VRAM_ADDR_START) as usize]
                }
            }
            OAM_ADDR_START..=OAM_ADDR_END => {
                if self.mode == Mode::OAMScan || self.mode == Mode::Drawing {
                    0xFF // can not read oam during oam scan or drawing
                } else {
                    self.oam[(addr - OAM_ADDR_START) as usize]
                }
            }
            _ => panic!("invalid ppu address: 0x{:04X}", addr),
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            LCDC_ADDR => self.lcdc = val,
            STAT_ADDR => self.stat = (self.stat & LYC_LY_COINCIDENCE) | (val & 0x78), // can not write 0-2 bits
            SCY_ADDR => self.scy = val,
            SCX_ADDR => self.scx = val,
            LY_ADDR => self.ly = 0,
            LYC_ADDR => self.lyc = val,
            DMA_ADDR => {
                self.oam_dma = Some((val as u16) << 8);
            }
            BGP_ADDR => self.bgp = val,
            OBP0_ADDR => self.obp0 = val,
            OBP1_ADDR => self.obp1 = val,
            WY_ADDR => self.wy = val,
            WX_ADDR => self.wx = val,
            VRAM_ADDR_START..=VRAM_ADDR_END => {
                if self.mode != Mode::Drawing {
                    // can not write vram during drawing
                    self.vram[(addr - VRAM_ADDR_START) as usize] = val;
                }
            }
            OAM_ADDR_START..=OAM_ADDR_END => {
                if self.mode != Mode::OAMScan && self.mode != Mode::Drawing {
                    // can not write oam during oam scan or drawing
                    if self.oam_dma.is_none() {
                        self.oam[(addr - OAM_ADDR_START) as usize] = val;
                    }
                }
            }
            _ => panic!("invalid ppu address: 0x{:04X}", addr),
        }
    }

    // tile data: 16 bytes * 0x180
    // tile: 16 bytes
    // pixex: 2 bits

    fn get_pixel_from_tile(&self, tile_idx: usize, row: u8, col: u8) -> u8 {
        let r = (row << 1) as usize; // 2 bytes per row
        let c = (7 - col) as usize; // col is (7-col) bit
        let tile_addr = tile_idx << 4;
        let low = self.vram[(tile_addr | r) & 0x1FFF];
        let high = self.vram[(tile_addr | (r + 1)) & 0x1FFF];
        ((high >> c) & 1) << 1 | ((low >> c) & 1)
    }

    fn get_tile_idx_from_tile_map(&self, tile_map: bool, row: u8, col: u8) -> usize {
        let start_addr = 0x1800 | ((tile_map as usize) << 10);
        let ret = self.vram[start_addr | ((row as usize) << 5) + col as usize & 0x3FF];
        if self.lcdc & TILE_DATA_ADDRESSING_MODE == 0 {
            // 0x8800-0x97FF
            (ret as i8 as i16 + 0x100) as usize
        } else {
            // 0x8000-0x8FFF
            ret as usize
        }
    }

    fn render_window(&mut self) {
        if self.lcdc & WINDOW_ENABLE == 0 || self.lcdc & BG_WINDOW_ENABLE == 0 || self.wy > self.ly
        {
            return;
        }
        let mut wy_add = 0;
        let y = self.wy;
        for i in 0..LCD_WIDTH {
            let (x, overflow) = (i as u8).overflowing_sub(self.wx.wrapping_sub(7));
            if overflow {
                continue;
            }

            wy_add = 1;

            let tile_index =
                self.get_tile_idx_from_tile_map(self.lcdc & WINDOW_TILE_MAP > 0, y >> 3, x >> 3);

            let pixel = self.get_pixel_from_tile(tile_index, y & 7, x & 7);

            self.buffer[LCD_WIDTH * self.ly as usize + i] = match (self.bgp >> (pixel << 1)) & 0b11
            {
                0b00 => 0xFF,
                0b01 => 0xAA,
                0b10 => 0x55,
                0b11 => 0x00,
                _ => unreachable!(),
            }
        }
        self.wy += wy_add;
    }

    fn render_sprite(&mut self, bg_prio: &[bool; LCD_WIDTH]) {
        if self.lcdc & SPRITE_ENABLE == 0 {
            return;
        }

        let size = if self.lcdc & SPRITE_SIZE == 0 { 8 } else { 16 };

        let mut sprites: Vec<Sprite> =
            unsafe { std::mem::transmute::<[u8; 0xA0], [Sprite; 40]>(self.oam.as_ref().clone()) }
                .into_iter()
                .filter_map(|mut sprite| {
                    sprite.y = sprite.y.wrapping_sub(16);
                    sprite.x = sprite.x.wrapping_sub(8);
                    if self.ly.wrapping_sub(sprite.y) < size {
                        Some(sprite)
                    } else {
                        None
                    }
                })
                .take(10)
                .collect();
        sprites.reverse();
        sprites.sort_by(|&a, &b| b.x.cmp(&a.x));

        for sprite in sprites {
            let palette = if sprite.flags & PALETTE == 0 {
                self.obp0
            } else {
                self.obp1
            };
            let mut tile_idx = sprite.tile_idx as usize;
            let mut row = if sprite.flags & Y_FLIP == 0 {
                self.ly.wrapping_sub(sprite.y)
            } else {
                size - 1 - self.ly.wrapping_sub(sprite.y)
            };
            if size == 16 {
                tile_idx &= 0xFE;
            }
            tile_idx += (row >= 8) as usize;

            row &= 7;

            for col in 0..8 {
                let col_flipped = if sprite.flags & X_FLIP == 0 {
                    col
                } else {
                    7 - col
                };
                let pixel = self.get_pixel_from_tile(tile_idx, row, col_flipped);
                let i = sprite.x.wrapping_add(col) as usize;
                if i < LCD_WIDTH && pixel != 0 {
                    if sprite.flags & OBJ2BG_PRIORITY == 0 || !bg_prio[i] {
                        self.buffer[LCD_WIDTH * self.ly as usize + i] =
                            match (palette >> (pixel << 1)) & 0b11 {
                                0b00 => 0xFF,
                                0b01 => 0xAA,
                                0b10 => 0x55,
                                0b11 => 0x00,
                                _ => unreachable!(),
                            }
                    }
                }
            }
        }
    }

    fn render_bg(&mut self) {
        if self.lcdc & BG_WINDOW_ENABLE == 0 {
            return;
        }
        let y = self.ly.wrapping_add(self.scy);
        for i in 0..LCD_WIDTH {
            let x = (i as u8).wrapping_add(self.scx);
            let tile_idx =
                self.get_tile_idx_from_tile_map(self.lcdc & BG_TILE_MAP != 0, y >> 3, x >> 3);
            let pixel = self.get_pixel_from_tile(tile_idx, y & 7, x & 7);
            self.buffer[LCD_WIDTH * self.ly as usize + i] = match (self.bgp >> (pixel << 1)) & 0b11
            {
                0b00 => 0xFF,
                0b01 => 0xAA,
                0b10 => 0x55,
                0b11 => 0x00,
                _ => unreachable!(),
            }
        }
    }

    fn check_lyc_eq_ly(&mut self, interrupts: &mut Interrupts) {
        if self.ly == self.lyc {
            self.stat |= LYC_LY_COINCIDENCE;
            if self.stat & LYC_LY_INTERRUPT != 0 {
                interrupts.irq(interrupt::LCD_STAT);
            }
        } else {
            self.stat &= !LYC_LY_COINCIDENCE;
        }
    }

    // drawing (mode: 3) のときは OAM と VRAM にアクセスできないので、
    // M-cycle ごとの厳密な実装ではなく、 drawing の際にレンダリングすることで実装を簡略化している
    pub fn emulate_cycle(&mut self, interrupts: &mut Interrupts) -> bool {
        if self.lcdc & PPU_ENABLE == 0 {
            return false;
        }

        self.cycles -= 1;
        if self.cycles > 0 {
            return false;
        }

        let mut need_vsync = false;

        match self.mode {
            Mode::HBlank => {
                self.ly += 1;
                if self.ly < 144 {
                    self.mode = Mode::OAMScan;
                    self.cycles = 20;
                    if self.stat & OAM_INTERRUPT != 0 {
                        interrupts.irq(interrupt::LCD_STAT);
                    }
                } else {
                    self.mode = Mode::VBlank;
                    self.cycles = 114;
                    interrupts.irq(interrupt::VBLANK);
                    if self.stat & VBLANK_INTERRUPT != 0 {
                        interrupts.irq(interrupt::LCD_STAT);
                    }
                }
                self.check_lyc_eq_ly(interrupts);
            }
            Mode::VBlank => {
                self.ly += 1;
                if self.ly > 153 {
                    self.ly = 0;
                    self.wy = 0;
                    self.mode = Mode::OAMScan;
                    self.cycles = 20;
                    need_vsync = true;
                    if self.stat & OAM_INTERRUPT != 0 {
                        interrupts.irq(interrupt::LCD_STAT);
                    }
                } else {
                    self.cycles = 114;
                }
                self.check_lyc_eq_ly(interrupts);
            }
            Mode::OAMScan => {
                self.mode = Mode::Drawing;
                self.cycles = 43;
            }
            Mode::Drawing => {
                let mut bg_prio = [false; LCD_WIDTH];
                self.render_bg();
                self.render_window();
                self.render_sprite(&bg_prio);
                self.mode = Mode::HBlank;
                self.cycles = 51;
                if self.stat & HBLANK_INTERRUPT != 0 {
                    interrupts.irq(interrupt::LCD_STAT);
                }
            }
        }
        need_vsync
    }

    pub fn oam_dma_emulate_cycle(&mut self, val: u8) {
        if let Some(addr) = self.oam_dma {
            if self.mode != Mode::OAMScan && self.mode != Mode::Drawing {
                // can not write oam during oam scan or drawing
                self.oam[addr as usize & 0xFF] = val;
            }
            self.oam_dma = Some(addr.wrapping_add(1)).filter(|&x| (x as u8) < 0xA0);
        }
    }

    pub fn write_oam(&mut self, addr: u16, val: u8) {
        if self.mode != Mode::OAMScan && self.mode != Mode::Drawing {
            // can not write oam during oam scan or drawing
            self.oam[addr as usize & 0xFF] = val;
        }
    }

    pub fn finish_oam_dma(&mut self) {
        self.oam_dma = None;
        self.cycles = 160;
    }

    // For LCD
    pub fn pixel_buffer(&self) -> Box<[u8]> {
        self.buffer
            .iter()
            .flat_map(|&e| iter::repeat(e.into()).take(3))
            .collect::<Box<[u8]>>()
    }

    pub fn draw(&mut self) {
        self.lcd.draw(&self.pixel_buffer());
    }
}
