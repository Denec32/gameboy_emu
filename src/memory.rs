pub(crate) struct Memory {
    memory: [u8; 0b_1111_1111_1111_1111],
}
impl Memory {
    pub(crate) fn new() -> Memory {
        Memory{memory: [0; 0b_1111_1111_1111_1111]}
    }

    pub(crate) fn read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub(crate) fn write(&mut self, addr: u16, value: u8) {
        self.memory[addr as usize] = value;
    }
}