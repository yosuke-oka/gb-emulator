const LCDC_ADDR: u16 = 0xFF40;
const STAT_ADDR: u16 = 0xFF41;
const SCY_ADDR: u16 = 0xFF42;
const SCX_ADDR: u16 = 0xFF43;
const LY_ADDR: u16 = 0xFF44;
const LYC_ADDR: u16 = 0xFF45;
const BGP_ADDR: u16 = 0xFF47;
const OBP0_ADDR: u16 = 0xFF48;
const OBP1_ADDR: u16 = 0xFF49;
const WY_ADDR: u16 = 0xFF4A;
const WX_ADDR: u16 = 0xFF4B;
// TODO: 以下 peripherals にも記載しているので DRYじゃない
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
}

impl Ppu {
    pub fn new() -> Self {
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
                    self.oam[(addr - OAM_ADDR_START) as usize] = val;
                }
            }
            _ => panic!("invalid ppu address: 0x{:04X}", addr),
        }
    }
}
