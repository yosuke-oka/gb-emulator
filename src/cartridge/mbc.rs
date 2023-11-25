pub enum Mbc {
    NoMbc,
    Mbc1 {
        sram_enable: bool,
        low_bank: usize,
        high_bank: usize,
        bank_mode: bool,
        rom_banks: usize,
    },
}

impl Mbc {
    pub fn new(cartridge_type: u8, rom_banks: usize) -> Self {
        match cartridge_type {
            0x00 | 0x08 | 0x09 => Self::NoMbc,
            0x01 | 0x02 | 0x03 => Self::Mbc1 {
                sram_enable: false,
                low_bank: 1,
                high_bank: 0,
                bank_mode: false,
                rom_banks,
            },
            _ => panic!("unsupported cartridge type: 0x{:02X}", cartridge_type),
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match *self {
            Self::NoMbc => {}
            Self::Mbc1 {
                ref mut sram_enable,
                ref mut low_bank,
                ref mut high_bank,
                ref mut bank_mode,
                ..
            } => match addr {
                0x0000..=0x1FFF => *sram_enable = val & 0xF == 0xA,
                0x2000..=0x3FFF => {
                    let val = val & 0x1F;
                    *low_bank = if val == 0 { 1 } else { val as usize };
                }
                0x4000..=0x5FFF => {
                    let val = val & 0x03;
                    *high_bank = (val & 0x03) as usize;
                }
                0x6000..=0x7FFF => *bank_mode = val & 0x01 == 0x01,
                _ => panic!("invalid mbc1 address: 0x{:04X}", addr),
            },
        }
    }

    pub fn get_addr(&self, addr: u16) -> usize {
        match *self {
            Self::NoMbc => addr as usize,
            Self::Mbc1 {
                low_bank,
                high_bank,
                bank_mode,
                rom_banks,
                ..
            } => match addr {
                0x0000..=0x3FFF => {
                    if bank_mode {
                        (high_bank << 19) | (addr & 0x3FFF) as usize
                    } else {
                        (addr & 0x3FFF) as usize
                    }
                }
                0x4000..=0x7FFF => {
                    (high_bank << 19)
                        | (low_bank & (rom_banks - 1)) << 14
                        | (addr as usize & 0x3FFF)
                }
                0xA000..=0xBFFF => {
                    if bank_mode {
                        (high_bank << 13) | (addr & 0x1FFF) as usize
                    } else {
                        (addr & 0x1FFF) as usize
                    }
                }
                _ => panic!("invalid mbc1 address: 0x{:04X}", addr),
            },
        }
    }
}
