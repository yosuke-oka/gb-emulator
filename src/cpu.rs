mod decode;
mod fetch;
mod instruction;
mod interrupt;
mod operand;
mod registers;

use self::interrupt::Interrupts;
use self::registers::Registers;
use crate::peripherals::Peripherals;

#[derive(Default)]
struct Ctx {
    opcode: u8,
    cb: bool,
    interrupt: bool,
}

pub struct Cpu {
    registers: Registers,
    interrupts: Interrupts,
    ctx: Ctx,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            registers: Registers::default(),
            interrupts: Interrupts::default(),
            ctx: Ctx::default(),
        }
    }
    pub fn emulate_cycle(&mut self, bus: &mut Peripherals) {
        if self.ctx.interrupt {
            self.call_isr(bus);
        } else {
            self.decode(bus);
        }
    }

    fn call_isr(&mut self, bus: &mut Peripherals) {}
}
