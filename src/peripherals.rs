use crate::bootrom::BootRom;
use crate::hram::HRam;
use crate::wram::WRam;

const WRAM_ADDR_START: u16 = 0xC000;
const WRAM_ADDR_END: u16 = 0xFDFF;
const HRAM_ADDR_START: u16 = 0xFF80;
const HRAM_ADDR_END: u16 = 0xFFFE;
const BOOTROM_ADDR_START: u16 = 0x0000;
const BOOTROM_ADDR_END: u16 = 0x00FF;
const BOOTROM_DEACTIVE_ADDR: u16 = 0xFF50;

pub struct Peripherals {
    pub bootrom: BootRom,
    pub wram: WRam,
    pub hram: HRam,
}

impl Peripherals {
    pub fn new(bootrom: BootRom) -> Self {
        Self {
            bootrom,
            wram: WRam::new(),
            hram: HRam::new(),
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
            _ => 0xFF,
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            WRAM_ADDR_START..=WRAM_ADDR_END => self.wram.write(addr, val),
            BOOTROM_DEACTIVE_ADDR => self.bootrom.write(addr, val),
            HRAM_ADDR_START..=HRAM_ADDR_END => self.hram.write(addr, val),
            _ => (),
        }
    }
}
