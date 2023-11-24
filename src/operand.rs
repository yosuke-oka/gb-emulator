use std::sync::atomic::{AtomicU16, AtomicU8, Ordering::Relaxed};
pub trait IO8<T: Copy> {
    fn read8(&self, bus: &Peripherals, src: T) -> Option<u8>;
    fn write8(&mut self, bus: &mut Peripherals, dst: T, val: u8) -> Option<()>;
}

pub trait IO16<T: Copy> {
    fn read16(&self, bus: &Peripherals, src: T) -> Option<u16>;
    fn write16(&mut self, bus: &mut Peripherals, dst: T, val: u16) -> Option<()>;
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
    fn read8(&mut self, _: &Peripherals, src: Reg8) -> Option<u8> {
        match src {
            Reg8::A => Some(self.regiters.a),
            Reg8::B => Some(self.regiters.b),
            Reg8::C => Some(self.regiters.c),
            Reg8::D => Some(self.regiters.d),
            Reg8::E => Some(self.regiters.e),
            Reg8::H => Some(self.regiters.h),
            Reg8::L => Some(self.regiters.l),
        }
    }

    fn write8(&mut self, _: &mut Peripherals, dst: Reg8, val: u8) -> Option<()> {
        match dst {
            Reg8::A => {
                self.regiters.a = val;
                Some(())
            }
            Reg8::B => {
                self.regiters.b = val;
                Some(())
            }
            Reg8::C => {
                self.regiters.c = val;
                Some(())
            }
            Reg8::D => {
                self.regiters.d = val;
                Some(())
            }
            Reg8::E => {
                self.regiters.e = val;
                Some(())
            }
            Reg8::H => {
                self.regiters.h = val;
                Some(())
            }
            Reg8::L => {
                self.regiters.l = val;
                Some(())
            }
        }
    }
}

impl IO16<Reg16> for Cpu {
    fn read16(&self, _: &Peripherals, src: Reg16) -> Option<u16> {
        match src {
            Reg16::AF => Some(self.regiters.af()),
            Reg16::BC => Some(self.regiters.bc()),
            Reg16::DE => Some(self.regiters.de()),
            Reg16::HL => Some(self.regiters.hl()),
            Reg16::SP => Some(self.regiters.sp),
        }
    }

    fn write16(&mut self, _: &mut Peripherals, dst: Reg16, val: u16) -> Option<()> {
        match dst {
            Reg16::AF => {
                self.regiters.write_af(val);
                Some(())
            }
            Reg16::BC => {
                self.regiters.write_bc(val);
                Some(())
            }
            Reg16::DE => {
                self.regiters.write_de(val);
                Some(())
            }
            Reg16::HL => {
                self.regiters.write_hl(val);
                Some(())
            }
            Reg16::SP => {
                self.regiters.sp = val;
                Some(())
            }
        }
    }
}

impl IO8<Imm8> for Cpu {
    fn read8(&mut self, bus: &Peripherals, _: Imm8) -> Option<u8> {
        static STEP: AtomicU8 = AtomicU8::new(0);
        static VAL8: AtomicU8 = AtomicU8::new(0);
        match STEP.load(Relaxed) {
            0 => {
                VAL8.store(bus.read(self.resiters.pc), Relaxed);
                self.regiters.pc = self.regiters.pc.wrapping_add(1);
                STEP.fetch_add(1, Relaxed);
                None
            }
            1 => {
                STEP.store(0, Relaxed);
                Some(VAL8.load(Relaxed))
            }
            _ => unreachable!(),
        }
    }

    fn write8(&mut self, _: &mut Peripherals, _: Imm8, _: u8) -> Option<()> {
        unreachable!()
    }
}

// 2 M-cycle
impl IO16<Imm16> for Cpu {
    fn read16(&self, bus: &Peripherals, _: Imm16) -> Option<u16> {
        static STEP: AtomicU8 = AtomicU8::new(0);
        static VAL8: AtomicU8 = AtomicU8::new(0);
        static VAL16: AtomicU16 = AtomicU16::new(0);
        match STEP.load(Relaxed) {
            0 => {
                if let Some(lo) = self.read8(bus, Imm8) {
                    VAL8.store(lo, Relaxed);
                    STEP.fetch_add(1, Relaxed);
                }
                None
            }
            1 => {
                if let Some(hi) = self.read8(bus, Imm8) {
                    VAL16.store(u16::from_le_bytes([VAL8.load(Relaxed), hi]), Relaxed);
                    STEP.fetch_add(1, Relaxed);
                }
                None
            }
            2 => {
                STEP.store(0, Relaxed);
                Some(VAL16.load(Relaxed))
            }
            _ => unreachable!(),
        }
    }

    fn write16(&mut self, _: &mut Peripherals, _: Imm16, _: u16) -> Option<()> {
        unreachable!()
    }
}

impl IO8<Indirect> for Cpu {
    fn read8(&mut self, bus: &Peripherals, src: Indirect) -> Option<u8> {
        static STEP: AtomicU8 = AtomicU8::new(0);
        static VAL8: AtomicU8 = AtomicU8::new(0);
        match STEP.load(Relaxed) {
            0 => {
                VAL8.store(
                    match src {
                        Indirect::BC => bus.read(self.regiters.bc()),
                        Indirect::DE => bus.read(self.regiters.de()),
                        Indirect::HL => bus.read(self.regiters.hl()),
                        Indirect::CFF => bus.read(0xFF00 | u16::from(self.regiters.c)),
                        Indirect::HLD => {
                            let addr = self.regiters.hl();
                            self.regiters.write_hl(addr.wrapping_sub(1));
                            bus.read(addr)
                        }
                        Indirect::HLI => {
                            let addr = self.regiters.hl();
                            self.regiters.write_hl(addr.wrapping_add(1));
                            bus.read(addr)
                        }
                    },
                    Relaxed,
                );
                STEP.fetch_add(1, Relaxed);
                None
            }
            1 => {
                STEP.store(0, Relaxed);
                Some(VAL8.load(Relaxed))
            }
            _ => unreachable!(),
        }
    }

