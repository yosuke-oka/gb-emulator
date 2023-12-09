use crate::bootrom::BootRom;
use crate::cartridge::Cartridge;
use crate::cpu::interrupt::Interrupts;
use crate::hram::HRam;
use crate::joypad::Joypad;
use crate::lcd::LCD;
use crate::ppu::Ppu;
use crate::timer::Timer;
use crate::wram::WRam;

const WRAM_ADDR_START: u16 = 0xC000;
const WRAM_ADDR_END: u16 = 0xFDFF;
const HRAM_ADDR_START: u16 = 0xFF80;
const HRAM_ADDR_END: u16 = 0xFFFE;
const BOOTROM_ADDR_START: u16 = 0x0000;
const BOOTROM_ADDR_END: u16 = 0x00FF;
const BOOTROM_DEACTIVE_ADDR: u16 = 0xFF50;
const CARTRIDGE_ADDR1_START: u16 = 0x0100;
const CARTRIDGE_ADDR1_END: u16 = 0x7FFF;
const CARTRIDGE_ADDR2_START: u16 = 0xA000;
const CARTRIDGE_ADDR2_END: u16 = 0xBFFF;
const TIMER_ADDR_START: u16 = 0xFF04;
const TIMER_ADDR_END: u16 = 0xFF07;
const JOYPAD_ADDR: u16 = 0xFF00;

// ppu
const PPU_REGISTER_START: u16 = 0xFF40;
const PPU_REGISTER_END: u16 = 0xFF4B;
const VRAM_ADDR_START: u16 = 0x8000;
const VRAM_ADDR_END: u16 = 0x9FFF;
const OAM_ADDR_START: u16 = 0xFE00;
const OAM_ADDR_END: u16 = 0xFE9F;

pub struct Bus {
    pub bootrom: BootRom,
    pub wram: WRam,
    pub hram: HRam,
    pub ppu: Ppu,
    pub timer: Timer,
    pub joypad: Joypad,
    cartridge: Cartridge,
}

impl Bus {
    pub fn new(bootrom: BootRom, cartridge: Cartridge, lcd: LCD) -> Self {
        Self {
            bootrom,
            wram: WRam::new(),
            hram: HRam::new(),
            ppu: Ppu::new(lcd),
            timer: Timer::default(),
            joypad: Joypad::new(),
            cartridge,
        }
    }

    pub fn read(&self, interrupts: &Interrupts, addr: u16) -> u8 {
        match addr {
            BOOTROM_ADDR_START..=BOOTROM_ADDR_END => {
                if self.bootrom.is_active() {
                    self.bootrom.read(addr)
                } else {
                    self.cartridge.read(addr)
                }
            }
            WRAM_ADDR_START..=WRAM_ADDR_END => self.wram.read(addr),
            HRAM_ADDR_START..=HRAM_ADDR_END => self.hram.read(addr),
            CARTRIDGE_ADDR1_START..=CARTRIDGE_ADDR1_END => self.cartridge.read(addr),
            CARTRIDGE_ADDR2_START..=CARTRIDGE_ADDR2_END => self.cartridge.read(addr),
            TIMER_ADDR_START..=TIMER_ADDR_END => self.timer.read(addr),
            PPU_REGISTER_START..=PPU_REGISTER_END => self.ppu.read(addr),
            VRAM_ADDR_START..=VRAM_ADDR_END => self.ppu.read(addr),
            OAM_ADDR_START..=OAM_ADDR_END => self.ppu.read(addr),
            JOYPAD_ADDR => self.joypad.read(),
            0xFF0F | 0xFFFF => interrupts.read(addr),
            _ => 0xFF,
        }
    }

    pub fn write(&mut self, interrupts: &mut Interrupts, addr: u16, val: u8) {
        match addr {
            BOOTROM_ADDR_START..=BOOTROM_ADDR_END => {
                if !self.bootrom.is_active() {
                    self.cartridge.write(addr, val)
                }
            }
            WRAM_ADDR_START..=WRAM_ADDR_END => self.wram.write(addr, val),
            BOOTROM_DEACTIVE_ADDR => self.bootrom.write(addr, val),
            HRAM_ADDR_START..=HRAM_ADDR_END => self.hram.write(addr, val),
            CARTRIDGE_ADDR1_START..=CARTRIDGE_ADDR1_END => self.cartridge.write(addr, val),
            CARTRIDGE_ADDR2_START..=CARTRIDGE_ADDR2_END => self.cartridge.write(addr, val),
            TIMER_ADDR_START..=TIMER_ADDR_END => self.timer.write(addr, val),
            PPU_REGISTER_START..=PPU_REGISTER_END => self.ppu.write(addr, val),
            VRAM_ADDR_START..=VRAM_ADDR_END => self.ppu.write(addr, val),
            OAM_ADDR_START..=OAM_ADDR_END => self.ppu.write(addr, val),
            JOYPAD_ADDR => self.joypad.write(val),
            0xFF0F | 0xFFFF => interrupts.write(addr, val),
            _ => (),
        }
    }

    pub fn tick(&mut self, interrupts: &mut Interrupts) {
        self.timer.emulate_cycle(interrupts);
        if let Some(addr) = self.ppu.oam_dma {
            self.ppu.oam_dma_emulate_cycle(self.read(interrupts, addr));
            // TODO: 実装があっているか不明かつ、ppuに処理を移動したい
            // for i in 0..0xA0 {
            //     let data = self.bus.read(&self.cpu.interrupts, addr + i);
            //     self.bus.ppu.write_oam(0xFE00 + i, data);
            // }
            // self.bus.ppu.finish_oam_dma();
        }
        if self.ppu.emulate_cycle(interrupts) {
            self.ppu.draw();
        }
    }
}
