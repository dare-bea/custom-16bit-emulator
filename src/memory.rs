pub trait Memory {
    fn read(&self, address: usize) -> u8;
    fn write(&mut self, address: usize, value: u8);

    fn read_word(&self, address: usize) -> u16 {
        let low = self.read(address) as u16;
        let high = self.read(address + 1) as u16;
        (high << 8) | low
    }

    fn write_word(&mut self, address: usize, value: u16) {
        self.write(address, value as u8);
        self.write(address + 1, (value >> 8) as u8);
    }

    fn load(&mut self, address: usize, data: &[u8]) {
        for (i, &byte) in data.iter().enumerate() {
            self.write(address + i, byte);
        }
    }

    fn dump(&self, address: usize, length: usize) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(length);
        for i in 0..length {
            buffer.push(self.read(address + i));
        }
        buffer
    }

    fn iter(&self, start: usize) -> impl Iterator<Item = u8> {
        (start..).map(move |addr| self.read(addr))
    }
}
