#[derive(Default)]
struct Ctx {
    opcode: u8,
    cb: bool,
}

pub struct Cpu {
    regiters: Registers,
    ctx: Ctx,
}

impl Cpu {
    pub fn fetch(&mut self, bus: &Peripherals) {
        self.ctx.opcode = bus.read(self.regiters.pc);
        self.regiters.pc = self.regiters.pc.wrapping_add(1);
        self.ctx.cb = false;
    }

    pub fn decode(&mut self, bus: &Peripherals) {
        match self.ctx.opcode {
            0x00 => self.nop(bus),
            _ => panic!("Not implemented: {:02X}", self.ctx.opcode),
        }
    }

    pub fn nop(&mut self, bus: &Peripherals) {
        // fetch / execute overlap
        self.fetch(bus);
    }

    pub fn emulate_cycle(&mut self, bus: &Peripherals) {
        self.decode(bus);
    }
}
