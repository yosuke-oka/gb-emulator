use crate::cpu::Cpu;
use crate::peripherals::Peripherals;

impl Cpu {
    pub fn decode(&mut self, bus: &Peripherals) {
        match self.ctx.opcode {
            0x00 => self.nop(bus),
            _ => panic!("Not implemented: {:02X}", self.ctx.opcode),
        }
    }
}
