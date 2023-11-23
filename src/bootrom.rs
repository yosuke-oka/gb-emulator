pub struct BootRom {
    rom: Box<[u8]>,
}

impl BootRom {
    pub fn new(rom: Box<[u8]>) -> Self {
        Self { rom, active: true }
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.rom[addr as usize]
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn write(&mut self, _: u16, val: u8) {
        self.active &= val == 0;
    }
}
