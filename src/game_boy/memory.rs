use crate::game_boy::memory::vram_tile_data::VramTileData;

pub mod object_attribute_memory;
mod vram_tile_data;

pub(crate) struct Memory {
    rom_bank_00: [u8; 16 * 1024],
    rom_bank_01: [u8; 16 * 1024],

    vram_tile_data_block0: VramTileData,
    vram_tile_data_block1: VramTileData,
    vram_tile_data_block2: VramTileData,

    video_ram: [u8; 8 * 1024],



    external_ram: [u8; 8 * 1024],
    work_ram_00: [u8; 4 * 1024],
    work_ram_01: [u8; 4 * 1024],
    object_attribute_memory: [u8; 160],
    input_output_registers: [u8; 128],
    high_ram: [u8; 128],
    interrupt_enable_register: u8,
}
impl Memory {
    pub(crate) fn new() -> Memory {
        Memory {
            rom_bank_00: [0; 16 * 1024],
            rom_bank_01: [0; 16 * 1024],
            vram_tile_data_block0: VramTileData::new(),
            vram_tile_data_block1: VramTileData::new(),
            vram_tile_data_block2: VramTileData::new(),
            video_ram: [0; 8 * 1024],
            external_ram: [0; 8 * 1024],
            work_ram_00: [0; 4 * 1024],
            work_ram_01: [0; 4 * 1024],
            object_attribute_memory: [0; 160],
            input_output_registers: [0; 128],
            high_ram: [0; 128],
            interrupt_enable_register: 0,
        }
    }

    pub (crate) fn load_cartridge(&mut self, cartridge_rom: Vec<u8>) {
        assert_eq!(cartridge_rom.len(), 32 * 1024);

        self.rom_bank_00.copy_from_slice(&cartridge_rom[0..16 * 1024]);
        self.rom_bank_01.copy_from_slice(&cartridge_rom[16 * 1024..32 * 1024]);
    }

    pub(crate) fn read_video_ram(&self, address: u16) -> u8 {

        match address {
            0x8000 ..= 0x87FF => self.vram_tile_data_block0.read(address - 0x8000),
            0x8800 ..= 0x8FFF => self.vram_tile_data_block1.read(address - 0x8800),
            0x9000 ..= 0x97FF => self.vram_tile_data_block2.read(address - 0x9000),

            _ => unimplemented!()
        }
    }

    pub(crate) fn read(&self, address: u16) -> u8 {
        match address {
            0x0000 ..= 0x3FFF => self.rom_bank_00[address as usize],
            0x4000 ..= 0x7FFF => self.rom_bank_01[(address - 0x4000) as usize],
            0x8000 ..= 0x9FFF => self.read_video_ram(address),  //self.video_ram[(address - 0x8000) as usize],
            0xA000 ..= 0xBFFF => self.external_ram[(address - 0xA000) as usize],
            0xC000 ..= 0xCFFF => self.work_ram_00[(address - 0xC000) as usize],
            0xD000 ..= 0xDFFF => self.work_ram_01[(address - 0xD000) as usize],
            0xE000 ..= 0xFDFF => panic!("Echo RAM, prohibited"),
            0xFE00 ..= 0xFE9F => self.object_attribute_memory[(address - 0xFE00) as usize],
            0xFEA0 ..= 0xFEFF => panic!("Not Usable, prohibited"),
            0xFF00 ..= 0xFF7F => {
                let local_address = (address - 0xFF00) as usize;
                self.input_output_registers[local_address]
            },
            0xFF80 ..= 0xFFFE => {
                let local_address = (address - 0xFF80) as usize;
                self.high_ram[local_address]
            },
            0xFFFF => self.interrupt_enable_register,
            _ => unreachable!(),
        }
    }

    pub(crate) fn write(&mut self, address: u16, value: u8) {
        match address {
            0x0000 ..= 0x3FFF => panic!("Cartridge ROM is readonly!"),
            0x4000 ..= 0x7FFF => panic!("Cartridge ROM is readonly!"),
            0x8000 ..= 0x9FFF => self.video_ram[(address - 0x8000) as usize] = value,
            0xA000 ..= 0xBFFF => self.external_ram[(address - 0xA000) as usize] = value,
            0xC000 ..= 0xCFFF => self.work_ram_00[(address - 0xC000) as usize] = value,
            0xD000 ..= 0xDFFF => self.work_ram_01[(address - 0xD000) as usize] = value,
            0xE000 ..= 0xFDFF => panic!("Echo RAM, prohibited"),
            0xFE00 ..= 0xFE9F => self.object_attribute_memory[(address - 0xFE00) as usize] = value,
            0xFEA0 ..= 0xFEFF => panic!("Not Usable, prohibited"),
            0xFF00 ..= 0xFF7F => self.input_output_registers[(address - 0xFF00) as usize] = value,
            0xFF80 ..= 0xFFFE => self.high_ram[(address - 0xFF80) as usize] = value,
            0xFFFF => self.interrupt_enable_register = value,
            _ => unreachable!(),
        };
    }
}