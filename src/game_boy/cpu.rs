pub mod registers;

use std::thread;
use std::time::Duration;
use regex::Regex;
use registers::Registers;
use crate::game_boy::memory::Memory;

trait InstructionMatcher {
    fn is_match(&self, opcode: u8) -> bool;
}

impl InstructionMatcher for str {
    fn is_match(&self, instruction: u8) -> bool {
        let rx = Regex::new(self).unwrap();
        let formated_instruction = format!("{:08b}", instruction);
        
        rx.is_match(formated_instruction.as_str())
    }
}

pub struct CPU {
    reg: Registers,
    memory: Memory,
    instructions: Vec<u8>,

    ime: bool,
    set_ime_after_instruction: bool,
}

impl CPU {
    pub fn default() -> Self {
        CPU{reg: Registers::new(), memory: Memory::new(), instructions: vec![], ime: false, set_ime_after_instruction: false}
    }

    pub fn new(instructions: Vec<u8>) -> CPU {
        CPU{reg: Registers::new(), memory: Memory::new(), instructions, ime: false, set_ime_after_instruction: false}
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
    
    fn encode_r8(&mut self, r8: u8, value: u8) {
        match r8 {
            0 => self.reg.write_b(value),
            1 => self.reg.write_c(value),
            2 => self.reg.write_d(value),
            3 => self.reg.write_e(value),
            4 => self.reg.write_h(value),
            5 => self.reg.write_l(value),
            6 => self.memory.write(self.reg.read_hl(), value),
            7 => self.reg.write_a(value),
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
    
    fn encode_r16(&mut self, r16: u8, value: u16) {
        match r16 {
            0 => self.reg.write_bc(value),
            1 => self.reg.write_de(value),
            2 => self.reg.write_hl(value),
            3 => self.reg.write_sp(value),
            _ => panic!("Invalid register code: {r16}")
        };
    }

    fn resolve_condition(&self, cond: u8) -> bool {
        match cond {
            0 => self.reg.read_zero_flag() & self.reg.read_subtraction_flag(),
            1 => self.reg.read_zero_flag(),
            2 => self.reg.read_subtraction_flag() & self.reg.read_carry_flag(),
            3 => self.reg.read_carry_flag(),
            _ => panic!("Invalid condition {cond}"),
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

    fn and_a_r8(&mut self, r8: u8) {
        let register_value = self.decode_r8(r8);
        self.and_a(register_value);
    }

    fn and_a(&mut self, value: u8) {
        let and_result = self.reg.read_a() & value;

        self.reg.set_zero_flag(and_result == 0);
        self.reg.set_subtraction_flag(false);
        self.reg.set_half_carry_flag(true);
        self.reg.set_carry_flag(false);

        self.reg.write_a(and_result);
    }

    fn bit_u3_r8(&mut self, u3: i8, r8: u8) {
        let register_value = self.decode_r8(r8);

        self.reg.set_zero_flag(register_value & (1 << u3) == 0);
        self.reg.set_subtraction_flag(false);
        self.reg.set_half_carry_flag(true);
    }

    fn call_cc_n16(&mut self, cc:u8, n16: u16) {
        if self.resolve_condition(cc) {
            self.call_n16(n16);
        }
    }

    fn call_n16(&mut self, n16: u16) {
        todo!()
    }

    fn ccf(&mut self) {
        self.reg.set_subtraction_flag(false);
        self.reg.set_half_carry_flag(false);
        self.reg.set_carry_flag(!self.reg.read_carry_flag());
    }
    
    fn cp_a_r8(&mut self, r8: u8) {
        let register_value = self.decode_r8(r8);
        self.cp_a(register_value);
    }
    
    fn cp_a(&mut self, value: u8) {
        self.reg.set_zero_flag(self.reg.read_a() == value);
        self.reg.set_subtraction_flag(true);
        self.reg.set_half_carry_flag((value & 0xF) > (self.reg.read_a() & 0xF));
        self.reg.set_carry_flag(value > self.reg.read_a());
    }
    
    fn cpl(&mut self) {
        self.reg.write_a(!self.reg.read_a());
        
        self.reg.set_subtraction_flag(true);
        self.reg.set_half_carry_flag(true);
    }
    
    fn daa(&mut self) {
        if self.reg.read_subtraction_flag() {
            let mut adj = 0;
            if self.reg.read_half_carry_flag() {
                adj += 0x6;
            }
            if self.reg.read_carry_flag() {
                adj += 0x60;
            }
            // TODO subtract adj from A
        } else {
            let mut adj = 0;
            if self.reg.read_half_carry_flag() || (self.reg.read_a() & 0xF) > 0x9 {
                adj += 0x6;
            }
            if self.reg.read_carry_flag() || self.reg.read_a() > 0x99 {
                adj += 0x60;
            }
            // TODO add adj to A
        }
        
        self.reg.set_zero_flag(self.reg.read_a() == 0);
        self.reg.set_half_carry_flag(false);
        // TODO set carry flag depending on the operation
    }

    fn detect_bit_3_borrow(a: u8, b: u8) -> bool {
        (a & 0x0F) < (b & 0x0F)
    }

    fn dec_r8(&mut self, r8: u8) {
        let register_value = self.decode_r8(r8);

        let new_value = register_value.wrapping_sub(1);

        self.reg.set_zero_flag(new_value == 0);
        self.reg.set_subtraction_flag(true);
        self.reg.set_half_carry_flag(Self::detect_bit_3_borrow(register_value, 1));

        self.encode_r8(r8, new_value);
    }

    fn dec_r16(&mut self, r16: u8) {
        let register_value = self.decode_r16(r16);
        let new_value = register_value.wrapping_sub(1);

        self.encode_r16(r16, new_value);
    }
    
    fn di(&mut self) {
        self.ime = false;
    }
    
    fn ei(&mut self) {
        self.set_ime_after_instruction = true
    }
    
    fn halt(&mut self) {
        todo!()
    }

    fn detect_bit_3_overflow(a: u8, b: u8) -> bool {
        (a & 0x0F) + (b & 0x0F) & 0x10 != 0
    }

    fn inc_r8(&mut self, r8: u8) {
        let register_value = self.decode_r8(r8);

        let new_value = register_value.wrapping_add(1);

        self.reg.set_zero_flag(new_value == 0);
        self.reg.set_subtraction_flag(false);
        self.reg.set_half_carry_flag(Self::detect_bit_3_overflow(register_value, 1));

        self.encode_r8(r8, new_value);
    }

    fn inc_r16(&mut self, r16: u8) {
        let register_value = self.decode_r16(r16);
        let new_value = register_value.wrapping_add(1);

        self.encode_r16(r16, new_value);
    }

    fn jp_n16(&mut self, n16: u16) {
        self.reg.write_pc(n16);
    }

    fn jp_cc_n16(&mut self, cc: u8, n16: u16) {
        if self.resolve_condition(cc) {
            self.reg.write_pc(n16);
        }
    }

    fn jp_hl(&mut self) {
        self.reg.write_pc(self.reg.read_hl())
    }

    fn ld_r8_r8(&mut self, left: u8, right: u8) {
        let right_value = self.decode_r8(right);

        self.encode_r8(left, right_value);
    }

    fn ld_r8_n8(&mut self, r8: u8, n8: u8) {
        self.encode_r8(r8, n8);
    }
    
    fn ld_r16_n16(&mut self, r16: u8, n16: u16) {
        self.encode_r16(r16, n16);
    }

    fn ld_r16_a(&mut self, r16: u8) {
        let a = self.reg.read_a();
        self.encode_r16(r16, a as u16);
    }
    
    fn ld_n16_a(&mut self, n16: u16) {
        let a = self.reg.read_a();
        self.memory.write(n16, a);
    }

    fn fetch_instruction(&mut self) -> u8 {
        let instruction = self.instructions[self.reg.read_pc() as usize];
        self.cycle();
        self.reg.inc_pc();

        instruction
    }

    fn fetch_n16(&mut self) -> u16 {
        let higher_nimble = self.fetch_instruction();
        let lower_nimble = self.fetch_instruction();

        (lower_nimble as u16) << 8 | higher_nimble as u16
    }
    
    fn fetch_n8(&mut self) -> u8 {
        self.fetch_instruction()
    }

    fn cycle_times(&self, times: u32) {
        thread::sleep(Duration::new(0, 1000 * times));
    }

    fn cycle(&self) {
        self.cycle_times(1);
    }

    pub(crate) fn execute_next_instruction(&mut self) {
        let instruction = self.fetch_instruction();

        let block = instruction >> 6;

        println!("{:08b}", instruction);

        match block {
            0b00 => println!("block 0"),
            0b01 => println!("block 1"),
            0b10 => println!("block 2"),
            0b11 => println!("block 3"),
            _ => unreachable!(),
        }

        if "11000011".is_match(instruction) {
            let n16 = self.fetch_n16();
            self.jp_n16(n16);
            self.cycle();
        } else if "00...110".is_match(instruction) {
            let n8 = self.fetch_n8();
            let r8 = (instruction & 0b00111000) >> 3;
            self.ld_r8_n8(r8, n8);
            self.cycle();
        } else if "11101010".is_match(instruction) {
            let n16 = self.fetch_n16();
            self.ld_n16_a(n16);
            self.cycle();
        } else {
            panic!("invalid instruction {:08b}", instruction);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_negative_to_sp() {
        let mut cpu = CPU::default();
        
        cpu.add_sp(-5);
        assert_eq!(-5, cpu.reg.read_sp() as i16);
    }
    
    #[test]
    fn add_to_sp_overflow() {
        let mut cpu = CPU::default();
        cpu.reg.write_sp(u16::MAX);
        cpu.add_sp(1);
        assert_eq!(0, cpu.reg.read_sp());
        assert_eq!(true, cpu.reg.read_carry_flag(), "carry flag is set");
        assert_eq!(true, cpu.reg.read_half_carry_flag(), "half carry flag is set");
    }
    
    #[test]
    fn add_to_sp_set_carry_flag() {
        let mut cpu = CPU::default();
        cpu.reg.write_sp(0xFF);
        cpu.add_sp(1);
        assert_eq!(0xFF + 1, cpu.reg.read_sp());
        assert_eq!(true, cpu.reg.read_carry_flag(), "carry flag is set");
        assert_eq!(true, cpu.reg.read_half_carry_flag(), "half carry flag is set");
    }
    
    #[test]
    fn add_from_ram_to_a() {
        let mut cpu = CPU::default();

        cpu.memory.write(0x00, 17);
        cpu.adc_a_r8(6);
        assert_eq!(cpu.reg.read_a(), 17);

        cpu.memory.write(0x00, 12);
        cpu.adc_a_r8(6);

        assert_eq!(cpu.reg.read_a(), 29);
    }

    #[test]
    fn and_a() {
        let mut cpu = CPU::default();
        let a = 0b1011_1101;
        let v = 0b1011_1110;
        cpu.reg.write_a(a);
        cpu.and_a(v);

        assert_eq!(0b1011_1100, cpu.reg.read_a());
    }

    #[test]
    fn test_bit() {
        let mut cpu = CPU::default();
        cpu.reg.write_a(0b_0000_0001);

        cpu.bit_u3_r8(0, 7);
        assert!(!cpu.reg.read_zero_flag(), "zero flag is not set");

        cpu.bit_u3_r8(1, 7);
        assert!(cpu.reg.read_zero_flag(), "zero flag is set");
    }
    
    #[test]
    fn compare_a() {
        let mut cpu = CPU::default();
        cpu.reg.write_a(0b_0010_1101);
        
        cpu.cp_a(0b_0011_1111);
        assert!(cpu.reg.read_carry_flag(), "carry flag is set");
        assert!(cpu.reg.read_half_carry_flag(), "half carry flag is set");

        cpu.cp_a(0b_0000_1111);
        assert!(!cpu.reg.read_carry_flag(), "carry flag is not set");
        assert!(cpu.reg.read_half_carry_flag(), "half carry flag is set");
        
        cpu.cp_a(0b_0100_0011);
        assert!(cpu.reg.read_carry_flag(), "carry flag is set");
        assert!(!cpu.reg.read_half_carry_flag(), "half carry flag is not set");
        
        cpu.cp_a(0b_0000_0000);
        assert!(!cpu.reg.read_carry_flag(), "carry flag is not set");
        assert!(!cpu.reg.read_half_carry_flag(), "half carry flag is not set");
    }
    
    #[test]
    fn complement_a() {
        let mut cpu = CPU::default();
        cpu.reg.write_a(0b_1011_1010);
        cpu.cpl();
        
        assert_eq!(0b_0100_0101, cpu.reg.read_a(), "carry flag is set");
    }
    
    #[test]
    fn dec_a() {
        let mut cpu = CPU::default();
        cpu.reg.write_a(0b_1011_0000);
        
        cpu.dec_r8(7);

        assert_eq!(0b_1010_1111, cpu.reg.read_a());
        assert!(cpu.reg.read_half_carry_flag(), "half carry flag is set");

        cpu.dec_r8(7);
        assert_eq!(0b_1010_1110, cpu.reg.read_a());
        assert!(!cpu.reg.read_half_carry_flag(), "half carry flag is not set");
    }

    #[test]
    fn inc_a() {
        let mut cpu = CPU::default();
        cpu.reg.write_a(0b_1010_1111);

        cpu.inc_r8(7);

        assert_eq!(0b_1011_0000, cpu.reg.read_a());
        assert!(cpu.reg.read_half_carry_flag(), "half carry flag is set");

        cpu.inc_r8(7);
        assert_eq!(0b_1011_0001, cpu.reg.read_a());
        assert!(!cpu.reg.read_half_carry_flag(), "half carry flag is not set");
    }
}