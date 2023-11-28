use crate::cpu::operand::Imm16;
use crate::cpu::{operand::Imm8, Cpu};
use crate::peripherals::Peripherals;
use std::sync::atomic::{AtomicU16, AtomicU8, Ordering::Relaxed};

use super::operand::{Cond, Reg16, IO16, IO8};

impl Cpu {
    pub fn nop(&mut self, bus: &Peripherals) {
        // fetch / execute overlap
        self.fetch(bus);
    }

    // load dst <- src
    pub fn ld<D: Copy, S: Copy>(&mut self, bus: &mut Peripherals, dst: D, src: S)
    where
        Self: IO8<D> + IO8<S>,
    {
        static STEP: AtomicU8 = AtomicU8::new(0);
        static VAL8: AtomicU8 = AtomicU8::new(0);
        match STEP.load(Relaxed) {
            0 => {
                if let Some(val) = self.read8(bus, src) {
                    VAL8.store(val, Relaxed);
                    STEP.fetch_add(1, Relaxed);
                }
            }
            1 => {
                if self.write8(bus, dst, VAL8.load(Relaxed)).is_some() {
                    STEP.fetch_add(1, Relaxed);
                };
            }
            2 => {
                STEP.store(0, Relaxed);
                self.fetch(bus);
            }
            _ => unreachable!(),
        }
    }

    pub fn ld16<D: Copy, S: Copy>(&mut self, bus: &mut Peripherals, dst: D, src: S)
    where
        Self: IO16<D> + IO16<S>,
    {
        static STEP: AtomicU8 = AtomicU8::new(0);
        static VAL16: AtomicU16 = AtomicU16::new(0);
        match STEP.load(Relaxed) {
            0 => {
                if let Some(val) = self.read16(bus, src) {
                    VAL16.store(val as u16, Relaxed);
                    STEP.fetch_add(1, Relaxed);
                }
            }
            1 => {
                if self.write16(bus, dst, VAL16.load(Relaxed)).is_some() {
                    STEP.fetch_add(1, Relaxed);
                };
            }
            2 => {
                STEP.store(0, Relaxed);
                self.fetch(bus);
            }
            _ => unreachable!(),
        }
    }

    // compare A register with src
    pub fn cp<S: Copy>(&mut self, bus: &Peripherals, src: S)
    where
        Self: IO8<S>,
    {
        if let Some(val) = self.read8(bus, src) {
            let (result, carry) = self.registers.a.overflowing_sub(val);
            self.registers.set_zf(result == 0);
            self.registers.set_nf(true);
            self.registers
                .set_hf((self.registers.a & 0xf) < (val & 0xf));
            self.registers.set_cf(carry);
            self.fetch(bus)
        }
    }

    // increment src
    pub fn inc<S: Copy>(&mut self, bus: &mut Peripherals, src: S)
    where
        Self: IO8<S>,
    {
        static STEP: AtomicU8 = AtomicU8::new(0);
        static VAL8: AtomicU8 = AtomicU8::new(0);
        match STEP.load(Relaxed) {
            0 => {
                if let Some(val) = self.read8(bus, src) {
                    let result = val.wrapping_add(1);
                    self.registers.set_zf(result == 0);
                    self.registers.set_nf(false);
                    self.registers.set_hf((val & 0xf) == 0xf);
                    VAL8.store(result, Relaxed);
                    STEP.fetch_add(1, Relaxed);
                }
            }
            1 => {
                if self.write8(bus, src, VAL8.load(Relaxed)).is_some() {
                    STEP.store(0, Relaxed);
                    self.fetch(bus);
                }
            }
            _ => unreachable!(),
        }
    }

    pub fn inc16<S: Copy>(&mut self, bus: &mut Peripherals, src: S)
    where
        Self: IO16<S>,
    {
        static STEP: AtomicU8 = AtomicU8::new(0);
        static VAL16: AtomicU16 = AtomicU16::new(0);
        match STEP.load(Relaxed) {
            0 => {
                if let Some(val) = self.read16(bus, src) {
                    let result = val.wrapping_add(1);
                    VAL16.store(result, Relaxed);
                    STEP.fetch_add(1, Relaxed);
                }
            }
            1 => {
                if self.write16(bus, src, VAL16.load(Relaxed)).is_some() {
                    // when Reg16, cycle +1
                    STEP.fetch_add(1, Relaxed);
                }
            }
            2 => {
                STEP.store(0, Relaxed);
                self.fetch(bus);
            }
            _ => unreachable!(),
        }
    }

