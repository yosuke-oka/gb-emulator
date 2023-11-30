mod decode;
mod fetch;
mod instruction;
pub mod interrupt;
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
    elapsed_cycle: usize,
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
    pub fn emulate_cycle(&mut self, bus: &mut Peripherals) -> usize {
        self.ctx.elapsed_cycle = 1;
        if self.ctx.interrupt {
            self.call_isr(bus);
        } else {
            self.decode(bus);
        }

        // fetch / execute overlap
        self.fetch(bus);

        return self.ctx.elapsed_cycle;
    }

    fn call_isr(&mut self, bus: &mut Peripherals) {
        //todo
    }

    fn tick(&mut self) {
        self.ctx.elapsed_cycle += 1;
    }

    fn read_bus(&mut self, bus: &Peripherals, addr: u16) -> u8 {
        self.tick();
        bus.read(&self.interrupts, addr)
    }

    fn write_bus(&mut self, bus: &mut Peripherals, addr: u16, val: u8) {
        self.tick();
        bus.write(&mut self.interrupts, addr, val)
    }
}
