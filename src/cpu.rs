pub mod registers;

use registers::Registers;
use crate::memory::Memory;

pub struct CPU {
    reg: Registers,
    memory: Memory,
}

impl CPU {
    pub fn new() -> CPU {
        CPU{reg: Registers::new(), memory: Memory::new()}
    }

    fn nop(&mut self) {}

    fn decode_r8(&self, r8: u8) -> u8 {
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

    fn decode_r16(&self, r16: u8) -> u16 {
        match r16 {
            0 => self.reg.read_bc(),
            1 => self.reg.read_de(),
            2 => self.reg.read_hl(),
            3 => self.reg.read_sp(),
            _ => panic!("Invalid register code: {r16}")
        }
    }

    /// add value in r8 plus the carry flag to register A
    fn adc_a_r8(&mut self, r8: u8) {
        let register_value = self.decode_r8(r8);
        self.adc_a(register_value);
    }

    /// add byte plus carry flag to register A
    fn adc_a(&mut self, value: u8) {
        let a = self.reg.read_a();
        let sum_result = a as u16 + value as u16 + if self.reg.read_carry_flag() { 1 } else { 0 };

        self.reg.set_zero_flag(sum_result == 0);
        self.reg.set_subtraction_flag(false);
        self.reg.set_half_carry_flag(sum_result > 0xF);
        self.reg.set_carry_flag(sum_result > 0xFF);

        self.reg.write_a(sum_result as u8);
    }
    
    fn add_a_r8(&mut self, r8: u8) {
        let register_value = self.decode_r8(r8);
        self.add_a(register_value); 
    }

    fn add_a(&mut self, value: u8) {
        let a = self.reg.read_a();
        let sum_result = a as u16 + value as u16;

        self.reg.set_zero_flag(sum_result == 0);
        self.reg.set_subtraction_flag(false);
        self.reg.set_half_carry_flag(sum_result > 0xF);
        self.reg.set_carry_flag(sum_result > 0xFF);

        self.reg.write_a(sum_result as u8);
    }
    
    fn add_hl_r16(&mut self, r16: u8) {
        let register_value = self.decode_r16(r16);
        self.add_hl(register_value);
    }
    
    fn add_hl(&mut self, value: u16) {
        let hl = self.reg.read_hl();

        let sum_result = hl as u32 + value as u32;

        self.reg.set_subtraction_flag(false);
        self.reg.set_half_carry_flag(sum_result > 0xFFF);
        self.reg.set_carry_flag(sum_result > 0xFFFF);
        
        self.reg.write_hl(sum_result as u16);
    }
    
    fn add_sp(&mut self, e8: i8) {
        let sp = self.reg.read_sp();
        let (sum_result, has_overflow) = sp.overflowing_add(e8 as u16);

        self.reg.set_zero_flag(false);
        self.reg.set_subtraction_flag(false);
        self.reg.set_half_carry_flag(sum_result > 0xF || has_overflow);
        self.reg.set_carry_flag(sum_result > 0xFF || has_overflow);
        
        self.reg.write_sp(sum_result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_negative_to_sp() {
        let mut cpu = CPU::new();
        
        cpu.add_sp(-5);
        assert_eq!(-5, cpu.reg.read_sp() as i16);
    }
    
    #[test]
    fn add_to_sp_overflow() {
        let mut cpu = CPU::new();
        cpu.reg.write_sp(u16::MAX);
        cpu.add_sp(1);
        assert_eq!(0, cpu.reg.read_sp());
        assert_eq!(true, cpu.reg.read_carry_flag(), "carry flag is set");
        assert_eq!(true, cpu.reg.read_half_carry_flag(), "half carry flag is set");
    }
    
    #[test]
    fn add_to_sp_set_carry_flag() {
        let mut cpu = CPU::new();
        cpu.reg.write_sp(0xFF);
        cpu.add_sp(1);
        assert_eq!(0xFF + 1, cpu.reg.read_sp());
        assert_eq!(true, cpu.reg.read_carry_flag(), "carry flag is set");
        assert_eq!(true, cpu.reg.read_half_carry_flag(), "half carry flag is set");
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