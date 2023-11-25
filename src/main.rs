use std::{env, fs::File, io::Read};

use gameboy::GameBoy;

mod bootrom;
mod cpu;
mod gameboy;
mod hram;
mod lcd;
mod peripherals;
mod ppu;
mod wram;

fn file2vec(fname: &str) -> Vec<u8> {
    println!("Loading {}...", fname);
    if let Ok(mut file) = File::open(fname) {
        let mut ret = vec![];
        file.read_to_end(&mut ret).unwrap();
        ret
    } else {
        panic!("Cannot open {}.", fname);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("no cartridge\nUsage: {} <cartridge file>", args[0]);
    };
    let cartridge_file = &args[1];
    let rom = file2vec("dmg_bootrom.bin");
    let bootrom = bootrom::BootRom::new(rom.into_boxed_slice());
    GameBoy::new(bootrom).run();
}
