use crate::bus::Bus;
use crate::cpu::operand::Imm16;
use crate::cpu::{operand::Imm8, Cpu};

use super::operand::{Cond, Reg16, IO16, IO8};

impl Cpu {
    pub fn nop(&mut self, bus: &mut Bus) {
        self.tick(bus)
    }

    // stop
    pub fn stop(&mut self, _: &mut Bus) {
        // omit implementation
    }

    pub fn halt(&mut self, _: &mut Bus) {
        if self.interrupts.get_interrupt() == 0 {
            self.halting = true;
        } else {
            self.halting = false;
        }
    }

    // load dst <- src
    pub fn ld<D: Copy, S: Copy>(&mut self, bus: &mut Bus, dst: D, src: S)
    where
        Self: IO8<D> + IO8<S>,
    {
        let val = self.read8(bus, src);
        self.write8(bus, dst, val);
    }

    pub fn ld16<D: Copy, S: Copy>(&mut self, bus: &mut Bus, dst: D, src: S)
    where
        Self: IO16<D> + IO16<S>,
    {
        let val = self.read16(bus, src);
        self.write16(bus, dst, val);
    }

    // load sp <- hl
    pub fn ld_sp_hl(&mut self, bus: &mut Bus) {
        self.registers.sp = self.registers.hl();
        self.tick(bus);
    }

    // load hl <- sp + e
    pub fn ld_hl_sp_e(&mut self, bus: &mut Bus) {
        let e = self.read8(bus, Imm8) as i8 as u16;
        let sp = self.registers.sp;
        let result = sp.wrapping_add(e);
        self.registers.set_zf(false);
        self.registers.set_nf(false);
        self.registers.set_hf((sp & 0xf) + (e & 0xf) > 0xf);
        self.registers.set_cf((sp & 0xff) + (e & 0xff) > 0xff);
        self.registers.write_hl(result);
        self.tick(bus); // cycle +1
    }

    // compare A register with src
    pub fn cp<S: Copy>(&mut self, bus: &mut Bus, src: S)
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

    // add src to A register
    pub fn add<S: Copy>(&mut self, bus: &mut Bus, src: S)
    where
        Self: IO8<S>,
    {
        let val = self.read8(bus, src);
        let (result, carry) = self.registers.a.overflowing_add(val);
        self.registers.set_zf(result == 0);
        self.registers.set_nf(false);
        self.registers
            .set_hf((self.registers.a & 0xf) + (val & 0xf) > 0xf);
        self.registers.set_cf(carry);
        self.registers.a = result;
    }

    // add hl + Reg16
    pub fn add_hl_reg16(&mut self, bus: &mut Bus, src: Reg16) {
        let val = self.read16(bus, src);
        let hl = self.registers.hl();
        let (result, carry) = hl.overflowing_add(val);
        self.registers.set_nf(false);
        self.registers.set_hf((hl & 0xfff) + (val & 0xfff) > 0xfff);
        self.registers.set_cf(carry);
        self.registers.write_hl(result);
        self.tick(bus); // cycle +1
    }

    // add sp + e
    pub fn add_sp_e(&mut self, bus: &mut Bus) {
        let e = self.read8(bus, Imm8) as i8 as u16;
        let sp = self.registers.sp;
        let result = sp.wrapping_add(e);
        self.registers.set_zf(false);
        self.registers.set_nf(false);
        self.registers.set_hf((sp & 0xf) + (e & 0xf) > 0xf);
        self.registers.set_cf((sp & 0xff) + (e & 0xff) > 0xff);
        self.registers.sp = result;
        self.tick(bus); // cycle +2
        self.tick(bus);
    }

    // add src + carry to A register
    pub fn adc<S: Copy>(&mut self, bus: &mut Bus, src: S)
    where
        Self: IO8<S>,
    {
        let val = self.read8(bus, src);
        let c = self.registers.cf() as u8;
        let result = self.registers.a.wrapping_add(val).wrapping_add(c);
        self.registers.set_zf(result == 0);
        self.registers.set_nf(false);
        self.registers
            .set_hf((self.registers.a & 0xf) + (val & 0xf) + c > 0xf);
        self.registers
            .set_cf(self.registers.a as u16 + val as u16 + c as u16 > 0xff);
        self.registers.a = result;
    }

