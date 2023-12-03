use std::{env, fs::File, io::Read};

use gameboy::GameBoy;

mod bootrom;
mod bus;
mod cartridge;
mod cpu;
mod gameboy;
mod hram;
mod lcd;
mod ppu;
mod timer;
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
    let cartridge = cartridge::Cartridge::new(file2vec(cartridge_file).into());
    let bootrom = bootrom::BootRom::new(file2vec("dmg_bootrom.bin").into());
    GameBoy::new(bootrom, cartridge).run();
}
