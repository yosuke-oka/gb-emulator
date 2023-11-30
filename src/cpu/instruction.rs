use crate::cpu::operand::Imm16;
use crate::cpu::{operand::Imm8, Cpu};
use crate::peripherals::Peripherals;
use std::sync::atomic::{AtomicU16, AtomicU8, Ordering::Relaxed};

use super::operand::{Cond, Reg16, IO16, IO8};

impl Cpu {
    pub fn nop(&mut self, bus: &Peripherals) {
        // nop
    }

    // load dst <- src
    pub fn ld<D: Copy, S: Copy>(&mut self, bus: &mut Peripherals, dst: D, src: S)
    where
        Self: IO8<D> + IO8<S>,
    {
        let val = self.read8(bus, src);
        self.write8(bus, dst, val);
    }

    pub fn ld16<D: Copy, S: Copy>(&mut self, bus: &mut Peripherals, dst: D, src: S)
    where
        Self: IO16<D> + IO16<S>,
    {
        let val = self.read16(bus, src);
        self.write16(bus, dst, val);
    }

    // compare A register with src
    pub fn cp<S: Copy>(&mut self, bus: &Peripherals, src: S)
    where
        Self: IO8<S>,
    {
        let val = self.read8(bus, src);
        let (result, carry) = self.registers.a.overflowing_sub(val);
        self.registers.set_zf(result == 0);
        self.registers.set_nf(true);
        self.registers
            .set_hf((self.registers.a & 0xf) < (val & 0xf));
        self.registers.set_cf(carry);
    }

    // increment src
    pub fn inc<S: Copy>(&mut self, bus: &mut Peripherals, src: S)
    where
        Self: IO8<S>,
    {
        let val = self.read8(bus, src);
        let result = val.wrapping_add(1);
        self.registers.set_zf(result == 0);
        self.registers.set_nf(false);
        self.registers.set_hf((val & 0xf) == 0xf);
        self.write8(bus, src, result);
    }

    pub fn inc16<S: Copy>(&mut self, bus: &mut Peripherals, src: S)
    where
        Self: IO16<S>,
    {
        let val = self.read16(bus, src);
        let result = val.wrapping_add(1);
        self.write16(bus, src, result);
    }

    // decrement src
    pub fn dec<S: Copy>(&mut self, bus: &mut Peripherals, src: S)
    where
        Self: IO8<S>,
    {
        let val = self.read8(bus, src);
        let result = val.wrapping_sub(1);
        self.registers.set_zf(result == 0);
        self.registers.set_nf(true);
        self.registers.set_hf((val & 0xf) == 0);
        self.write8(bus, src, result);
    }

    pub fn dec16<S: Copy>(&mut self, bus: &mut Peripherals, src: S)
    where
        Self: IO16<S>,
    {
        let val = self.read16(bus, src);
        let result = val.wrapping_sub(1);
        self.write16(bus, src, result);
    }

    // rotate left
    pub fn rl<S: Copy>(&mut self, bus: &mut Peripherals, src: S)
    where
        Self: IO8<S>,
    {
        let val = self.read8(bus, src);
        let carry = self.registers.cf() as u8;
        let result = (val << 1) | carry;
        self.registers.set_zf(result == 0);
        self.registers.set_nf(false);
        self.registers.set_hf(false);
        self.registers.set_cf(val & 0x80 != 0);
        self.write8(bus, src, result);
    }

    // check bit n of src
    pub fn bit<S: Copy>(&mut self, bus: &Peripherals, n: u8, src: S)
    where
        Self: IO8<S>,
    {
        let val = self.read8(bus, src);
        self.registers.set_zf(val & (1 << n) == 0);
        self.registers.set_nf(false);
        self.registers.set_hf(true);
    }

    // push val onto stack
    pub fn push(&mut self, bus: &mut Peripherals, src: Reg16) {
        let val = self.read16(bus, src);
        self.push16(bus, val);
        self.tick(); // cycle +1
    }

    pub fn push16(&mut self, bus: &mut Peripherals, val: u16) {
        let [lo, hi] = u16::to_le_bytes(val);
        self.registers.sp = self.registers.sp.wrapping_sub(1);
        self.write_bus(bus, self.registers.sp, hi);
        self.registers.sp = self.registers.sp.wrapping_sub(1);
        self.write_bus(bus, self.registers.sp, lo);
    }

    // pop from stack
    pub fn pop(&mut self, bus: &mut Peripherals, dst: Reg16) {
        let val = self.pop16(bus);
        self.write16(bus, dst, val);
    }

    pub fn pop16(&mut self, bus: &mut Peripherals) -> u16 {
        let lo = self.read_bus(bus, self.registers.sp);
        self.registers.sp = self.registers.sp.wrapping_add(1);
        let hi = self.read_bus(bus, self.registers.sp);
        self.registers.sp = self.registers.sp.wrapping_add(1);
        u16::from_le_bytes([lo, hi])
    }

    // jump relative
    pub fn jr(&mut self, bus: &Peripherals) {
        let val = self.read8(bus, Imm8);
        self.registers.pc = self.registers.pc.wrapping_add(val as i8 as u16);
        self.tick(); // cycle +1
    }

    fn cond(&self, cond: Cond) -> bool {
        match cond {
            Cond::NZ => !self.registers.zf(),
            Cond::Z => self.registers.zf(),
            Cond::NC => !self.registers.cf(),
            Cond::C => self.registers.cf(),
        }
    }

    // jump relative if condition
    pub fn jr_c(&mut self, bus: &Peripherals, c: Cond) {
        let val = self.read8(bus, Imm8);

        if self.cond(c) {
            self.registers.pc = self.registers.pc.wrapping_add(val as i8 as u16);
            self.tick(); // cycle +1
        }
    }

    // jump
    pub fn jp(&mut self, bus: &mut Peripherals) {
        let val = self.read16(bus, Imm16);
        self.registers.pc = val;
        self.tick(); // cycle +1
    }

    // call subroutine
    pub fn call(&mut self, bus: &mut Peripherals) {
        let val = self.read16(bus, Imm16);
        self.push16(bus, self.registers.pc);
        self.registers.pc = val;
    }

    // return from subroutine
    pub fn ret(&mut self, bus: &mut Peripherals) {
        let val = self.pop16(bus);
        self.registers.pc = val;
        self.tick(); // cycle +1
    }

    // return from interrupts
    pub fn reti(&mut self, bus: &mut Peripherals) {
        self.ret(bus);
        self.interrupts.ime = true;
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
