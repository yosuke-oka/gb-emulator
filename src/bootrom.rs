pub struct BootRom {
    rom: Box<[u8]>,
    active: bool,
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

    // Boot ROM に対する書き込みができるわけではなさそうなので、addr, val は Pheripherals に移動させた方が良いかも？
    pub fn write(&mut self, _: u16, val: u8) {
        self.active &= val == 0;
    }
}