    // decrement src
    pub fn dec<S: Copy>(&mut self, bus: &mut Peripherals, src: S)
    where
        Self: IO8<S>,
    {
        static STEP: AtomicU8 = AtomicU8::new(0);
        static VAL8: AtomicU8 = AtomicU8::new(0);
        match STEP.load(Relaxed) {
            0 => {
                if let Some(val) = self.read8(bus, src) {
                    let result = val.wrapping_sub(1);
                    self.registers.set_zf(result == 0);
                    self.registers.set_nf(true);
                    self.registers.set_hf((val & 0xf) == 0);
                    VAL8.store(result, Relaxed);
                    STEP.fetch_add(1, Relaxed);
                }
            }
            1 => {
                if self.write8(bus, src, VAL8.load(Relaxed)).is_some() {
                    STEP.store(0, Relaxed);
                    self.fetch(bus);
                }
            }
            _ => unreachable!(),
        }
    }

    pub fn dec16<S: Copy>(&mut self, bus: &mut Peripherals, src: S)
    where
        Self: IO16<S>,
    {
        static STEP: AtomicU8 = AtomicU8::new(0);
        static VAL16: AtomicU16 = AtomicU16::new(0);
        match STEP.load(Relaxed) {
            0 => {
                if let Some(val) = self.read16(bus, src) {
                    let result = val.wrapping_sub(1);
                    VAL16.store(result, Relaxed);
                    STEP.fetch_add(1, Relaxed);
                }
            }
            1 => {
                if self.write16(bus, src, VAL16.load(Relaxed)).is_some() {
                    // when Reg16, cycle +1
                    STEP.fetch_add(1, Relaxed);
                }
            }
            2 => {
                STEP.store(0, Relaxed);
                self.fetch(bus);
            }
            _ => unreachable!(),
        }
    }

    // rotate left
    pub fn rl<S: Copy>(&mut self, bus: &mut Peripherals, src: S)
    where
        Self: IO8<S>,
    {
        static STEP: AtomicU8 = AtomicU8::new(0);
        static VAL8: AtomicU8 = AtomicU8::new(0);
        match STEP.load(Relaxed) {
            0 => {
                if let Some(val) = self.read8(bus, src) {
                    let carry = self.registers.cf() as u8;
                    let result = (val << 1) | carry;
                    self.registers.set_zf(result == 0);
                    self.registers.set_nf(false);
                    self.registers.set_hf(false);
                    self.registers.set_cf(val & 0x80 != 0);
                    VAL8.store(result, Relaxed);
                    STEP.fetch_add(1, Relaxed);
                }
            }
            1 => {
                if self.write8(bus, src, VAL8.load(Relaxed)).is_some() {
                    STEP.store(0, Relaxed);
                    self.fetch(bus);
                }
            }
            _ => unreachable!(),
        }
    }

    // check bit n of src
    pub fn bit<S: Copy>(&mut self, bus: &Peripherals, n: u8, src: S)
    where
        Self: IO8<S>,
    {
        if let Some(val) = self.read8(bus, src) {
            self.registers.set_zf(val & (1 << n) == 0);
            self.registers.set_nf(false);
            self.registers.set_hf(true);
            self.fetch(bus);
        }
    }

    // push val onto stack
    pub fn push(&mut self, bus: &mut Peripherals, src: Reg16) {
        static STEP: AtomicU8 = AtomicU8::new(0);
        static VAL16: AtomicU16 = AtomicU16::new(0);
        match STEP.load(Relaxed) {
            0 => {
                VAL16.store(self.read16(bus, src).unwrap(), Relaxed);
                STEP.fetch_add(1, Relaxed);
            }
            1 => {
                if self.push16(bus, VAL16.load(Relaxed)).is_some() {
                    STEP.fetch_add(1, Relaxed);
                }
            }
            2 => {
                STEP.store(0, Relaxed);
                self.fetch(bus);
            }
            _ => unreachable!(),
        }
    }

