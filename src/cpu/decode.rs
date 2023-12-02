use crate::cpu::Cpu;
use crate::peripherals::Peripherals;

use super::operand::{Cond, Direct16, Direct8, Imm16, Imm8, Indirect, Reg16, Reg8, IO8};

impl Cpu {
    // gameboy opecodes
    // https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html
    pub fn decode(&mut self, bus: &mut Peripherals) {
        if self.ctx.cb {
            self.cb_decode(bus);
            return;
        }
        //print!("opecode: {:02x}", self.ctx.opcode);
        match self.ctx.opcode {
            0x00 => self.nop(bus),
            0x10 => self.stop(bus),
            0x20 => self.jr_c(bus, Cond::NZ),
            0x30 => self.jr_c(bus, Cond::NC),
            0x01 => self.ld16(bus, Reg16::BC, Imm16),
            0x11 => self.ld16(bus, Reg16::DE, Imm16),
            0x21 => self.ld16(bus, Reg16::HL, Imm16),
            0x31 => self.ld16(bus, Reg16::SP, Imm16),
            0x02 => self.ld(bus, Indirect::BC, Reg8::A),
            0x12 => self.ld(bus, Indirect::DE, Reg8::A),
            0x22 => self.ld(bus, Indirect::HLI, Reg8::A),
            0x32 => self.ld(bus, Indirect::HLD, Reg8::A),
            0x03 => self.inc16(bus, Reg16::BC),
            0x13 => self.inc16(bus, Reg16::DE),
            0x23 => self.inc16(bus, Reg16::HL),
            0x33 => self.inc16(bus, Reg16::SP),
            0x04 => self.inc(bus, Reg8::B),
            0x14 => self.inc(bus, Reg8::D),
            0x24 => self.inc(bus, Reg8::H),
            0x34 => self.inc(bus, Indirect::HL),
            0x05 => self.dec(bus, Reg8::B),
            0x15 => self.dec(bus, Reg8::D),
            0x25 => self.dec(bus, Reg8::H),
            0x35 => self.dec(bus, Indirect::HL),
            0x06 => self.ld(bus, Reg8::B, Imm8),
            0x16 => self.ld(bus, Reg8::D, Imm8),
            0x26 => self.ld(bus, Reg8::H, Imm8),
            0x36 => self.ld(bus, Indirect::HL, Imm8),
            0x07 => self.rlca(bus),
            0x17 => self.rla(bus),
            0x27 => self.daa(bus),
            0x37 => self.scf(bus),
            0x08 => self.ld16(bus, Direct16, Reg16::SP),
            0x18 => self.jr(bus),
            0x28 => self.jr_c(bus, Cond::Z),
            0x38 => self.jr_c(bus, Cond::C),
            0x09 => self.add_hl_reg16(bus, Reg16::BC),
            0x19 => self.add_hl_reg16(bus, Reg16::DE),
            0x29 => self.add_hl_reg16(bus, Reg16::HL),
            0x39 => self.add_hl_reg16(bus, Reg16::SP),
            0x0A => self.ld(bus, Reg8::A, Indirect::BC),
            0x1A => self.ld(bus, Reg8::A, Indirect::DE),
            0x2A => self.ld(bus, Reg8::A, Indirect::HLI),
            0x3A => self.ld(bus, Reg8::A, Indirect::HLD),
            0x0B => self.dec16(bus, Reg16::BC),
            0x1B => self.dec16(bus, Reg16::DE),
            0x2B => self.dec16(bus, Reg16::HL),
            0x3B => self.dec16(bus, Reg16::SP),
            0x0C => self.inc(bus, Reg8::C),
            0x1C => self.inc(bus, Reg8::E),
            0x2C => self.inc(bus, Reg8::L),
            0x3C => self.inc(bus, Reg8::A),
            0x0D => self.dec(bus, Reg8::C),
            0x1D => self.dec(bus, Reg8::E),
            0x2D => self.dec(bus, Reg8::L),
            0x3D => self.dec(bus, Reg8::A),
            0x0E => self.ld(bus, Reg8::C, Imm8),
            0x1E => self.ld(bus, Reg8::E, Imm8),
            0x2E => self.ld(bus, Reg8::L, Imm8),
            0x3E => self.ld(bus, Reg8::A, Imm8),
            0x0F => self.rrca(bus),
            0x1F => self.rra(bus),
            0x2F => self.cpl(bus),
            0x3F => self.ccf(bus),
            0x40 => self.ld(bus, Reg8::B, Reg8::B),
            0x50 => self.ld(bus, Reg8::D, Reg8::B),
            0x60 => self.ld(bus, Reg8::H, Reg8::B),
            0x70 => self.ld(bus, Indirect::HL, Reg8::B),
            0x41 => self.ld(bus, Reg8::B, Reg8::C),
            0x51 => self.ld(bus, Reg8::D, Reg8::C),
            0x61 => self.ld(bus, Reg8::H, Reg8::C),
            0x71 => self.ld(bus, Indirect::HL, Reg8::C),
            0x42 => self.ld(bus, Reg8::B, Reg8::D),
            0x52 => self.ld(bus, Reg8::D, Reg8::D),
            0x62 => self.ld(bus, Reg8::H, Reg8::D),
            0x72 => self.ld(bus, Indirect::HL, Reg8::D),
            0x43 => self.ld(bus, Reg8::B, Reg8::E),
            0x53 => self.ld(bus, Reg8::D, Reg8::E),
            0x63 => self.ld(bus, Reg8::H, Reg8::E),
            0x73 => self.ld(bus, Indirect::HL, Reg8::E),
            0x44 => self.ld(bus, Reg8::B, Reg8::H),
            0x54 => self.ld(bus, Reg8::D, Reg8::H),
            0x64 => self.ld(bus, Reg8::H, Reg8::H),
            0x74 => self.ld(bus, Indirect::HL, Reg8::H),
            0x45 => self.ld(bus, Reg8::B, Reg8::L),
            0x55 => self.ld(bus, Reg8::D, Reg8::L),
            0x65 => self.ld(bus, Reg8::H, Reg8::L),
            0x75 => self.ld(bus, Indirect::HL, Reg8::L),
            0x46 => self.ld(bus, Reg8::B, Indirect::HL),
            0x56 => self.ld(bus, Reg8::D, Indirect::HL),
            0x66 => self.ld(bus, Reg8::H, Indirect::HL),
            0x76 => self.halt(bus),
            0x47 => self.ld(bus, Reg8::B, Reg8::A),
            0x57 => self.ld(bus, Reg8::D, Reg8::A),
            0x67 => self.ld(bus, Reg8::H, Reg8::A),
            0x77 => self.ld(bus, Indirect::HL, Reg8::A),
            0x48 => self.ld(bus, Reg8::C, Reg8::B),
            0x58 => self.ld(bus, Reg8::E, Reg8::B),
            0x68 => self.ld(bus, Reg8::L, Reg8::B),
            0x78 => self.ld(bus, Reg8::A, Reg8::B),
            0x49 => self.ld(bus, Reg8::C, Reg8::C),
            0x59 => self.ld(bus, Reg8::E, Reg8::C),
            0x69 => self.ld(bus, Reg8::L, Reg8::C),
            0x79 => self.ld(bus, Reg8::A, Reg8::C),
            0x4A => self.ld(bus, Reg8::C, Reg8::D),
            0x5A => self.ld(bus, Reg8::E, Reg8::D),
            0x6A => self.ld(bus, Reg8::L, Reg8::D),
            0x7A => self.ld(bus, Reg8::A, Reg8::D),
            0x4B => self.ld(bus, Reg8::C, Reg8::E),
            0x5B => self.ld(bus, Reg8::E, Reg8::E),
            0x6B => self.ld(bus, Reg8::L, Reg8::E),
            0x7B => self.ld(bus, Reg8::A, Reg8::E),
            0x4C => self.ld(bus, Reg8::C, Reg8::H),
            0x5C => self.ld(bus, Reg8::E, Reg8::H),
            0x6C => self.ld(bus, Reg8::L, Reg8::H),
            0x7C => self.ld(bus, Reg8::A, Reg8::H),
            0x4D => self.ld(bus, Reg8::C, Reg8::L),
            0x5D => self.ld(bus, Reg8::E, Reg8::L),
            0x6D => self.ld(bus, Reg8::L, Reg8::L),
            0x7D => self.ld(bus, Reg8::A, Reg8::L),
            0x4E => self.ld(bus, Reg8::C, Indirect::HL),
            0x5E => self.ld(bus, Reg8::E, Indirect::HL),
            0x6E => self.ld(bus, Reg8::L, Indirect::HL),
            0x7E => self.ld(bus, Reg8::A, Indirect::HL),
            0x4F => self.ld(bus, Reg8::C, Reg8::A),
            0x5F => self.ld(bus, Reg8::E, Reg8::A),
            0x6F => self.ld(bus, Reg8::L, Reg8::A),
            0x7F => self.ld(bus, Reg8::A, Reg8::A),
            0x80 => self.add(bus, Reg8::B),
            0x90 => self.sub(bus, Reg8::B),
            0xA0 => self.and(bus, Reg8::B),
            0xB0 => self.or(bus, Reg8::B),
            0x81 => self.add(bus, Reg8::C),
            0x91 => self.sub(bus, Reg8::C),
            0xA1 => self.and(bus, Reg8::C),
            0xB1 => self.or(bus, Reg8::C),
            0x82 => self.add(bus, Reg8::D),
            0x92 => self.sub(bus, Reg8::D),
            0xA2 => self.and(bus, Reg8::D),
            0xB2 => self.or(bus, Reg8::D),
            0x83 => self.add(bus, Reg8::E),
            0x93 => self.sub(bus, Reg8::E),
            0xA3 => self.and(bus, Reg8::E),
            0xB3 => self.or(bus, Reg8::E),
            0x84 => self.add(bus, Reg8::H),
            0x94 => self.sub(bus, Reg8::H),
            0xA4 => self.and(bus, Reg8::H),
            0xB4 => self.or(bus, Reg8::H),
            0x85 => self.add(bus, Reg8::L),
            0x95 => self.sub(bus, Reg8::L),
            0xA5 => self.and(bus, Reg8::L),
            0xB5 => self.or(bus, Reg8::L),
            0x86 => self.add(bus, Indirect::HL),
            0x96 => self.sub(bus, Indirect::HL),
            0xA6 => self.and(bus, Indirect::HL),
            0xB6 => self.or(bus, Indirect::HL),
            0x87 => self.add(bus, Reg8::A),
            0x97 => self.sub(bus, Reg8::A),
            0xA7 => self.and(bus, Reg8::A),
            0xB7 => self.or(bus, Reg8::A),
            0x88 => self.adc(bus, Reg8::B),
            0x98 => self.sbc(bus, Reg8::B),
            0xA8 => self.xor(bus, Reg8::B),
            0xB8 => self.cp(bus, Reg8::B),
            0x89 => self.adc(bus, Reg8::C),
            0x99 => self.sbc(bus, Reg8::C),
            0xA9 => self.xor(bus, Reg8::C),
            0xB9 => self.cp(bus, Reg8::C),
            0x8A => self.adc(bus, Reg8::D),
            0x9A => self.sbc(bus, Reg8::D),
            0xAA => self.xor(bus, Reg8::D),
            0xBA => self.cp(bus, Reg8::D),
            0x8B => self.adc(bus, Reg8::E),
            0x9B => self.sbc(bus, Reg8::E),
            0xAB => self.xor(bus, Reg8::E),
            0xBB => self.cp(bus, Reg8::E),
            0x8C => self.adc(bus, Reg8::H),
            0x9C => self.sbc(bus, Reg8::H),
            0xAC => self.xor(bus, Reg8::H),
            0xBC => self.cp(bus, Reg8::H),
            0x8D => self.adc(bus, Reg8::L),
            0x9D => self.sbc(bus, Reg8::L),
            0xAD => self.xor(bus, Reg8::L),
            0xBD => self.cp(bus, Reg8::L),
            0x8E => self.adc(bus, Indirect::HL),
            0x9E => self.sbc(bus, Indirect::HL),
            0xAE => self.xor(bus, Indirect::HL),
            0xBE => self.cp(bus, Indirect::HL),
            0x8F => self.adc(bus, Reg8::A),
            0x9F => self.sbc(bus, Reg8::A),
            0xAF => self.xor(bus, Reg8::A),
            0xBF => self.cp(bus, Reg8::A),
            0xC0 => self.ret_c(bus, Cond::NZ),
            0xD0 => self.ret_c(bus, Cond::NC),
            0xE0 => self.ld(bus, Direct8::DFF, Reg8::A),
            0xF0 => self.ld(bus, Reg8::A, Direct8::DFF),
            0xC1 => self.pop(bus, Reg16::BC),
            0xD1 => self.pop(bus, Reg16::DE),
            0xE1 => self.pop(bus, Reg16::HL),
            0xF1 => self.pop(bus, Reg16::AF),
            0xC2 => self.jp_c(bus, Cond::NZ),
            0xD2 => self.jp_c(bus, Cond::NC),
            0xE2 => self.ld(bus, Indirect::CFF, Reg8::A),
            0xF2 => self.ld(bus, Reg8::A, Indirect::CFF),
            0xC3 => self.jp(bus),
            0xD3 => self.undefined(bus),
            0xE3 => self.undefined(bus),
            0xF3 => self.di(bus),
            0xC4 => self.call_c(bus, Cond::NZ),
            0xD4 => self.call_c(bus, Cond::NC),
            0xE4 => self.undefined(bus),
            0xF4 => self.undefined(bus),
            0xC5 => self.push(bus, Reg16::BC),
            0xD5 => self.push(bus, Reg16::DE),
            0xE5 => self.push(bus, Reg16::HL),
            0xF5 => self.push(bus, Reg16::AF),
            0xC6 => self.add(bus, Imm8),
            0xD6 => self.sub(bus, Imm8),
            0xE6 => self.and(bus, Imm8),
            0xF6 => self.or(bus, Imm8),
            0xC7 => self.rst(bus, 0x00),
            0xD7 => self.rst(bus, 0x10),
            0xE7 => self.rst(bus, 0x20),
            0xF7 => self.rst(bus, 0x30),
            0xC8 => self.ret_c(bus, Cond::Z),
            0xD8 => self.ret_c(bus, Cond::C),
            0xE8 => self.add_sp_e(bus),
            0xF8 => self.ld_hl_sp_e(bus),
            0xC9 => self.ret(bus),
            0xD9 => self.reti(bus),
            0xE9 => self.jp_hl(bus),
            0xF9 => self.ld_sp_hl(bus),
            0xCA => self.jp_c(bus, Cond::Z),
            0xDA => self.jp_c(bus, Cond::C),
            0xEA => self.ld(bus, Direct8::D, Reg8::A),
            0xFA => self.ld(bus, Reg8::A, Direct8::D),
            0xCB => self.cb_prefixed(bus),
            0xDB => self.undefined(bus),
            0xEB => self.undefined(bus),
            0xFB => self.ei(bus),
            0xCC => self.call_c(bus, Cond::Z),
            0xDC => self.call_c(bus, Cond::C),
            0xEC => self.undefined(bus),
            0xFC => self.undefined(bus),
            0xCD => self.call(bus),
            0xDD => self.undefined(bus),
            0xED => self.undefined(bus),
            0xFD => self.undefined(bus),
            0xCE => self.adc(bus, Imm8),
            0xDE => self.sbc(bus, Imm8),
            0xEE => self.xor(bus, Imm8),
            0xFE => self.cp(bus, Imm8),
            0xCF => self.rst(bus, 0x08),
            0xDF => self.rst(bus, 0x18),
            0xEF => self.rst(bus, 0x28),
            0xFF => self.rst(bus, 0x38),
        }
    }

