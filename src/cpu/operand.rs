use crate::cpu::Cpu;
use crate::peripherals::Peripherals;

pub trait IO8<T: Copy> {
    fn read8(&mut self, bus: &Peripherals, src: T) -> u8;
    fn write8(&mut self, bus: &mut Peripherals, dst: T, val: u8);
}

pub trait IO16<T: Copy> {
    fn read16(&mut self, bus: &Peripherals, src: T) -> u16;
    fn write16(&mut self, bus: &mut Peripherals, dst: T, val: u16);
}

// 8-bit register
#[derive(Copy, Clone, Debug)]
pub enum Reg8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

// 16-bit register
#[derive(Copy, Clone, Debug)]
pub enum Reg16 {
    AF,
    BC,
    DE,
    HL,
    SP,
}

// read 8bit from PC
#[derive(Copy, Clone, Debug)]
pub struct Imm8;

// read 16bit from PC
#[derive(Copy, Clone, Debug)]
pub struct Imm16;

// read 8 bit from 16 bit register or two 8 bit registers
#[derive(Copy, Clone, Debug)]
pub enum Indirect {
    BC,
    DE,
    HL,
    CFF,
    HLD,
    HLI,
}

// read 8 bit from PC addressed register
#[derive(Copy, Clone, Debug)]
pub enum Direct8 {
    D,
    DFF,
}

// read 16 bit from PC addressed register
#[derive(Copy, Clone, Debug)]
pub struct Direct16;

// flag condition
#[derive(Copy, Clone, Debug)]
pub enum Cond {
    NZ,
    Z,
    NC,
    C,
}

impl IO8<Reg8> for Cpu {
    fn read8(&mut self, _: &Peripherals, src: Reg8) -> u8 {
        match src {
            Reg8::A => self.registers.a,
            Reg8::B => self.registers.b,
            Reg8::C => self.registers.c,
            Reg8::D => self.registers.d,
            Reg8::E => self.registers.e,
            Reg8::H => self.registers.h,
            Reg8::L => self.registers.l,
        }
    }

    fn write8(&mut self, _: &mut Peripherals, dst: Reg8, val: u8) {
        match dst {
            Reg8::A => self.registers.a = val,
            Reg8::B => self.registers.b = val,
            Reg8::C => self.registers.c = val,
            Reg8::D => self.registers.d = val,
            Reg8::E => self.registers.e = val,
            Reg8::H => self.registers.h = val,
            Reg8::L => self.registers.l = val,
        }
    }
}

impl IO16<Reg16> for Cpu {
    fn read16(&mut self, _: &Peripherals, src: Reg16) -> u16 {
        match src {
            Reg16::AF => self.registers.af(),
            Reg16::BC => self.registers.bc(),
            Reg16::DE => self.registers.de(),
            Reg16::HL => self.registers.hl(),
            Reg16::SP => self.registers.sp,
        }
    }

    fn write16(&mut self, _: &mut Peripherals, dst: Reg16, val: u16) {
        match dst {
            Reg16::AF => self.registers.write_af(val),
            Reg16::BC => self.registers.write_bc(val),
            Reg16::DE => self.registers.write_de(val),
            Reg16::HL => self.registers.write_hl(val),
            Reg16::SP => self.registers.sp = val,
        }
    }
}

impl IO8<Imm8> for Cpu {
    fn read8(&mut self, bus: &Peripherals, _: Imm8) -> u8 {
        let val = self.read_bus(bus, self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        return val;
    }

    fn write8(&mut self, _: &mut Peripherals, _: Imm8, _: u8) {
        unreachable!()
    }
}

// 2 M-cycle
impl IO16<Imm16> for Cpu {
    fn read16(&mut self, bus: &Peripherals, _: Imm16) -> u16 {
        let lo = self.read8(bus, Imm8);
        let hi = self.read8(bus, Imm8);
        u16::from_le_bytes([lo, hi])
    }

    fn write16(&mut self, _: &mut Peripherals, _: Imm16, _: u16) {
        unreachable!()
    }
}

impl IO8<Indirect> for Cpu {
    fn read8(&mut self, bus: &Peripherals, src: Indirect) -> u8 {
        match src {
            Indirect::BC => self.read_bus(bus, self.registers.bc()),
            Indirect::DE => self.read_bus(bus, self.registers.de()),
            Indirect::HL => self.read_bus(bus, self.registers.hl()),
            Indirect::CFF => self.read_bus(bus, 0xFF00 | u16::from(self.registers.c)),
            Indirect::HLD => {
                let addr = self.registers.hl();
                self.registers.write_hl(addr.wrapping_sub(1));
                self.read_bus(bus, addr)
            }
            Indirect::HLI => {
                let addr = self.registers.hl();
                self.registers.write_hl(addr.wrapping_add(1));
                self.read_bus(bus, addr)
            }
        }
    }

    fn write8(&mut self, bus: &mut Peripherals, dst: Indirect, val: u8) {
        match dst {
            Indirect::BC => self.write_bus(bus, self.registers.bc(), val),
            Indirect::DE => self.write_bus(bus, self.registers.de(), val),
            Indirect::HL => self.write_bus(bus, self.registers.hl(), val),
            Indirect::CFF => self.write_bus(bus, 0xFF00 | u16::from(self.registers.c), val),
            Indirect::HLD => {
                let addr = self.registers.hl();
                self.registers.write_hl(addr.wrapping_sub(1));
                self.write_bus(bus, addr, val)
            }
            Indirect::HLI => {
                let addr = self.registers.hl();
                self.registers.write_hl(addr.wrapping_add(1));
                self.write_bus(bus, addr, val)
            }
        }
    }
}

impl IO8<Direct8> for Cpu {
    fn read8(&mut self, bus: &Peripherals, src: Direct8) -> u8 {
        let lo = self.read8(bus, Imm8);
        let hi = if let Direct8::DFF = src {
            0xFF
        } else {
            self.read8(bus, Imm8)
        };
        self.read_bus(bus, u16::from_le_bytes([lo, hi]))
    }

    fn write8(&mut self, bus: &mut Peripherals, dst: Direct8, val: u8) {
        let lo = self.read8(bus, Imm8);
        let hi = if let Direct8::DFF = dst {
            0xFF
        } else {
            self.read8(bus, Imm8)
        };
        self.write_bus(bus, u16::from_le_bytes([lo, hi]), val);
    }
}

impl IO16<Direct16> for Cpu {
    fn read16(&mut self, _: &Peripherals, _: Direct16) -> u16 {
        unreachable!()
    }

    fn write16(&mut self, bus: &mut Peripherals, _: Direct16, val: u16) {
        let lo = self.read8(bus, Imm8);
        let hi = self.read8(bus, Imm8);
        self.write_bus(bus, u16::from_le_bytes([lo, hi]), val as u8);
        self.write_bus(
            bus,
            u16::from_le_bytes([lo, hi]).wrapping_add(1),
            (val >> 8) as u8,
        );
    }
}
