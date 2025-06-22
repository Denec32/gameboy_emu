use cpu::CPU;
use memory::Memory;

pub mod cpu;
mod memory;

pub struct GameBoy {
    cpu: CPU,
}

impl Default for GameBoy {
    fn default() -> Self {
        Self::new()
    }
}

impl GameBoy {
    pub fn new() -> GameBoy {
        let mut cpu = CPU::new();
        GameBoy{ cpu }
    }

    pub fn start(&mut self, cartridge_rom: Vec<u8>) {
        if !self.is_nintendo_logo_correct(&cartridge_rom) {
            panic!("wrong logo")
        }
        self.print_nintendo_logo(&cartridge_rom);

        let header_checksum = cartridge_rom[0x14D];
        assert_eq!(header_checksum, Self::calculate_checksum(&cartridge_rom));

        let global_checksum = ((cartridge_rom[0x14E] as u16) << 8) + (cartridge_rom[0x14F] as u16);
        assert_eq!(global_checksum, Self::calculate_global_checksum(&cartridge_rom));
        
        self.cpu.load_cartridge(cartridge_rom);
        
        loop {
            self.cpu.execute_next_instruction();
        }
    }

    fn is_nintendo_logo_correct(&self, cartridge_rom: &[u8]) -> bool {
        const NINTENDO_LOGO: &[u8] = &[
            0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
            0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
            0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
        ];

        let nintendo_logo_content = &cartridge_rom[0x104..0x133 + 1];
        NINTENDO_LOGO == nintendo_logo_content
    }

    fn print_nintendo_logo(&self, cartridge_rom: &[u8]) {
        let nintendo_logo_content = &cartridge_rom[0x104..0x133 + 1];
        self.print_half(&nintendo_logo_content[0..24]);
        self.print_half(&nintendo_logo_content[24..48]);
    }

    fn print_half(&self, bytes: &[u8]) {
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

    fn calculate_checksum(content: &[u8]) -> u8 {
        let mut checksum: u8 = 0;

        for byte in content.iter().take(0x014C + 1).skip(0x0134) {
            checksum = checksum.wrapping_sub(byte + 1);
        }

        checksum
    }

    fn calculate_global_checksum(content: &[u8]) -> u16 {
        let mut checksum: u16 = 0;

        for (address, byte) in content.iter().enumerate() {
            if address == 0x014E || address == 0x014F {
                continue;
            }
            checksum = checksum.wrapping_add(*byte as u16);
        }

        checksum
    }
}