use crate::cpu::Cpu;
use crate::peripherals::Peripherals;

impl Cpu {
    pub fn fetch(&mut self, bus: &Peripherals) {
        self.ctx.opcode = bus.read(self.regiters.pc);
        self.regiters.pc = self.regiters.pc.wrapping_add(1);
        self.ctx.cb = false;
    }
}
