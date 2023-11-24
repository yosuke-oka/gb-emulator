use gameboy::GameBoy;

mod bootrom;
mod cpu;
mod gameboy;
mod hram;
mod lcd;
mod peripherals;
mod ppu;
mod wram;

fn main() {
    let rom = Box::new([0; 0x100]);
    let bootrom = bootrom::BootRom::new(rom);
    GameBoy::new(bootrom).run();
}
