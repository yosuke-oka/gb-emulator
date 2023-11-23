pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
}

impl Registers {
    pub fn af(&self) -> u16 {
        (self.a as u16) << 8 | (self.f as u16)
    }

    pub fn write_af(&mut self, val: u16) {
        self.a = (val >> 8) as u8;
        self.f = (val & 0xF0) as u8; // lower 4 bits in flag register are always zero
    }

    pub fn bc(&self) -> u16 {
        (self.b as u16) << 8 | (self.c as u16)
    }

    pub fn write_bc(&mut self, val: u16) {
        self.b = (val >> 8) as u8;
        self.c = val as u8;
    }

    pub fn de(&self) -> u16 {
        (self.d as u16) << 8 | (self.e as u16)
    }

    pub fn write_de(&mut self, val: u16) {
        self.d = (val >> 8) as u8;
        self.e = val as u8;
    }

    pub fn hl(&self) -> u16 {
        (self.h as u16) << 8 | (self.l as u16)
    }

    pub fn write_hl(&mut self, val: u16) {
        self.h = (val >> 8) as u8;
        self.l = val as u8;
    }

    // flag register, z flag, n flag, h flag, c flag
    pub fn zf(&self) -> bool {
        self.f & 0x80 != 0
    }

    pub fn set_zf(&mut self, zf: bool) {
        if zf {
            self.f |= 0x80;
        } else {
            self.f &= 0x7F;
        }
    }

    pub fn nf(&self) -> bool {
        self.f & 0x40 != 0
    }

    pub fn set_nf(&mut self, nf: bool) {
        if nf {
            self.f |= 0x40;
        } else {
            self.f &= 0xBF;
        }
    }

    pub fn hf(&self) -> bool {
        self.f & 0x20 != 0
    }

    pub fn set_hf(&mut self, hf: bool) {
        if hf {
            self.f |= 0x20;
        } else {
            self.f &= 0xDF;
        }
    }

    pub fn cf(&self) -> bool {
        self.f & 0x10 != 0
    }

    pub fn set_cf(&mut self, cf: bool) {
        if cf {
            self.f |= 0x10;
        } else {
            self.f &= 0xEF;
        }
    }
}
