use sdl2::{self, event::Event, keyboard::Keycode, Sdl};

use std::time;

use crate::{bootrom::BootRom, bus::Bus, cartridge::Cartridge, cpu::Cpu, lcd::LCD};

pub const CPU_CLOCK_HZ: u128 = 4_194_304;
pub const M_CYCLE_CLOCK: u128 = 4;
const M_CYCLE_NANOS: u128 = M_CYCLE_CLOCK * 1_000_000_000 / CPU_CLOCK_HZ;

pub struct GameBoy {
    cpu: Cpu,
    bus: Bus,
    sdl: Sdl,
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
                            keycode: Some(Keycode::Escape),
                            ..
                        } => break 'running,
                        _ => {}
                    }
                }
                let elapsed_m_cycle = self.cpu.emulate_cycle(&mut self.bus);
                self.bus
                    .timer
                    .emulate_cycle(elapsed_m_cycle, &mut self.cpu.interrupts);
                if let Some(addr) = self.bus.ppu.oam_dma {
                    // TODO: 実装があっているか不明かつ、ppuに処理を移動したい
                    for i in 0..0xA0 {
                        let data = self.bus.read(&self.cpu.interrupts, addr + i);
                        self.bus.ppu.write_oam(0xFE00 + i, data);
                    }
                    self.bus.ppu.finish_oam_dma();
                }

                elapsed += M_CYCLE_NANOS;
            }
        }
    }
}
