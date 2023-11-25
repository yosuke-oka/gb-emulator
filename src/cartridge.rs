#[repr(C)]
pub struct CartridgeHeader {
    entry_point: [u8; 4],
    logo: [u8; 48],
    title: [u8; 11],
    maker_code: [u8; 4],
    cgb_flag: u8,
    new_licensee_code: [u8; 2],
    sgb_flag: u8,
    cartridge_type: u8,
    rom_size: u8,
    sram_size: u8,
    destination_code: u8,
    old_licensee_code: u8,
    game_version: u8,
    header_checksum: u8,
    global_checksum: [u8; 2],
}

impl CartridgeHeader {
    fn new(data: [u8; 0x50]) -> Self {
        let ret = unsafe { std::mem::transmute::<[u8; 0x50], Self>(data) };
        let mut checksum = 0u8;
        for i in 0x34..=0x4C {
            checksum = checksum.wrapping_sub(data[i]).wrapping_sub(1);
        }
        assert_eq!(checksum, ret.header_checksum, "invalid header checksum");
        ret
    }
    fn rom_size(&self) -> usize {
        assert!(
            self.rom_size <= 0x08,
            "invalid rom size: 0x{:02X}",
            self.rom_size
        );
        1 << (15 + self.rom_size)
    }

    fn sram_size(&self) -> usize {
        match self.sram_size {
            0x00 => 0,
            0x01 => 0x800,
            0x02 => 0x2000,
            0x03 => 0x8000,
            0x04 => 0x20000,
            0x05 => 0x10000,
            _ => panic!("invalid sram size: 0x{:02X}", self.sram_size),
        }
    }
}
