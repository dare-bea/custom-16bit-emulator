pub const ZERO: u8 = 0;
pub const SIGN: u8 = 1;
pub const CARRY: u8 = 2;
pub const OVERFLOW: u8 = 3;
pub const INTERRUPT: u8 = 6;
pub const HALT: u8 = 7;

pub fn set_flag(status: &mut u8, flag: u8, value: bool) {
    if value {
        *status |= 1 << flag;
    } else {
        *status &= !(1 << flag);
    }
}

pub fn get_flag(status: u8, flag: u8) -> bool {
    (status & (1 << flag)) != 0
}
