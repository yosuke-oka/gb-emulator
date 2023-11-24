use std::{
    fs::File,
    io::{BufReader, Read},
};

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
    let rom = file2vec("dmg_bootrom.bin");
    //let mut reader = BufReader::with_capacity(8, file);
    //let rom: Box<[u8]> = reader.fill_buf().unwrap().into();
    let bootrom = bootrom::BootRom::new(rom.into_boxed_slice());
    GameBoy::new(bootrom).run();
}
