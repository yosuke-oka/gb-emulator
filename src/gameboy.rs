use sdl2::{self, event::Event, keyboard::Keycode, Sdl};

use std::time;

use crate::{bootrom::BootRom, cpu::Cpu, lcd::LCD, peripherals::Peripherals};
pub const CPU_CLOCK: u128 = 4_194_304;
pub const M_CYCLE_CLOCK: u128 = 4;
const M_CYCLE_NANOS: u128 = 1_000_000_000 / CPU_CLOCK * M_CYCLE_CLOCK;

pub struct GameBoy {
    cpu: Cpu,
    peripherals: Peripherals,
    lcd: LCD,
    sdl: Sdl,
}

impl GameBoy {
    pub fn new(bootrom: BootRom) -> Self {
        let sdl = sdl2::init().expect("failed to initialize SDL");
        Self {
            cpu: Cpu::new(),
            peripherals: Peripherals::new(bootrom),
            lcd: LCD::new(&sdl, 4),
            sdl,
        }
    }

    pub fn run(&mut self) {
        let time = time::Instant::now();
        let mut event_pump = self.sdl.event_pump().unwrap();
        let mut elapsed = 0;
        'running: loop {
            let e = time.elapsed().as_nanos();
            for _ in 0..((e - elapsed) / M_CYCLE_NANOS) {
                for event in event_pump.poll_iter() {
                    match event {
                        Event::Quit { .. } => break 'running,
                        Event::KeyDown {
                            keycode: Some(Keycode::Escape),
                            ..
                        } => break 'running,
                        _ => {}
                    }
                }
                self.cpu.emulate_cycle(&mut self.peripherals);
                if self.peripherals.ppu.emulate_cycle() {
                    self.lcd.draw(&self.peripherals.ppu.pixel_buffer());
                }

                elapsed += M_CYCLE_NANOS;
            }
        }
    }
}
