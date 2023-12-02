use crate::cpu::Cpu;
use crate::peripherals::Peripherals;

impl Cpu {
    pub fn fetch(&mut self, bus: &Peripherals) {
        self.ctx.opcode = self.read_bus(bus, self.registers.pc);
        if self.interrupts.ime && self.interrupts.get_interrupt() != 0 {
            self.ctx.interrupt = true;
        } else {
            self.registers.pc = self.registers.pc.wrapping_add(1);
            self.ctx.interrupt = false;
        }
        self.ctx.cb = false;
    }
}
