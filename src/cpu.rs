mod decode;
mod fetch;
mod instruction;
pub mod interrupt;
mod operand;
mod registers;

use self::interrupt::{Interrupts, JOYPAD, LCD_STAT, SERIAL, TIMER, VBLANK};
use self::registers::Registers;
use crate::bus::Bus;

#[derive(Default)]
struct Ctx {
    opcode: u8,
    cb: bool,
    interrupt: bool,
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
    pub fn emulate_cycle(&mut self, bus: &mut Bus) {
        if self.ctx.interrupt {
            self.call_isr(bus);
        } else {
            self.decode(bus);
        }

        if self.halting {
            self.tick(bus);
            return;
        }

        // fetch / execute overlap
        self.fetch(bus);

        // ei は fetch のあとに割り込みフラグをtrueにする
        if self.ei_delay {
            self.interrupts.ime = true;
            self.ei_delay = false;
        }
    }

    fn call_isr(&mut self, bus: &mut Bus) {
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

    fn tick(&mut self, bus: &mut Bus) {
        bus.tick(&mut self.interrupts);
    }

    fn read_bus(&mut self, bus: &mut Bus, addr: u16) -> u8 {
        let val = bus.read(&self.interrupts, addr);
        self.tick(bus);
        val
    }

    fn write_bus(&mut self, bus: &mut Bus, addr: u16, val: u8) {
        bus.write(&mut self.interrupts, addr, val);
        self.tick(bus);
    }
}
