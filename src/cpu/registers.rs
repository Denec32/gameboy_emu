pub(crate) struct Registers {
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

    pub(crate) fn new() -> Registers {
        Registers{ a: 0, f: 0, b: 0, c: 0, d: 0, e: 0, h: 0, l: 0, sp: 0, pc: 0, }
    }

    fn merge_to_16_bit(first: u8, second: u8) -> u16 {
        ((first as u16) << 8) | second as u16
    }

    pub(crate) fn write_a(&mut self, val: u8) { self.a = val; }

    pub(crate) fn write_b(&mut self, val: u8) { self.b = val }
    pub(crate) fn write_c(&mut self, val: u8) { self.c = val }
    pub(crate) fn write_d(&mut self, val: u8) { self.d = val }
    pub(crate) fn write_e(&mut self, val: u8) { self.e = val }
    pub(crate) fn write_h(&mut self, val: u8) { self.h = val }
    pub(crate) fn write_l(&mut self, val: u8) { self.l = val }

    pub(crate) fn write_bc(&mut self, val: u16) {
        self.b = (val >> 8) as u8;
        self.c = val as u8;
    }
    pub(crate) fn write_de(&mut self, val: u16) {
        self.d = (val >> 8) as u8;
        self.e = val as u8;
    }
    
    pub(crate) fn write_hl(&mut self, val: u16) { 
        self.h = (val >> 8) as u8;
        self.l = val as u8;
    }

    pub(crate) fn write_sp(&mut self, value: u16) { self.sp = value; }
    
    pub(crate) fn read_a(&self) -> u8 { self.a }
    pub(crate) fn read_b(&self) -> u8 { self.b }
    pub(crate) fn read_c(&self) -> u8 { self.c }

    pub(crate) fn read_d(&self) -> u8 { self.d }
    pub(crate) fn read_e(&self) -> u8 { self.e }

    pub(crate) fn read_h(&self) -> u8 { self.h }
    pub(crate) fn read_l(&self) -> u8 { self.l }
    pub(crate) fn read_hl(&self) -> u16 { Self::merge_to_16_bit(self.h, self.l) }
    pub(crate) fn read_bc(&self) -> u16 { Self::merge_to_16_bit(self.b, self.c) }
    pub(crate) fn read_de(&self) -> u16 { Self::merge_to_16_bit(self.d, self.e) }
    pub(crate) fn read_sp(&self) -> u16 { self.sp }

    pub(crate) fn read_zero_flag(&self) -> bool { (self.f & Self::ZERO_FLAG_BITS ) != 0 }
    pub(crate) fn read_subtraction_flag(&self) -> bool { (self.f & Self::SUBTRACTION_FLAG_BITS) != 0 }
    pub(crate) fn read_half_carry_flag(&self) -> bool { (self.f & Self::HALF_CARRY_FLAG_BITS) != 0 }
    pub(crate) fn read_carry_flag(&self) -> bool { (self.f & Self::CARRY_FLAG_BITS) != 0 }

    pub(crate) fn set_zero_flag(&mut self, value: bool) { if value { self.f |= Self::ZERO_FLAG_BITS } else { self.f &= !Self::ZERO_FLAG_BITS }; }
    pub(crate) fn set_subtraction_flag(&mut self, value: bool) { if value { self.f |= Self::SUBTRACTION_FLAG_BITS } else { self.f &= !Self::SUBTRACTION_FLAG_BITS }; }
    pub(crate) fn set_half_carry_flag(&mut self, value: bool) { if value { self.f |= Self::HALF_CARRY_FLAG_BITS } else { self.f &= !Self::HALF_CARRY_FLAG_BITS }; }
    pub(crate) fn set_carry_flag(&mut self, value: bool) { if value { self.f |= Self::CARRY_FLAG_BITS } else { self.f &= !Self::CARRY_FLAG_BITS }; }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}