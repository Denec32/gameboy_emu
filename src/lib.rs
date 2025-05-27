struct Registers {
    a: u8,
    f: u8,
    
    b: u8,
    c: u8,
    
    d: u8,
    e: u8,
    
    h: u8,
    l: u8,
    
    sp: u16,
    
    pc: u16,
}

impl Registers {
    const ZERO_FLAG_BITS: u8 = 0b1000_0000;
    const SUBTRACTION_FLAG_BITS: u8 = 0b0100_0000;
    const HALF_CARRY_FLAG_BITS: u8 = 0b0010_0000;
    const CARRY_FLAG_BITS: u8 = 0b0001_0000;
    
    const LOWER_BITS: u16 = 0b00000000_11111111;
    
    fn new() -> Registers {
        Registers{ a: 0, f: 0, b: 0, c: 0, d: 0, e: 0, h: 0, l: 0, sp: 0, pc: 0, }
    }

    fn write_a(&mut self, val: u8) { self.a = val; }
    
    fn read_a(&self) -> u8 { self.a }
    fn read_b(&self) -> u8 { self.b }
    fn read_c(&self) -> u8 { self.c }

    fn read_d(&self) -> u8 { self.d }
    fn read_e(&self) -> u8 { self.e }

    fn read_h(&self) -> u8 { self.h }
    fn read_l(&self) -> u8 { self.l }
    fn read_hl(&self) -> u16 { ((self.h as u16) << 8) | self.l as u16 }
    

    fn read_zero_flag(&self) -> bool { (self.f & Self::ZERO_FLAG_BITS ) != 0 }
    fn read_subtraction_flag(&self) -> bool { (self.f & Self::SUBTRACTION_FLAG_BITS) != 0 }
    fn read_half_carry_flag(&self) -> bool { (self.f & Self::HALF_CARRY_FLAG_BITS) != 0 }
    fn read_carry_flag(&self) -> bool { (self.f & Self::CARRY_FLAG_BITS) != 0 }
    
    fn set_zero_flag(&mut self, value: bool) { if value { self.f |= Self::ZERO_FLAG_BITS } else { self.f &= !Self::ZERO_FLAG_BITS }; }
    fn set_subtraction_flag(&mut self, value: bool) { if value { self.f |= Self::SUBTRACTION_FLAG_BITS } else { self.f &= !Self::SUBTRACTION_FLAG_BITS }; }
    fn set_half_carry_flag(&mut self, value: bool) { if value { self.f |= Self::HALF_CARRY_FLAG_BITS } else { self.f &= !Self::HALF_CARRY_FLAG_BITS }; }
    fn set_carry_flag(&mut self, value: bool) { if value { self.f |= Self::CARRY_FLAG_BITS } else { self.f &= !Self::CARRY_FLAG_BITS }; }

    fn read_hl_pointer(&self) -> u8 {
        todo!()
    }
}

struct Memory {
    memory: [u8; 0b_1111_1111_1111_1111],
}
impl Memory {
    fn new() -> Memory {
        Memory{memory: [0; 0b_1111_1111_1111_1111]}
    }
    
    fn read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }    
    
    fn write(&mut self, addr: u16, value: u8) {
        self.memory[addr as usize] = value;
    }
}

struct CPU {
    reg: Registers,
    memory: Memory,
}

impl CPU {
    fn new() -> CPU {
        CPU{reg: Registers::new(), memory: Memory::new()}
    }
    
    fn nop(&mut self) {}

    fn decode_r8_register(&self, r8: u8) -> u8 {
        match r8 {
            0 => self.reg.read_b(),
            1 => self.reg.read_c(),
            2 => self.reg.read_d(),
            3 => self.reg.read_e(),
            4 => self.reg.read_h(),
            5 => self.reg.read_l(),
            6 => self.memory.read(self.reg.read_hl()),
            7 => self.reg.read_a(),
            _ => panic!("Invalid register code: {r8}")
        }
    }
    
    /// add value in r8 plus the carry flag to register A
    fn adc_a_r8(&mut self, r8: u8) {
        let a = self.reg.read_a();
        let register_value = self.decode_r8_register(r8);
        let sum_result = a as u16 + register_value as u16 + if self.reg.read_carry_flag() { 1 } else { 0 };
        
        self.reg.set_zero_flag(sum_result == 0);
        self.reg.set_subtraction_flag(false);
        self.reg.set_half_carry_flag(sum_result > 0xF);
        self.reg.set_carry_flag(sum_result > 0xFF);
        
        self.reg.write_a(sum_result as u8);
    }
}

#[cfg(test)]
mod tests {
    use crate::{Registers, CPU};

    #[test]
    fn read_hi() {
        let register = Registers { a: 0b11001100, f: 0, b: 0, c: 0, d: 0, e: 0, h: 0, l: 0, sp: 0, pc: 0 };
        assert_eq!(register.read_a(), 0b11001100);
    }
    
    #[test]
    fn read_lo() {
        let register = Registers { a: 0, f: 0, b: 0, c: 0b11001100, d: 0, e: 0, h: 0, l: 0, sp: 0, pc: 0 };
        assert_eq!(register.read_c(), 0b11001100);
    }
    
    #[test]
    fn read_hl() {
        let register = Registers{ a: 0, f: 0, b: 0, c: 0, d: 0, e: 0, h: 0b10101010, l: 0b01010101, sp: 0, pc: 0, };
        assert_eq!(register.read_hl(), 0b10101010_01010101);
    }
    
    #[test]
    fn add_from_ram_to_a() {
        let mut cpu = CPU::new();
        
        cpu.memory.write(0x00, 17);
        cpu.adc_a_r8(6);
        assert_eq!(cpu.reg.read_a(), 17);
        
        cpu.memory.write(0x00, 12);
        cpu.adc_a_r8(6);

        assert_eq!(cpu.reg.read_a(), 29);
    }
}