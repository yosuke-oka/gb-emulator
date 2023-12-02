use crate::bootrom::BootRom;
use crate::cartridge::Cartridge;
use crate::cpu::interrupt::Interrupts;
use crate::hram::HRam;
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
    pub timer: Timer,
    cartridge: Cartridge,
}

impl Peripherals {
    pub fn new(bootrom: BootRom, cartridge: Cartridge) -> Self {
        Self {
            bootrom,
            wram: WRam::new(),
            hram: HRam::new(),
            ppu: Ppu::new(),
            timer: Timer::default(),
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
            0xFF0F | 0xFFFF => interrupts.write(addr, val),
            _ => (),
        }
    }
}
