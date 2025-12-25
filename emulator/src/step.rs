use super::Emulator;
use std::io;

impl Emulator {
    pub fn step(&mut self) -> io::Result<()> {
        match self.next_byte()? {
            0x00 => {
                // LD addr
                let addr = self.next_word()?;
                self.cpu.a = self.memory.read_byte(addr)? as u16;
            }

            op => unimplemented!("Opcode ${op:X} is undefined."),
        };
        Ok(())
    }
}
