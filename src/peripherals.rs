pub struct Peripherals {
    pub bootrom: BootRom,
    pub wram: WRam,
    pub hram: HRam,
}

const WRAM_ADDR_RANGE: RangeInclusive<u16> = 0xC000..=0xFDFF;
const HRAM_ADDR_RANGE: RangeInclusive<u16> = 0xFF80..=0xFFFE;
const BOOTROM_ADDR_RANGE: RangeInclusive<u16> = 0x0000..=0x00FF;
const BOOTROM_WRITE_ADDR: u16 = 0xFF50;

impl Peripherals {
    pub fn new(bootrom: BootRom) -> Self {
        Self {
            bootrom,
            wram: WRam::new(),
            hram: HRam::new(),
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match adadr {
            BOOTROM_ADDR_RANGE => {
                if self.bootrom.is_active() {
                    self.bootrom.read(addr)
                } else {
                    self.wram.read(addr)
                }
            }
            WRAM_ADDR_RANGE => self.wram.read(addr),
            HRAM_ADDR_RANGE => self.hram.read(addr),
            _ => 0xFF,
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            WRAM_ADDR_RANGE => self.wram.write(addr, val),
            BOOTROM_WRITE_ADDR => self.bootrom.write(addr, val),
            HRAM_ADDR_RANGE => self.hram.write(addr, val),
            _ => (),
        }
    }
}