    pub fn push16(&mut self, bus: &mut Peripherals, val: u16) -> Option<()> {
        static STEP: AtomicU8 = AtomicU8::new(0);
        static VAL8: AtomicU8 = AtomicU8::new(0);
        match STEP.load(Relaxed) {
            0 => {
                // cycle count is memory accsess + 1
                STEP.fetch_add(1, Relaxed);
                None
            }
            1 => {
                let [lo, hi] = u16::to_le_bytes(val);
                self.registers.sp = self.registers.sp.wrapping_sub(1);
                bus.write(&mut self.interrupts, self.registers.sp, hi);
                VAL8.store(lo, Relaxed);
                STEP.fetch_add(1, Relaxed);
                None
            }
            2 => {
                self.registers.sp = self.registers.sp.wrapping_sub(1);
                bus.write(&mut self.interrupts, self.registers.sp, VAL8.load(Relaxed));
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

    // pop from stack
    pub fn pop(&mut self, bus: &mut Peripherals, dst: Reg16) {
        if let Some(val) = self.pop16(bus) {
            self.write16(bus, dst, val);
            self.fetch(bus)
        }
    }

    pub fn pop16(&mut self, bus: &mut Peripherals) -> Option<u16> {
        static STEP: AtomicU8 = AtomicU8::new(0);
        static VAL8: AtomicU8 = AtomicU8::new(0);
        static VAL16: AtomicU16 = AtomicU16::new(0);
        match STEP.load(Relaxed) {
            0 => {
                VAL8.store(bus.read(&self.interrupts, self.registers.sp), Relaxed);
                self.registers.sp = self.registers.sp.wrapping_add(1);
                STEP.fetch_add(1, Relaxed);
                None
            }
            1 => {
                let hi = bus.read(&self.interrupts, self.registers.sp);
                self.registers.sp = self.registers.sp.wrapping_add(1);
                VAL16.store(u16::from_le_bytes([VAL8.load(Relaxed), hi]), Relaxed);
                STEP.fetch_add(1, Relaxed);
                None
            }
            2 => {
                STEP.store(0, Relaxed);
                Some(VAL16.load(Relaxed))
            }
            _ => unreachable!(),
        }
    }

    // jump relative
    pub fn jr(&mut self, bus: &Peripherals) {
        static STEP: AtomicU8 = AtomicU8::new(0);
        match STEP.load(Relaxed) {
            0 => {
                if let Some(val) = self.read8(bus, Imm8) {
                    self.registers.pc = self.registers.pc.wrapping_add(val as i8 as u16);
                    STEP.fetch_add(1, Relaxed);
                }
            }
            1 => {
                STEP.store(0, Relaxed);
                self.fetch(bus);
            }
            _ => unreachable!(),
        }
    }

    // jump relative if condition
    fn cond(&self, cond: Cond) -> bool {
        match cond {
            Cond::NZ => !self.registers.zf(),
            Cond::Z => self.registers.zf(),
            Cond::NC => !self.registers.cf(),
            Cond::C => self.registers.cf(),
        }
    }
    pub fn jr_c(&mut self, bus: &Peripherals, c: Cond) {
        static STEP: AtomicU8 = AtomicU8::new(0);
        match STEP.load(Relaxed) {
            0 => {
                if let Some(val) = self.read8(bus, Imm8) {
                    STEP.fetch_add(1, Relaxed);
                    if self.cond(c) {
                        self.registers.pc = self.registers.pc.wrapping_add(val as i8 as u16);
                    }
                }
            }
            1 => {
                STEP.store(0, Relaxed);
                self.fetch(bus);
            }
            _ => unreachable!(),
        }
    }

    // jump
    pub fn jp(&mut self, bus: &mut Peripherals) {
        static STEP: AtomicU8 = AtomicU8::new(0);
        match STEP.load(Relaxed) {
            0 => {
                if let Some(val) = self.read16(bus, Imm16) {
                    self.registers.pc = val;
                    STEP.fetch_add(1, Relaxed);
                }
            }
            1 => {
                STEP.store(0, Relaxed);
                self.fetch(bus);
            }
            _ => unreachable!(),
        }
    }

    // call subroutine
    pub fn call(&mut self, bus: &mut Peripherals) {
        static STEP: AtomicU8 = AtomicU8::new(0);
        static VAL16: AtomicU16 = AtomicU16::new(0);
        match STEP.load(Relaxed) {
            0 => {
                if let Some(val) = self.read16(bus, Imm16) {
                    VAL16.store(val, Relaxed);
                    STEP.fetch_add(1, Relaxed);
                }
            }
            1 => {
                if self.push16(bus, self.registers.pc).is_some() {
                    self.registers.pc = VAL16.load(Relaxed);
                    STEP.store(0, Relaxed);
                    self.fetch(bus);
                }
            }
            _ => unreachable!(),
        }
    }

    // return from subroutine
    pub fn ret(&mut self, bus: &mut Peripherals) {
        static STEP: AtomicU8 = AtomicU8::new(0);
        match STEP.load(Relaxed) {
            0 => {
                if let Some(val) = self.pop16(bus) {
                    self.registers.pc = val;
                    STEP.fetch_add(1, Relaxed);
                }
            }
            1 => {
                STEP.store(0, Relaxed);
                self.fetch(bus);
            }
            _ => unreachable!(),
        }
    }

    // return from interrupts
    pub fn reti(&mut self, bus: &mut Peripherals) {
        static STEP: AtomicU8 = AtomicU8::new(0);
        match STEP.load(Relaxed) {
            0 => {
                if let Some(val) = self.pop16(bus) {
                    self.registers.pc = val;
                    STEP.fetch_add(1, Relaxed);
                }
            }
            1 => {
                self.interrupts.ime = true;
                STEP.store(0, Relaxed);
                self.fetch(bus);
            }
            _ => unreachable!(),
        }
    }

    // enable interrupts
    pub fn ei(&mut self, bus: &Peripherals) {
        self.fetch(bus);
        self.interrupts.ime = true;
    }

    // disable interrupts
    pub fn di(&mut self, bus: &Peripherals) {
        self.fetch(bus);
        self.interrupts.ime = false;
    }
}
