#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
#[repr(u8)]
pub enum Register {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
}