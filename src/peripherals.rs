use crate::bootrom::BootRom;
use crate::hram::HRam;
use crate::ppu::Ppu;
use crate::wram::WRam;

const WRAM_ADDR_START: u16 = 0xC000;
const WRAM_ADDR_END: u16 = 0xFDFF;
const HRAM_ADDR_START: u16 = 0xFF80;
const HRAM_ADDR_END: u16 = 0xFFFE;
const BOOTROM_ADDR_START: u16 = 0x0000;
const BOOTROM_ADDR_END: u16 = 0x00FF;
const BOOTROM_DEACTIVE_ADDR: u16 = 0xFF50;

// ppu
const PPU_REGISTER_START: u16 = 0xFF40;
const PPU_REGISTER_END: u16 = 0xFF4B;
const VRAM_ADDR_START: u16 = 0x8000;
const VRAM_ADDR_END: u16 = 0x9FFF;
const OAM_ADDR_START: u16 = 0xFE00;
const OAM_ADDR_END: u16 = 0xFE9F;

pub struct Peripherals {
    pub bootrom: BootRom,
    pub wram: WRam,
    pub hram: HRam,
    pub ppu: Ppu,
}

impl Peripherals {
    pub fn new(bootrom: BootRom) -> Self {
        Self {
            bootrom,
            wram: WRam::new(),
            hram: HRam::new(),
            ppu: Ppu::new(),
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            BOOTROM_ADDR_START..=BOOTROM_ADDR_END => {
                if self.bootrom.is_active() {
                    self.bootrom.read(addr)
                } else {
                    self.wram.read(addr)
                }
            }
            WRAM_ADDR_START..=WRAM_ADDR_END => self.wram.read(addr),
            HRAM_ADDR_START..=HRAM_ADDR_END => self.hram.read(addr),
            PPU_REGISTER_START..=PPU_REGISTER_END => self.ppu.read(addr),
            VRAM_ADDR_START..=VRAM_ADDR_END => self.ppu.read(addr),
            OAM_ADDR_START..=OAM_ADDR_END => self.ppu.read(addr),
            _ => 0xFF,
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            WRAM_ADDR_START..=WRAM_ADDR_END => self.wram.write(addr, val),
            BOOTROM_DEACTIVE_ADDR => self.bootrom.write(addr, val),
            HRAM_ADDR_START..=HRAM_ADDR_END => self.hram.write(addr, val),
            PPU_REGISTER_START..=PPU_REGISTER_END => self.ppu.write(addr, val),
            VRAM_ADDR_START..=VRAM_ADDR_END => self.ppu.write(addr, val),
            OAM_ADDR_START..=OAM_ADDR_END => self.ppu.write(addr, val),
            _ => (),
        }
    }
}