    // subtract src from A register
    pub fn sub<S: Copy>(&mut self, bus: &mut Bus, src: S)
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
        self.registers.a = result;
    }

    // subtract src + carry from A register
    pub fn sbc<S: Copy>(&mut self, bus: &mut Bus, src: S)
    where
        Self: IO8<S>,
    {
        let val = self.read8(bus, src);
        let c = self.registers.cf() as u8;
        let result = self.registers.a.wrapping_sub(val).wrapping_sub(c);
        self.registers.set_zf(result == 0);
        self.registers.set_nf(true);
        self.registers
            .set_hf((self.registers.a & 0xf) < (val & 0xf) + c);
        self.registers
            .set_cf((self.registers.a as u16) < (val as u16) + (c as u16));
        self.registers.a = result;
    }

    // logical and src with A register
    pub fn and<S: Copy>(&mut self, bus: &mut Bus, src: S)
    where
        Self: IO8<S>,
    {
        let val = self.read8(bus, src);
        let result = self.registers.a & val;
        self.registers.set_zf(result == 0);
        self.registers.set_nf(false);
        self.registers.set_hf(true);
        self.registers.set_cf(false);
        self.registers.a = result;
    }

    // logical or src with A register
    pub fn or<S: Copy>(&mut self, bus: &mut Bus, src: S)
    where
        Self: IO8<S>,
    {
        let val = self.read8(bus, src);
        let result = self.registers.a | val;
        self.registers.set_zf(result == 0);
        self.registers.set_nf(false);
        self.registers.set_hf(false);
        self.registers.set_cf(false);
        self.registers.a = result;
    }

    // logical xor src with A register
    pub fn xor<S: Copy>(&mut self, bus: &mut Bus, src: S)
    where
        Self: IO8<S>,
    {
        let val = self.read8(bus, src);
        let result = self.registers.a ^ val;
        self.registers.set_zf(result == 0);
        self.registers.set_nf(false);
        self.registers.set_hf(false);
        self.registers.set_cf(false);
        self.registers.a = result;
    }

    // increment src
    pub fn inc<S: Copy>(&mut self, bus: &mut Bus, src: S)
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

    pub fn inc16<S: Copy>(&mut self, bus: &mut Bus, src: S)
    where
        Self: IO16<S>,
    {
        let val = self.read16(bus, src);
        let result = val.wrapping_add(1);
        self.tick(bus);
        self.write16(bus, src, result);
    }

    // decrement src
    pub fn dec<S: Copy>(&mut self, bus: &mut Bus, src: S)
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

    pub fn dec16<S: Copy>(&mut self, bus: &mut Bus, src: S)
    where
        Self: IO16<S>,
    {
        let val = self.read16(bus, src);
        let result = val.wrapping_sub(1);
        self.tick(bus);
        self.write16(bus, src, result);
    }

    // decimal adjust A register
    // https://ehaskins.com/2018-01-30%20Z80%20DAA/
    pub fn daa(&mut self, _: &mut Bus) {
        let mut correction = 0;
        let mut cf = false;
        if self.registers.cf() || (!self.registers.nf() && self.registers.a > 0x99) {
            correction |= 0x60;
            cf = true;
        }

        if self.registers.hf() || (!self.registers.nf() && (self.registers.a & 0x0f) > 0x09) {
            correction |= 0x06;
        }

        if self.registers.nf() {
            self.registers.a = self.registers.a.wrapping_sub(correction);
        } else {
            self.registers.a = self.registers.a.wrapping_add(correction);
        }
        self.registers.set_zf(self.registers.a == 0);
        self.registers.set_hf(false);
        self.registers.set_cf(cf);
    }

    // A = A xor FF
    pub fn cpl(&mut self, _: &mut Bus) {
        self.registers.a = !self.registers.a;
        self.registers.set_nf(true);
        self.registers.set_hf(true);
    }

    // rotate A register left
    pub fn rlca(&mut self, _: &mut Bus) {
        let val = self.registers.a;
        let highest_bit = val & 0x80 != 0;
        let result = (val << 1) | highest_bit as u8;
        self.registers.set_zf(false);
        self.registers.set_nf(false);
        self.registers.set_hf(false);
        self.registers.set_cf(highest_bit);
        self.registers.a = result;
    }

    // rotate A register left through carry
    pub fn rla(&mut self, _: &mut Bus) {
        let val = self.registers.a;
        let highest_bit = val & 0x80 != 0;
        let carry = self.registers.cf() as u8;
        let result = (val << 1) | carry;
        self.registers.set_zf(false);
        self.registers.set_nf(false);
        self.registers.set_hf(false);
        self.registers.set_cf(highest_bit);
        self.registers.a = result;
    }

    // rotate right A register
    pub fn rrca(&mut self, _: &mut Bus) {
        let val = self.registers.a;
        let lowest_bit = val & 0x01 != 0;
        let result = (val >> 1) | (lowest_bit as u8) << 7;
        self.registers.set_zf(false);
        self.registers.set_nf(false);
        self.registers.set_hf(false);
        self.registers.set_cf(lowest_bit);
        self.registers.a = result;
    }

    // rotate right A register through carry
    pub fn rra(&mut self, _: &mut Bus) {
        let val = self.registers.a;
        let carry = self.registers.cf() as u8;
        let lowest_bit = val & 0x01 != 0;
        let result = (val >> 1) | (carry << 7);
        self.registers.set_zf(false);
        self.registers.set_nf(false);
        self.registers.set_hf(false);
        self.registers.set_cf(lowest_bit);
        self.registers.a = result;
    }

    // rotate left through carry
    pub fn rl<S: Copy>(&mut self, bus: &mut Bus, src: S)
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

    // rotate left
    pub fn rlc<S: Copy>(&mut self, bus: &mut Bus, src: S)
    where
        Self: IO8<S>,
    {
        let val = self.read8(bus, src);
        let highest_bit = val & 0x80 != 0;
        let result = (val << 1) | highest_bit as u8;
        self.registers.set_zf(result == 0);
        self.registers.set_nf(false);
        self.registers.set_hf(false);
        self.registers.set_cf(highest_bit);
        self.write8(bus, src, result);
    }

    // rotate right through carry
    pub fn rr<S: Copy>(&mut self, bus: &mut Bus, src: S)
    where
        Self: IO8<S>,
    {
        let val = self.read8(bus, src);
        let carry = self.registers.cf() as u8;
        let lowest_bit = val & 0x01 != 0;
        let result = (val >> 1) | (carry << 7);
        self.registers.set_zf(result == 0);
        self.registers.set_nf(false);
        self.registers.set_hf(false);
        self.registers.set_cf(lowest_bit);
        self.write8(bus, src, result);
    }

    // rotate right
    pub fn rrc<S: Copy>(&mut self, bus: &mut Bus, src: S)
    where
        Self: IO8<S>,
    {
        let val = self.read8(bus, src);
        let lowest_bit = val & 0x01 != 0;
        let result = (val >> 1) | (lowest_bit as u8) << 7;
        self.registers.set_zf(result == 0);
        self.registers.set_nf(false);
        self.registers.set_hf(false);
        self.registers.set_cf(lowest_bit);
        self.write8(bus, src, result);
    }

    // shift left arithmetic
    pub fn sla<S: Copy>(&mut self, bus: &mut Bus, src: S)
    where
        Self: IO8<S>,
    {
        let val = self.read8(bus, src);
        let result = val << 1;
        self.registers.set_zf(result == 0);
        self.registers.set_nf(false);
        self.registers.set_hf(false);
        self.registers.set_cf(val & 0x80 != 0);
        self.write8(bus, src, result);
    }

    // shift right arithmetic (b7 = b7)
    pub fn sra<S: Copy>(&mut self, bus: &mut Bus, src: S)
    where
        Self: IO8<S>,
    {
        let val = self.read8(bus, src);
        let result = (val >> 1) | (val & 0x80);
        self.registers.set_zf(result == 0);
        self.registers.set_nf(false);
        self.registers.set_hf(false);
        self.registers.set_cf(val & 0x01 != 0);
        self.write8(bus, src, result);
    }

    // shift right logical (b7 = 0)
    pub fn srl<S: Copy>(&mut self, bus: &mut Bus, src: S)
    where
        Self: IO8<S>,
    {
        let val = self.read8(bus, src);
        let result = val >> 1;
        self.registers.set_zf(result == 0);
        self.registers.set_nf(false);
        self.registers.set_hf(false);
        self.registers.set_cf(val & 0x01 != 0);
        self.write8(bus, src, result);
    }

    // swap nibbles
    pub fn swap<S: Copy>(&mut self, bus: &mut Bus, src: S)
    where
        Self: IO8<S>,
    {
        let val = self.read8(bus, src);
        let result = (val << 4) | (val >> 4);
        self.registers.set_zf(result == 0);
        self.registers.set_nf(false);
        self.registers.set_hf(false);
        self.registers.set_cf(false);
        self.write8(bus, src, result);
    }

    // check bit n of src
    pub fn bit<S: Copy>(&mut self, bus: &mut Bus, n: u8, src: S)
    where
        Self: IO8<S>,
    {
        let val = self.read8(bus, src);
        self.registers.set_zf(val & (1 << n) == 0);
        self.registers.set_nf(false);
        self.registers.set_hf(true);
    }

    // set bit n
    pub fn set<S: Copy>(&mut self, bus: &mut Bus, n: u8, src: S)
    where
        Self: IO8<S>,
    {
        let val = self.read8(bus, src);
        self.write8(bus, src, val | (1 << n));
    }

    // reset bit n
    pub fn res<S: Copy>(&mut self, bus: &mut Bus, n: u8, src: S)
    where
        Self: IO8<S>,
    {
        let val = self.read8(bus, src);
        self.write8(bus, src, val & !(1 << n));
    }

    // push val onto stack
    pub fn push(&mut self, bus: &mut Bus, src: Reg16) {
        let val = self.read16(bus, src);
        self.push16(bus, val);
        self.tick(bus); // cycle +1
    }

    pub fn push16(&mut self, bus: &mut Bus, val: u16) {
        let [lo, hi] = u16::to_le_bytes(val);
        self.registers.sp = self.registers.sp.wrapping_sub(1);
        self.write_bus(bus, self.registers.sp, hi);
        self.registers.sp = self.registers.sp.wrapping_sub(1);
        self.write_bus(bus, self.registers.sp, lo);
    }

    // pop from stack
    pub fn pop(&mut self, bus: &mut Bus, dst: Reg16) {
        let val = self.pop16(bus);
        self.write16(bus, dst, val);
    }

    pub fn pop16(&mut self, bus: &mut Bus) -> u16 {
        let lo = self.read_bus(bus, self.registers.sp);
        self.registers.sp = self.registers.sp.wrapping_add(1);
        let hi = self.read_bus(bus, self.registers.sp);
        self.registers.sp = self.registers.sp.wrapping_add(1);
        u16::from_le_bytes([lo, hi])
    }

    // jump relative
    pub fn jr(&mut self, bus: &mut Bus) {
        let val = self.read8(bus, Imm8);
        self.registers.pc = self.registers.pc.wrapping_add(val as i8 as u16);
        self.tick(bus); // cycle +1
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
    pub fn jr_c(&mut self, bus: &mut Bus, c: Cond) {
        let val = self.read8(bus, Imm8);

        if self.cond(c) {
            self.registers.pc = self.registers.pc.wrapping_add(val as i8 as u16);
            self.tick(bus); // cycle +1
        }
    }

    // jump
    pub fn jp(&mut self, bus: &mut Bus) {
        let val = self.read16(bus, Imm16);
        self.registers.pc = val;
        self.tick(bus); // cycle +1
    }

    // jump if condition
    pub fn jp_c(&mut self, bus: &mut Bus, c: Cond) {
        let val = self.read16(bus, Imm16);

        if self.cond(c) {
            self.registers.pc = val;
            self.tick(bus); // cycle +1
        }
    }

    // jump to HL
    pub fn jp_hl(&mut self, _: &mut Bus) {
        self.registers.pc = self.registers.hl();
    }

    // call subroutine
    pub fn call(&mut self, bus: &mut Bus) {
        let val = self.read16(bus, Imm16);
        self.push16(bus, self.registers.pc);
        self.registers.pc = val;
        self.tick(bus); // cycle +1
    }

    // call subroutine if condition
    pub fn call_c(&mut self, bus: &mut Bus, c: Cond) {
        let val = self.read16(bus, Imm16);

        if self.cond(c) {
            self.push16(bus, self.registers.pc);
            self.registers.pc = val;
            self.tick(bus); // cycle +1
        }
    }

    // return from subroutine
    pub fn ret(&mut self, bus: &mut Bus) {
        let val = self.pop16(bus);
        self.registers.pc = val;
        self.tick(bus); // cycle +1
    }

    // return from subroutine if condition
    pub fn ret_c(&mut self, bus: &mut Bus, c: Cond) {
        self.tick(bus);
        if self.cond(c) {
            let val = self.pop16(bus);
            self.registers.pc = val;
            self.tick(bus); // cycle +1
        }
    }

    // return from interrupts
    pub fn reti(&mut self, bus: &mut Bus) {
        self.ret(bus);
        self.interrupts.ime = true;
    }

    // restart
    pub fn rst(&mut self, bus: &mut Bus, addr: u16) {
        self.push16(bus, self.registers.pc);
        self.registers.pc = addr;
        self.tick(bus); // cycle +1
    }

    // cy=cy xor 1
    pub fn ccf(&mut self, _: &mut Bus) {
        self.registers.set_nf(false);
        self.registers.set_hf(false);
        self.registers.set_cf(!self.registers.cf());
    }

    // cy=1
    pub fn scf(&mut self, _: &mut Bus) {
        self.registers.set_nf(false);
        self.registers.set_hf(false);
        self.registers.set_cf(true);
    }

    // enable interrupts
    pub fn ei(&mut self, _: &mut Bus) {
        self.ei_delay = true;
    }

    // disable interrupts
    pub fn di(&mut self, _: &mut Bus) {
        self.interrupts.ime = false;
    }

    pub fn undefined(&mut self, _: &mut Bus) {
        panic!("undefined instruction {:2X}", self.ctx.opcode);
    }
}
