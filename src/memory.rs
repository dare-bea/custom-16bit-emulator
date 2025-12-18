pub trait Memory {
    fn size(&self) -> usize;

    fn read_byte(&self, address: usize) -> u8;
    fn read_word(&self, address: usize) -> u16;
    fn write_byte(&mut self, address: usize, value: u8);
    fn write_word(&mut self, address: usize, value: u16);

    fn read_array<const N: usize>(&self, address: usize) -> [u8; N] {
        let mut result = [0; N];
        for i in 0..N {
            result[i] = self.read_byte(address.wrapping_add(i));
        }
        result
    }
    fn write_array(&mut self, address: usize, bytes: &[u8]) {
        for i in 0..bytes.len() {
            self.write_byte(address.wrapping_add(i), bytes[i]);
        }
    }
}

impl Memory for [u8] {
    fn size(&self) -> usize {self.len()}

    fn read_byte(&self, address: usize) -> u8 {self[address]}
    fn read_word(&self, address: usize) -> u16 {u16::from_le_bytes([self.read_byte(address), self.read_byte(address + 1)])}
    fn write_byte(&mut self, address: usize, value: u8) {self[address] = value}
    fn write_word(&mut self, address: usize, value: u16) {self.write_byte(address, value as u8); self.write_byte(address + 1, (value >> 8) as u8)}
}