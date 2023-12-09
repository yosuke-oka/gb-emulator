use sdl2::{self, event::Event, keyboard::Keycode, Sdl};

use std::time;

use crate::{
    bootrom::BootRom, bus::Bus, cartridge::Cartridge, cpu::Cpu, joypad::Buttons, lcd::LCD,
};

pub const CPU_CLOCK_HZ: u128 = 4_194_304;
pub const M_CYCLE_CLOCK: u128 = 4;
const M_CYCLE_NANOS: u128 = M_CYCLE_CLOCK * 1_000_000_000 / CPU_CLOCK_HZ;

pub struct GameBoy {
    cpu: Cpu,
    bus: Bus,
    sdl: Sdl,
}

fn key_to_joy(keycode: Keycode) -> Option<Buttons> {
    match keycode {
        Keycode::Up => Some(Buttons::Up),
        Keycode::Down => Some(Buttons::Down),
        Keycode::Left => Some(Buttons::Left),
        Keycode::Right => Some(Buttons::Right),
        Keycode::W => Some(Buttons::Up),
        Keycode::A => Some(Buttons::Left),
        Keycode::S => Some(Buttons::Down),
        Keycode::D => Some(Buttons::Right),
        Keycode::J => Some(Buttons::A),
        Keycode::K => Some(Buttons::B),
        Keycode::U => Some(Buttons::Start),
        Keycode::I => Some(Buttons::Select),
        Keycode::Return => Some(Buttons::Start),
        Keycode::Space => Some(Buttons::Select),
        _ => None,
    }
}

impl GameBoy {
    pub fn new(bootrom: BootRom, cartridge: Cartridge) -> Self {
        let sdl = sdl2::init().expect("failed to initialize SDL");
        Self {
            cpu: Cpu::new(),
            bus: Bus::new(bootrom, cartridge, LCD::new(&sdl, 4)),
            sdl,
        }
    }

    pub fn run(&mut self) {
        let time = time::Instant::now();
        let mut event_pump = self.sdl.event_pump().unwrap();
        let mut elapsed = 0;
        'running: loop {
            let e = time.elapsed().as_nanos();
            for _ in 0..(e - elapsed) / M_CYCLE_NANOS {
                for event in event_pump.poll_iter() {
                    match event {
                        Event::Quit { .. } => break 'running,
                        Event::KeyDown {
                            keycode: Some(key), ..
                        } => {
                            if key == Keycode::Escape {
                                break 'running;
                            }
                            if let Some(button) = key_to_joy(key) {
                                self.bus.joypad.press(&mut self.cpu.interrupts, button);
                            }
                        }
                        Event::KeyUp {
                            keycode: Some(key), ..
                        } => {
                            if let Some(button) = key_to_joy(key) {
                                self.bus.joypad.release(&mut self.cpu.interrupts, button);
                            }
                        }
                        _ => {}
                    }
                }
                self.cpu.emulate_cycle(&mut self.bus);

                elapsed += M_CYCLE_NANOS;
            }
        }
    }
}
