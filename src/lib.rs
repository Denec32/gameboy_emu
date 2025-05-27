struct Registers {
    /// Accumulator and Flags
    AF: u16,
    BC: u16,
    DE: u16,
    HL: u16,
    
    /// Stack pointer
    SP: u16,
    
    /// Program counter and pointer
    PC: u16,
}

impl Registers {
    const ZERO_FLAG_BITS: u16 = 0b1000_0000;
    const SUBTRACTION_FLAG_BITS: u16 = 0b0100_0000;
    const HALF_CARRY_FLAG_BITS: u16 = 0b0010_0000;
    const CARRY_FLAG_BITS: u16 = 0b0001_0000;
    
    const LOWER_BITS: u16 = 0b00000000_11111111;
    
    fn new() -> Registers {
        Registers{ AF: 0, BC: 0, DE: 0, HL: 0, SP: 0, PC: 0 }
    }

    fn write_af(&mut self, val: u16) { self.AF = val }
    fn write_bc(&mut self, val: u16) {self.BC = val }
    fn write_de(&mut self, val: u16) {self.DE = val }
    fn write_hl(&mut self, val: u16) {self.HL = val }
    fn write_sp(&mut self, val: u16) {self.SP = val }
    fn write_pc(&mut self, val: u16) {self.PC = val }

    fn read_af(&self) -> u16 { self.AF }
    fn read_bc(&self) -> u16 {self.BC}
    fn read_de(&self) -> u16 {self.DE}
    fn read_hl(&self) -> u16 {self.HL}
    fn read_sp(&self) -> u16 {self.SP}
    fn read_pc(&self) -> u16 {self.PC}

    fn read_a(&self) -> u8 { (self.AF >> 8) as u8 }
    fn read_b(&self) -> u8 { (self.BC >> 8) as u8 }
    fn read_d(&self) -> u8 { (self.DE >> 8) as u8 }
    fn read_h(&self) -> u8 { (self.HL >> 8) as u8 }

    fn read_c(&self) -> u8 { (self.BC & Self::LOWER_BITS) as u8 }
    fn read_e(&self) -> u8 { (self.DE & Self::LOWER_BITS) as u8 }
    fn read_l(&self) -> u8 { (self.HL & Self::LOWER_BITS) as u8 }

    fn read_zero_flag(&self) -> bool { (self.AF & Self::ZERO_FLAG_BITS ) != 0 }
    fn read_subtraction_flag(&self) -> bool { (self.AF & Self::SUBTRACTION_FLAG_BITS) != 0 }
    fn read_half_carry_flag(&self) -> bool { (self.AF & Self::HALF_CARRY_FLAG_BITS) != 0 }
    fn read_carry_flag(&self) -> bool { (self.AF & Self::CARRY_FLAG_BITS) != 0 }
}

#[cfg(test)]
mod tests {
    use crate::Registers;

    #[test]
    fn read_hi() {
        let register = Registers { AF: 0b11001100_00000000, BC: 0, DE: 0, HL: 0, SP: 0, PC: 0 };
        assert_eq!(register.read_a(), 0b11001100);
    }
    
    #[test]
    fn read_lo() {
        let register = Registers { AF: 0, BC: 0b00000000_11001100, DE: 0, HL: 0, SP: 0, PC: 0 };
        assert_eq!(register.read_c(), 0b11001100);
    }
}