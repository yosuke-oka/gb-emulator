use crate::cpu::Cpu;
use crate::peripherals::Peripherals;

impl Cpu {
    pub fn nop(&mut self, bus: &Peripherals) {
        // fetch / execute overlap
        self.fetch(bus);
    }
}
