use std::ops::{BitAnd, BitOr, BitXor, Not};

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum MyEmulatorInstruction {
    #[default]
    NoOperation,
    StoreB,
    StoreC,
    StoreD,

    LoadB = Self::StoreD as u8 + 2,
    LoadC,
    LoadD,

    ZeroA,
    ZeroB,
    ZeroC,
    ZeroD,

    LoadImmediate,
    LoadDirectAddress,
    LoadIndirectAddress,

    AndB,
    OrB,
    XorB,
    AddB,
    SubtractB,

    Jump,
    JumpIfZero,
    JumpIfNotZero,
    JumpIfSign,
    JumpIfNotSign,
    JumpIfCarry,
    JumpIfNotCarry,
    JumpIfOverflow,
    JumpIfNotOverflow,

    Call,
    Return,

    In,
    Out,

    Halt,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
#[repr(transparent)]
pub struct Flags(u16);

impl Flags {
    pub const NONE: Self = Self(0);
    pub const ALL: Self = Self(u16::MAX);

    pub const ZERO: Self = Self(1 << 0);
    pub const SIGN: Self = Self(1 << 1);
    pub const CARRY: Self = Self(1 << 2);
    pub const OVERFLOW: Self = Self(1 << 3);
    pub const HALT: Self = Self(1 << 15);

    pub fn set(self, rhs: Self) -> Self {
        self | rhs
    }

    pub fn cleared(self, rhs: Self) -> Self {
        self & !rhs
    }

    pub fn is_set(self, rhs: Self) -> bool {
        self & rhs != Self::NONE
    }

    pub fn is_clear(self, rhs: Self) -> bool {
        self & !rhs != todo!()
    }
}

impl BitAnd for Flags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitOr for Flags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitXor for Flags {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl Not for Flags {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

const MEM_SIZE: usize = 0x10000;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct MyEmulator {
    /// Accumulator (operations)
    a: u16,
    /// Base (addresses)
    b: u16,
    /// Counter (loops)
    c: u16,
    /// Data (ports)
    d: u16,
    /// Program Counter
    pc: u16,
    /// Stack Pointer
    sp: u16,
    /// Program Flags
    flags: Flags,
    /// Program Memory
    memory: [u8; MEM_SIZE],
}

impl MyEmulator {
    pub fn new<IterableBytes: IntoIterator<Item = u8>>(memory: IterableBytes) -> Self {
        Self {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            pc: 0,
            sp: 0,
            flags: Flags(0),
            memory: memory
                .into_iter()
                .chain(std::iter::repeat(0))
                .take(MEM_SIZE)
                .collect::<Vec<_>>()
                .try_into()
                .expect("Should have exactly MEM_SIZE elements"),
        }
    }

    pub fn register_a(&self) -> u16 {
        self.a
    }
    pub fn register_pc(&self) -> u16 {
        self.pc
    }
    pub fn memory(&self) -> &[u8] {
        &self.memory
    }

    fn advance(&mut self) -> () {

        match self.memory[{
            self.pc += 1;
            self.pc
        } as usize]
        {
            0x00 => {},
            opcode => unimplemented!("Unknown opcode 0x{opcode:02X}"),
        }
    }
}

fn main() {
    let mut emu = MyEmulator::new([

    ]);
    println!("{:?}", emu.advance());
}
