mod decode;
mod fetch;
mod instruction;
mod operand;
mod registers;

use crate::cpu::registers::Registers;
use crate::peripherals::Peripherals;

#[derive(Default)]
struct Ctx {
    opcode: u8,
    cb: bool,
}

pub struct Cpu {
    registers: Registers,
    ctx: Ctx,
}

impl Cpu {
    pub fn emulate_cycle(&mut self, bus: &Peripherals) {
        self.decode(bus);
    }
}
