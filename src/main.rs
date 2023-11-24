mod bootrom;
mod cpu;
mod hram;
mod lcd;
mod peripherals;
mod ppu;
mod wram;

fn main() {
    let rom = Box::new([0; 0x100]);
    bootrom::BootRom::new(rom);
    println!("Hello, world!");
}
