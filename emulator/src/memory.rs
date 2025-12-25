use std::{
    fmt,
    io::{Cursor, Read, Result, Seek, Write},
    ops::{Deref, DerefMut},
};

use utils::chained_io::ChainedIO;

pub type Ram = Cursor<[u8; 0x8000]>;

pub trait RomDevice: Read + Seek + Write + fmt::Debug {}

impl<T: Read + Seek + Write + fmt::Debug> RomDevice for T {}

pub type SimpleRom = Cursor<[u8; 0x8000]>;

pub type MmuInner = ChainedIO<Ram, Box<dyn RomDevice>>;

#[derive(Debug)]
pub struct Mmu(MmuInner);

impl Mmu {
    pub fn new(first: Cursor<[u8; 32768]>, second: Box<dyn RomDevice>) -> Result<Self> {
        Ok(Self(MmuInner::new(first, second)?))
    }

    pub fn read_byte(&mut self, pos: u16) -> Result<u8> {
        let mut buf = [0u8; 1];
        self.0.seek(std::io::SeekFrom::Start(pos as u64))?;
        self.0.read_exact(&mut buf)?;
        Ok(u8::from_le_bytes(buf))
    }

    pub fn read_word(&mut self, pos: u16) -> Result<u16> {
        Ok(u16::from_le_bytes([
            self.read_byte(pos)?,
            self.read_byte(pos.wrapping_add(1))?,
        ]))
    }
}

impl Deref for Mmu {
    type Target = MmuInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Mmu {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
