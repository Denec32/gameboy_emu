use std::fs;
use gameboy_emu::game_boy::GameBoy;
fn decode_ram_size(code: u8) -> String {
    match code { 
        0 => String::from("No RAM"),
        1 => String::from("Unused"),
        2 => String::from("1 bank"),
        3 => String::from("4 banks of 8 KiB each"),
        4 => String::from("16 banks of 8 KiB each"),
        5 => String::from("8 banks of 8 KiB each"),
        _ => panic!("invalid code") 
    }
}

fn main() {
    let content = fs::read("hello-world.gb").unwrap();
    
    println!("Cartridge type: {:0x}", content[0x147]);
    println!("ROM size: 32 KiB * {}", 1 << content[0x148]);
    println!("RAM size: {}", decode_ram_size(content[0x149]));

    let mut game_boy = GameBoy::new();
    game_boy.start(content);
}