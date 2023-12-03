use self::{cartridge_header::CartridgeHeader, mbc::Mbc};

mod cartridge_header;
mod mbc;

pub struct Cartridge {
    rom: Box<[u8]>,
    sram: Box<[u8]>,
    mbc: Mbc,
}

impl Cartridge {
    pub fn new(rom: Box<[u8]>) -> Self {
        let header = CartridgeHeader::new(rom[0x100..0x150].try_into().unwrap());
        let title = std::str::from_utf8(&header.title).unwrap();
        let rom_size = header.rom_size();
        let sram_size = header.sram_size();
        let mbc = Mbc::new(header.cartridge_type, rom_size >> 14); // rom bank is 16 KiB
        println!(
            "title: {}, type: {}, rom_size: {} B, sram_size: {} B",
            title,
            match mbc {
                Mbc::NoMbc => "ROM ONLY",
                Mbc::Mbc1 { .. } => "MBC1",
            },
            rom_size,
            sram_size
        );
        assert_eq!(rom.len(), rom_size, "invalid rom size");

        Self {
            rom,
            sram: vec![0; sram_size].into(),
            mbc,
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7FFF => self.rom[self.mbc.get_addr(addr) & self.rom.len() - 1],
            0xA000..=0xBFFF => match self.mbc {
                Mbc::NoMbc => self.sram[addr as usize & (self.sram.len() - 1)],
                Mbc::Mbc1 {
                    ref sram_enable, ..
                } => {
                    if *sram_enable {
                        self.sram[addr as usize & (self.sram.len() - 1)]
                    } else {
                        0xFF
                    }
                }
            },
            _ => panic!("invalid cartridge address: 0x{:04X}", addr),
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000..=0x7FFF => self.mbc.write(addr, val),
            0xA000..=0xBFFF => match self.mbc {
                Mbc::NoMbc => self.sram[addr as usize & (self.sram.len() - 1)] = val,
                Mbc::Mbc1 {
                    ref sram_enable, ..
                } => {
                    if *sram_enable {
                        self.sram[self.mbc.get_addr(addr) & (self.sram.len() - 1)] = val
                    }
                }
            },
            _ => panic!("invalid cartridge address: 0x{:04X}", addr),
        }
    }
}