    pub fn cb_decode(&mut self, bus: &mut Peripherals) {
        //print!(" opecode: {:02x} cb: t", self.ctx.opcode);
        match self.ctx.opcode {
            0x00 => self.rlc(bus, Reg8::B),
            0x10 => self.rl(bus, Reg8::B),
            0x20 => self.sla(bus, Reg8::B),
            0x30 => self.swap(bus, Reg8::B),
            0x01 => self.rlc(bus, Reg8::C),
            0x11 => self.rl(bus, Reg8::C),
            0x21 => self.sla(bus, Reg8::C),
            0x31 => self.swap(bus, Reg8::C),
            0x02 => self.rlc(bus, Reg8::D),
            0x12 => self.rl(bus, Reg8::D),
            0x22 => self.sla(bus, Reg8::D),
            0x32 => self.swap(bus, Reg8::D),
            0x03 => self.rlc(bus, Reg8::E),
            0x13 => self.rl(bus, Reg8::E),
            0x23 => self.sla(bus, Reg8::E),
            0x33 => self.swap(bus, Reg8::E),
            0x04 => self.rlc(bus, Reg8::H),
            0x14 => self.rl(bus, Reg8::H),
            0x24 => self.sla(bus, Reg8::H),
            0x34 => self.swap(bus, Reg8::H),
            0x05 => self.rlc(bus, Reg8::L),
            0x15 => self.rl(bus, Reg8::L),
            0x25 => self.sla(bus, Reg8::L),
            0x35 => self.swap(bus, Reg8::L),
            0x06 => self.rlc(bus, Indirect::HL),
            0x16 => self.rl(bus, Indirect::HL),
            0x26 => self.sla(bus, Indirect::HL),
            0x36 => self.swap(bus, Indirect::HL),
            0x07 => self.rlc(bus, Reg8::A),
            0x17 => self.rl(bus, Reg8::A),
            0x27 => self.sla(bus, Reg8::A),
            0x37 => self.swap(bus, Reg8::A),
            0x08 => self.rrc(bus, Reg8::B),
            0x18 => self.rr(bus, Reg8::B),
            0x28 => self.sra(bus, Reg8::B),
            0x38 => self.srl(bus, Reg8::B),
            0x09 => self.rrc(bus, Reg8::C),
            0x19 => self.rr(bus, Reg8::C),
            0x29 => self.sra(bus, Reg8::C),
            0x39 => self.srl(bus, Reg8::C),
            0x0A => self.rrc(bus, Reg8::D),
            0x1A => self.rr(bus, Reg8::D),
            0x2A => self.sra(bus, Reg8::D),
            0x3A => self.srl(bus, Reg8::D),
            0x0B => self.rrc(bus, Reg8::E),
            0x1B => self.rr(bus, Reg8::E),
            0x2B => self.sra(bus, Reg8::E),
            0x3B => self.srl(bus, Reg8::E),
            0x0C => self.rrc(bus, Reg8::H),
            0x1C => self.rr(bus, Reg8::H),
            0x2C => self.sra(bus, Reg8::H),
            0x3C => self.srl(bus, Reg8::H),
            0x0D => self.rrc(bus, Reg8::L),
            0x1D => self.rr(bus, Reg8::L),
            0x2D => self.sra(bus, Reg8::L),
            0x3D => self.srl(bus, Reg8::L),
            0x0E => self.rrc(bus, Indirect::HL),
            0x1E => self.rr(bus, Indirect::HL),
            0x2E => self.sra(bus, Indirect::HL),
            0x3E => self.srl(bus, Indirect::HL),
            0x0F => self.rrc(bus, Reg8::A),
            0x1F => self.rr(bus, Reg8::A),
            0x2F => self.sra(bus, Reg8::A),
            0x3F => self.srl(bus, Reg8::A),
            0x40 => self.bit(bus, 0, Reg8::B),
            0x50 => self.bit(bus, 2, Reg8::B),
            0x60 => self.bit(bus, 4, Reg8::B),
            0x70 => self.bit(bus, 6, Reg8::B),
            0x41 => self.bit(bus, 0, Reg8::C),
            0x51 => self.bit(bus, 2, Reg8::C),
            0x61 => self.bit(bus, 4, Reg8::C),
            0x71 => self.bit(bus, 6, Reg8::C),
            0x42 => self.bit(bus, 0, Reg8::D),
            0x52 => self.bit(bus, 2, Reg8::D),
            0x62 => self.bit(bus, 4, Reg8::D),
            0x72 => self.bit(bus, 6, Reg8::D),
            0x43 => self.bit(bus, 0, Reg8::E),
            0x53 => self.bit(bus, 2, Reg8::E),
            0x63 => self.bit(bus, 4, Reg8::E),
            0x73 => self.bit(bus, 6, Reg8::E),
            0x44 => self.bit(bus, 0, Reg8::H),
            0x54 => self.bit(bus, 2, Reg8::H),
            0x64 => self.bit(bus, 4, Reg8::H),
            0x74 => self.bit(bus, 6, Reg8::H),
            0x45 => self.bit(bus, 0, Reg8::L),
            0x55 => self.bit(bus, 2, Reg8::L),
            0x65 => self.bit(bus, 4, Reg8::L),
            0x75 => self.bit(bus, 6, Reg8::L),
            0x46 => self.bit(bus, 0, Indirect::HL),
            0x56 => self.bit(bus, 2, Indirect::HL),
            0x66 => self.bit(bus, 4, Indirect::HL),
            0x76 => self.bit(bus, 6, Indirect::HL),
            0x47 => self.bit(bus, 0, Reg8::A),
            0x57 => self.bit(bus, 2, Reg8::A),
            0x67 => self.bit(bus, 4, Reg8::A),
            0x77 => self.bit(bus, 6, Reg8::A),
            0x48 => self.bit(bus, 1, Reg8::B),
            0x58 => self.bit(bus, 3, Reg8::B),
            0x68 => self.bit(bus, 5, Reg8::B),
            0x78 => self.bit(bus, 7, Reg8::B),
            0x49 => self.bit(bus, 1, Reg8::C),
            0x59 => self.bit(bus, 3, Reg8::C),
            0x69 => self.bit(bus, 5, Reg8::C),
            0x79 => self.bit(bus, 7, Reg8::C),
            0x4A => self.bit(bus, 1, Reg8::D),
            0x5A => self.bit(bus, 3, Reg8::D),
            0x6A => self.bit(bus, 5, Reg8::D),
            0x7A => self.bit(bus, 7, Reg8::D),
            0x4B => self.bit(bus, 1, Reg8::E),
            0x5B => self.bit(bus, 3, Reg8::E),
            0x6B => self.bit(bus, 5, Reg8::E),
            0x7B => self.bit(bus, 7, Reg8::E),
            0x4C => self.bit(bus, 1, Reg8::H),
            0x5C => self.bit(bus, 3, Reg8::H),
            0x6C => self.bit(bus, 5, Reg8::H),
            0x7C => self.bit(bus, 7, Reg8::H),
            0x4D => self.bit(bus, 1, Reg8::L),
            0x5D => self.bit(bus, 3, Reg8::L),
            0x6D => self.bit(bus, 5, Reg8::L),
            0x7D => self.bit(bus, 7, Reg8::L),
            0x4E => self.bit(bus, 1, Indirect::HL),
            0x5E => self.bit(bus, 3, Indirect::HL),
            0x6E => self.bit(bus, 5, Indirect::HL),
            0x7E => self.bit(bus, 7, Indirect::HL),
            0x4F => self.bit(bus, 1, Reg8::A),
            0x5F => self.bit(bus, 3, Reg8::A),
            0x6F => self.bit(bus, 5, Reg8::A),
            0x7F => self.bit(bus, 7, Reg8::A),
            0x80 => self.res(bus, 0, Reg8::B),
            0x90 => self.res(bus, 2, Reg8::B),
            0xA0 => self.res(bus, 4, Reg8::B),
            0xB0 => self.res(bus, 6, Reg8::B),
            0x81 => self.res(bus, 0, Reg8::C),
            0x91 => self.res(bus, 2, Reg8::C),
            0xA1 => self.res(bus, 4, Reg8::C),
            0xB1 => self.res(bus, 6, Reg8::C),
            0x82 => self.res(bus, 0, Reg8::D),
            0x92 => self.res(bus, 2, Reg8::D),
            0xA2 => self.res(bus, 4, Reg8::D),
            0xB2 => self.res(bus, 6, Reg8::D),
            0x83 => self.res(bus, 0, Reg8::E),
            0x93 => self.res(bus, 2, Reg8::E),
            0xA3 => self.res(bus, 4, Reg8::E),
            0xB3 => self.res(bus, 6, Reg8::E),
            0x84 => self.res(bus, 0, Reg8::H),
            0x94 => self.res(bus, 2, Reg8::H),
            0xA4 => self.res(bus, 4, Reg8::H),
            0xB4 => self.res(bus, 6, Reg8::H),
            0x85 => self.res(bus, 0, Reg8::L),
            0x95 => self.res(bus, 2, Reg8::L),
            0xA5 => self.res(bus, 4, Reg8::L),
            0xB5 => self.res(bus, 6, Reg8::L),
            0x86 => self.res(bus, 0, Indirect::HL),
            0x96 => self.res(bus, 2, Indirect::HL),
            0xA6 => self.res(bus, 4, Indirect::HL),
            0xB6 => self.res(bus, 6, Indirect::HL),
            0x87 => self.res(bus, 0, Reg8::A),
            0x97 => self.res(bus, 2, Reg8::A),
            0xA7 => self.res(bus, 4, Reg8::A),
            0xB7 => self.res(bus, 6, Reg8::A),
            0x88 => self.res(bus, 1, Reg8::B),
            0x98 => self.res(bus, 3, Reg8::B),
            0xA8 => self.res(bus, 5, Reg8::B),
            0xB8 => self.res(bus, 7, Reg8::B),
            0x89 => self.res(bus, 1, Reg8::C),
            0x99 => self.res(bus, 3, Reg8::C),
            0xA9 => self.res(bus, 5, Reg8::C),
            0xB9 => self.res(bus, 7, Reg8::C),
            0x8A => self.res(bus, 1, Reg8::D),
            0x9A => self.res(bus, 3, Reg8::D),
            0xAA => self.res(bus, 5, Reg8::D),
            0xBA => self.res(bus, 7, Reg8::D),
            0x8B => self.res(bus, 1, Reg8::E),
            0x9B => self.res(bus, 3, Reg8::E),
            0xAB => self.res(bus, 5, Reg8::E),
            0xBB => self.res(bus, 7, Reg8::E),
            0x8C => self.res(bus, 1, Reg8::H),
            0x9C => self.res(bus, 3, Reg8::H),
            0xAC => self.res(bus, 5, Reg8::H),
            0xBC => self.res(bus, 7, Reg8::H),
            0x8D => self.res(bus, 1, Reg8::L),
            0x9D => self.res(bus, 3, Reg8::L),
            0xAD => self.res(bus, 5, Reg8::L),
            0xBD => self.res(bus, 7, Reg8::L),
            0x8E => self.res(bus, 1, Indirect::HL),
            0x9E => self.res(bus, 3, Indirect::HL),
            0xAE => self.res(bus, 5, Indirect::HL),
            0xBE => self.res(bus, 7, Indirect::HL),
            0x8F => self.res(bus, 1, Reg8::A),
            0x9F => self.res(bus, 3, Reg8::A),
            0xAF => self.res(bus, 5, Reg8::A),
            0xBF => self.res(bus, 7, Reg8::A),
            0xC0 => self.set(bus, 0, Reg8::B),
            0xD0 => self.set(bus, 2, Reg8::B),
            0xE0 => self.set(bus, 4, Reg8::B),
            0xF0 => self.set(bus, 6, Reg8::B),
            0xC1 => self.set(bus, 0, Reg8::C),
            0xD1 => self.set(bus, 2, Reg8::C),
            0xE1 => self.set(bus, 4, Reg8::C),
            0xF1 => self.set(bus, 6, Reg8::C),
            0xC2 => self.set(bus, 0, Reg8::D),
            0xD2 => self.set(bus, 2, Reg8::D),
            0xE2 => self.set(bus, 4, Reg8::D),
            0xF2 => self.set(bus, 6, Reg8::D),
            0xC3 => self.set(bus, 0, Reg8::E),
            0xD3 => self.set(bus, 2, Reg8::E),
            0xE3 => self.set(bus, 4, Reg8::E),
            0xF3 => self.set(bus, 6, Reg8::E),
            0xC4 => self.set(bus, 0, Reg8::H),
            0xD4 => self.set(bus, 2, Reg8::H),
            0xE4 => self.set(bus, 4, Reg8::H),
            0xF4 => self.set(bus, 6, Reg8::H),
            0xC5 => self.set(bus, 0, Reg8::L),
            0xD5 => self.set(bus, 2, Reg8::L),
            0xE5 => self.set(bus, 4, Reg8::L),
            0xF5 => self.set(bus, 6, Reg8::L),
            0xC6 => self.set(bus, 0, Indirect::HL),
            0xD6 => self.set(bus, 2, Indirect::HL),
            0xE6 => self.set(bus, 4, Indirect::HL),
            0xF6 => self.set(bus, 6, Indirect::HL),
            0xC7 => self.set(bus, 0, Reg8::A),
            0xD7 => self.set(bus, 2, Reg8::A),
            0xE7 => self.set(bus, 4, Reg8::A),
            0xF7 => self.set(bus, 6, Reg8::A),
            0xC8 => self.set(bus, 1, Reg8::B),
            0xD8 => self.set(bus, 3, Reg8::B),
            0xE8 => self.set(bus, 5, Reg8::B),
            0xF8 => self.set(bus, 7, Reg8::B),
            0xC9 => self.set(bus, 1, Reg8::C),
            0xD9 => self.set(bus, 3, Reg8::C),
            0xE9 => self.set(bus, 5, Reg8::C),
            0xF9 => self.set(bus, 7, Reg8::C),
            0xCA => self.set(bus, 1, Reg8::D),
            0xDA => self.set(bus, 3, Reg8::D),
            0xEA => self.set(bus, 5, Reg8::D),
            0xFA => self.set(bus, 7, Reg8::D),
            0xCB => self.set(bus, 1, Reg8::E),
            0xDB => self.set(bus, 3, Reg8::E),
            0xEB => self.set(bus, 5, Reg8::E),
            0xFB => self.set(bus, 7, Reg8::E),
            0xCC => self.set(bus, 1, Reg8::H),
            0xDC => self.set(bus, 3, Reg8::H),
            0xEC => self.set(bus, 5, Reg8::H),
            0xFC => self.set(bus, 7, Reg8::H),
            0xCD => self.set(bus, 1, Reg8::L),
            0xDD => self.set(bus, 3, Reg8::L),
            0xED => self.set(bus, 5, Reg8::L),
            0xFD => self.set(bus, 7, Reg8::L),
            0xCE => self.set(bus, 1, Indirect::HL),
            0xDE => self.set(bus, 3, Indirect::HL),
            0xEE => self.set(bus, 5, Indirect::HL),
            0xFE => self.set(bus, 7, Indirect::HL),
            0xCF => self.set(bus, 1, Reg8::A),
            0xDF => self.set(bus, 3, Reg8::A),
            0xEF => self.set(bus, 5, Reg8::A),
            0xFF => self.set(bus, 7, Reg8::A),
        }
    }

    pub fn cb_prefixed(&mut self, bus: &mut Peripherals) {
        let v = self.read8(bus, Imm8);
        // TODO: need tick here ?
        //self.tick();
        self.ctx.opcode = v;
        self.ctx.cb = true;
        self.cb_decode(bus)
    }
}
