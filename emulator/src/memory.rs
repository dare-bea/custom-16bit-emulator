use std::{
    fmt,
    io::{Cursor, Read, Seek, Write},
};

use utils::chained_io::ChainedIO;

pub type Ram = Cursor<[u8; 0x8000]>;

pub trait RomDevice: Read + Seek + Write + fmt::Debug {}

impl<T: Read + Seek + Write + fmt::Debug> RomDevice for T {}

pub type SimpleRom = Cursor<[u8; 0x8000]>;

pub type Mmu = ChainedIO<Ram, Box<dyn RomDevice>>;
