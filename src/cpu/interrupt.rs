#[derive(Default)]
pub struct Interrupts {
    pub ime: bool, // interrupt master enable
    pub interrupt_enable: u8,
    pub interrupt_flags: u8,
}

pub const VBLANK: u8 = 1 << 0;
pub const LCD_STAT: u8 = 1 << 1;
pub const TIMER: u8 = 1 << 2;
pub const SERIAL: u8 = 1 << 3;
pub const JOYPAD: u8 = 1 << 4;

impl Interrupts {
    // interrupt request
    pub fn irq(&mut self, val: u8) {
        self.interrupt_flags |= val;
    }

    pub fn get_interrupt(&self) -> u8 {
        self.interrupt_flags & self.interrupt_enable & 0b11111
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0xff0f => self.interrupt_flags,
            0xffff => self.interrupt_enable,
            _ => panic!("Invalid interrupt read: {:04x}", addr),
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0xff0f => self.interrupt_flags = val,
            0xffff => self.interrupt_enable = val,
            _ => panic!("Invalid interrupt write: {:04x}", addr),
        }
    }
}