    fn write8(&mut self, bus: &mut Peripherals, dst: Indirect, val: u8) -> Option<()> {
        static STEP: AtomicU8 = AtomicU8::new(0);
        static VAL8: AtomicU8 = AtomicU8::new(0);
        match STEP.load(Relaxed) {
            0 => {
                match dst {
                    Indirect::BC => bus.write(self.regiters.bc(), val),
                    Indirect::DE => bus.write(self.regiters.de(), val),
                    Indirect::HL => bus.write(self.regiters.hl(), val),
                    Indirect::CFF => bus.write(0xFF00 | u16::from(self.regiters.c), val),
                    Indirect::HLD => {
                        let addr = self.regiters.hl();
                        self.regiters.write_hl(addr.wrapping_sub(1));
                        bus.write(addr, val)
                    }
                    Indirect::HLI => {
                        let addr = self.regiters.hl();
                        self.regiters.write_hl(addr.wrapping_add(1));
                        bus.write(addr, val)
                    }
                }
                STEP.fetch_add(1, Relaxed);
                None
            }
            1 => {
                STEP.store(0, Relaxed);
                Some(())
            }
            _ => unreachable!(),
        }
    }
}

impl IO8<Direct8> for Cpu {
    fn read8(&mut self, bus: &Peripherals, src: Direct8) -> Option<u8> {
        static STEP: AtomicU8 = AtomicU8::new(0);
        static VAL8: AtomicU8 = AtomicU8::new(0);
        static VAL16: AtomicU16 = AtomicU16::new(0);
        match STEP.load(Relaxed) {
            0 => {
                if let Some(lo) = self.read8(bus, Imm8) {
                    VAL8.store(lo, Relaxed);
                    STEP.fetch_add(1, Relaxed);

                    // for DFF, write 0xFF00 + lo, and read in 2 M-cycle
                    if let Direct8::DFF = src {
                        VAL16.store(0xFF00 | u16::from(lo), Relaxed);
                        STEP.fetch_add(1, Relaxed);
                    }
                }
                None
            }
            1 => {
                if let Some(hi) = self.read8(bus, Imm8) {
                    VAL16.store(u16::from_le_bytes([VAL8.load(Relaxed), hi]), Relaxed);
                    STEP.fetch_add(1, Relaxed);
                }
                None
            }
            2 => {
                VAL8.store(bus.read(VAL16.load(Relaxed)), Relaxed);
                STEP.fetch_add(1, Relaxed);
                None
            }
            3 => {
                STEP.store(0, Relaxed);
                Some(VAL8.load(Relaxed))
            }
            _ => unreachable!(),
        }
    }

    fn write8(&mut self, bus: &mut Peripherals, dst: Direct8, val: u8) -> Option<()> {
        static STEP: AtomicU8 = AtomicU8::new(0);
        static VAL8: AtomicU8 = AtomicU8::new(0);
        static VAL16: AtomicU16 = AtomicU16::new(0);
        match STEP.load(Relaxed) {
            0 => {
                if let Some(lo) = self.read8(bus, Imm8) {
                    VAL8.store(lo, Relaxed);
                    STEP.fetch_add(1, Relaxed);

                    // for DFF, write 0xFF00 + lo, and write in 2 M-cycle
                    if let Direct8::DFF = dst {
                        VAL16.store(0xFF00 | u16::from(lo), Relaxed);
                        STEP.fetch_add(1, Relaxed);
                    }
                }
                None
            }
            1 => {
                if let Some(hi) = self.read8(bus, Imm8) {
                    VAL16.store(u16::from_le_bytes([VAL8.load(Relaxed), hi]), Relaxed);
                    STEP.fetch_add(1, Relaxed);
                }
                None
            }
            2 => {
                bus.write(VAL16.load(Relaxed), val);
                STEP.fetch_add(1, Relaxed);
                None
            }
            3 => {
                STEP.store(0, Relaxed);
                Some(())
            }
            _ => unreachable!(),
        }
    }
}

impl IO16<Direct16> for Cpu {
    fn read16(&mut self, _: &Peripherals, _: Direct16) -> Option<u16> {
        unreachable!()
    }

    fn write16(&mut self, bus: &mut Peripherals, _: Direct16, val: u16) -> Option<()> {
        static STEP: AtomicU8 = AtomicU8::new(0);
        static VAL8: AtomicU8 = AtomicU8::new(0);
        static VAL16: AtomicU16 = AtomicU16::new(0);
        match STEP.load(Relaxed) {
            0 => {
                if let Some(lo) = self.read8(bus, Imm8) {
                    VAL8.store(lo, Relaxed);
                    STEP.fetch_add(1, Relaxed);
                }
                None
            }
            1 => {
                if let Some(hi) = self.read8(bus, Imm8) {
                    VAL16.store(u16::from_le_bytes([VAL8.load(Relaxed), hi]), Relaxed);
                    STEP.fetch_add(1, Relaxed);
                }
                None
            }
            2 => {
                bus.write(VAL16.load(Relaxed), val as u8);
                STEP.fetch_add(1, Relaxed);
                None
            }
            3 => {
                bus.write(VAL16.load(Relaxed).wrapping_add(1), (val >> 8) as u8);
                STEP.fetch_add(1, Relaxed);
                None
            }
            4 => {
                STEP.store(0, Relaxed);
                Some(())
            }
            _ => unreachable!(),
        }
    }
}
