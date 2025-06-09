use std::fs;

static NINTENDO_LOGO: &'static [u8] = &[
    0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
    0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
    0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
];

fn is_nintendo_logo(bytes: &[u8]) -> bool {
    bytes == NINTENDO_LOGO
}

fn print_nintendo_logo(bytes: &[u8]) {
    print_half(&bytes[0..24]);
    print_half(&bytes[24..48]);
}

fn print_half(bytes: &[u8]) {
    assert_eq!(bytes.len(), 24);
    for nimble_idx in 0..4 {
        for byte_pair in bytes.chunks(2) {
            let first_byte = format!("{:08b}", byte_pair[0]);
            let second_byte = format!("{:08b}", byte_pair[1]);
            let nimble = match nimble_idx {
                0 => &first_byte[0..4],
                1 => &first_byte[4..8],
                2 => &second_byte[0..4],
                3 => &second_byte[4..8],
                _ => panic!("invalid nimble"),
            };
            for bit in nimble.chars() {
                print!("{}", if bit == '1' { '▮' } else { ' ' });
            }
        }
        println!();
    }
}

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

fn calculate_checksum(content: &[u8]) -> u8 {
    let mut checksum: u8 = 0;
    
    for address in 0x0134..0x014C + 1 {
        checksum = checksum.wrapping_sub(content[address] + 1);
    }
    
    checksum
}

fn calculate_global_checksum(content: &[u8]) -> u16 {
    let mut checksum: u16 = 0;
    
    for address in 0..content.len() {
        if address == 0x014E || address == 0x014F {
            continue;
        }
        checksum = checksum.wrapping_add(content[address] as u16);
    }
    
    checksum
}

fn main() {
    let content = fs::read("hello-world.gb").unwrap();
    
    let nintendo_logo_content = &content[0x104..0x133 + 1];
    
    if !is_nintendo_logo(nintendo_logo_content) {
        panic!("wrong logo")
    }

    print_nintendo_logo(nintendo_logo_content);
    
    println!("Cartridge type: {}", format!("{:0x}", content[0x147]));
    println!("ROM size: 32 KiB * {}", 1 << content[0x148]);
    println!("RAM size: {}", decode_ram_size(content[0x149]));
    
    let header_checksum = content[0x14D];
    assert_eq!(header_checksum, calculate_checksum(&content));
    
    let global_checksum = ((content[0x14E] as u16) << 8) + (content[0x14F] as u16);
    assert_eq!(global_checksum, calculate_global_checksum(&content));
}