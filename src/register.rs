#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
#[repr(u8)]
pub enum GeneralPurposeRegister {
    A,
    B,
    C,
    D,
}
