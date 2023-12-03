use crate::cpu::interrupt::{Interrupts, TIMER};

#[derive(Default)]
pub struct Timer {
    div: u16,
    tima: u8,
    overflow: bool,
    tma: u8,
    tac: u8,
}

impl Timer {
    pub fn emulate_cycle(&mut self, interrupts: &mut Interrupts) {
        self.div = self.div.wrapping_add(4);

        if self.tac & 0b100 == 0 {
            return;
        }

        let modulo = match self.tac & 0b11 {
            0b00 => 1024,
            0b01 => 16,
            0b10 => 64,
            0b11 => 256,
            _ => unreachable!(),
        };

        if self.overflow {
            self.tima = self.tma;
            self.overflow = false;
            interrupts.irq(TIMER);
        } else if self.div & (modulo - 1) == 0 {
            let (tima, overflow) = self.tima.overflowing_add(1);
            self.tima = tima;
            self.overflow = overflow;
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0xFF04 => (self.div >> 8) as u8,
            0xFF05 => self.tima,
            0xFF06 => self.tma,
            0xFF07 => 0xF8 | self.tac, // upper 5 bits are always 1
            _ => panic!("Invalid timer read: {:04x}", addr),
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0xFF04 => self.div = 0, // write to DIV resets it
            0xFF05 => {
                if !self.overflow {
                    self.tima = val
                }
            }
            0xFF06 => self.tma = val,
            0xFF07 => self.tac = val & 0b111,
            _ => panic!("Invalid timer write: {:04x}", addr),
        }
    }
}
