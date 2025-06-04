use gameboy_emu::cpu::CPU;
use std::fs;

fn main() {
    let cpu = CPU::new();

    let nintendo_logo = vec![
        0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
        0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
        0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
    ];

    let content = fs::read("hello-world.gb").unwrap();

    for pos in 0x104..0x133 {
        if content[pos] != nintendo_logo[pos - 0x104] {
            panic!("wrong logo")
        }
            //println!("{}", format!("{:08b}", content[pos]));
    }

    println!("correct logo");

    for pos in (0x104..0x011B).step_by(2) {
        let first_byte = format!("{:08b}", content[pos]);
        let second_byte = format!("{:08b}", content[pos]);
        let first_high_nimble = &first_byte[0..3];
        let first_low_nimble = &first_byte[4..7];
        
        let second_high_nimble = &second_byte[0..3];
        let second_low_nimble = &second_byte[4..7];
        
        println!("{first_high_nimble}\n{first_low_nimble}\n{second_high_nimble}\n{second_low_nimble}");
        println!();
    }
}