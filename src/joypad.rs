use crate::cpu::interrupt::{self, Interrupts};

pub enum Buttons {
    A,
    B,
    Start,
    Select,
    Up,
    Down,
    Left,
    Right,
}

impl Buttons {
    fn as_direction(&self) -> u8 {
        match self {
            Buttons::Down => 0x08,
            Buttons::Up => 0x04,
            Buttons::Left => 0x02,
            Buttons::Right => 0x01,
            _ => 0x00,
        }
    }

    fn as_action(&self) -> u8 {
        match self {
            Buttons::Select => 0x08,
            Buttons::Start => 0x04,
            Buttons::B => 0x02,
            Buttons::A => 0x01,
            _ => 0x00,
        }
    }
}

pub struct Joypad {
    mode: u8,
    action: u8,
    direction: u8,
}

impl Joypad {
    pub fn new() -> Self {
        Self {
            mode: 0x00,
            action: 0xFF,
            direction: 0xFF,
        }
    }

    pub fn read(&self) -> u8 {
        let mut ret = 0xCF | self.mode;
        if ret & 0x10 == 0 {
            ret &= self.direction;
        }
        if ret & 0x20 == 0 {
            ret &= self.action;
        }
        ret
    }

    pub fn write(&mut self, data: u8) {
        self.mode = data & 0x30;
    }

    pub fn press(&mut self, interrupts: &mut Interrupts, button: Buttons) {
        self.action &= !button.as_action();
        self.direction &= !button.as_direction();
        interrupts.irq(interrupt::JOYPAD);
    }

    pub fn release(&mut self, interrupts: &mut Interrupts, button: Buttons) {
        self.action |= button.as_action();
        self.direction |= button.as_direction();
        interrupts.irq(interrupt::JOYPAD);
    }
}
