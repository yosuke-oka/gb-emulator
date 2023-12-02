mod decode;
mod fetch;
mod instruction;
pub mod interrupt;
mod operand;
mod registers;

use self::interrupt::{Interrupts, JOYPAD, LCD_STAT, SERIAL, TIMER, VBLANK};
use self::registers::Registers;
use crate::peripherals::Peripherals;

#[derive(Default)]
struct Ctx {
    opcode: u8,
    cb: bool,
    interrupt: bool,
    elapsed_cycle: u8,
}

pub struct Cpu {
    registers: Registers,
    pub interrupts: Interrupts,
    halting: bool,
    ei_delay: bool,
    ctx: Ctx,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            registers: Registers::default(),
            interrupts: Interrupts::default(),
            halting: false,
            ei_delay: false,
            ctx: Ctx::default(),
        }
    }
    pub fn emulate_cycle(&mut self, bus: &mut Peripherals) -> u8 {
        self.ctx.elapsed_cycle = 1;
        if self.ctx.interrupt {
            self.call_isr(bus);
        } else {
            self.decode(bus);
        }

        // fetch / execute overlap
        if !self.halting {
            self.fetch(bus);
        }

        // ei は fetch のあとに割り込みフラグをtrueにする
        if self.ei_delay {
            self.interrupts.ime = true;
            self.ei_delay = false;
        }
        //println!(" elapsed_cycle: {}", self.ctx.elapsed_cycle);
        return self.ctx.elapsed_cycle;
    }

    fn call_isr(&mut self, bus: &mut Peripherals) {
        self.push16(bus, self.registers.pc);
        let highest_interrupt = 1 << self.interrupts.get_interrupt().trailing_zeros();
        self.interrupts.interrupt_flags &= !highest_interrupt;
        // cal isr
        self.registers.pc = match highest_interrupt {
            VBLANK => 0x0040,
            LCD_STAT => 0x0048,
            TIMER => 0x0050,
            SERIAL => 0x0058,
            JOYPAD => 0x0060,
            _ => panic!("invalid interrupt: {:02X}", highest_interrupt),
        };

        self.ctx.interrupt = false;
    }

    fn tick(&mut self) {
        self.ctx.elapsed_cycle += 1;
    }

    fn read_bus(&mut self, bus: &Peripherals, addr: u16) -> u8 {
        let val = bus.read(&self.interrupts, addr);
        self.tick();
        val
    }

    fn write_bus(&mut self, bus: &mut Peripherals, addr: u16, val: u8) {
        bus.write(&mut self.interrupts, addr, val);
        self.tick();
    }
}
