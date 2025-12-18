
/// Zero flag is set. Equivalent to `[condition::EQUAL]`.
pub const ZERO: u8 = 0;
/// Zero flag is set. Equivalent to `[condition::ZERO]`.
pub const EQUAL: u8 = 0;
/// Sign flag is set.
pub const SIGN: u8 = 1;
/// Carry flag is set. Equivalent to `[condition::BELOW]`, `[condition::NOT_ABOVE_EQUAL]`.
pub const CARRY: u8 = 2;
/// Carry flag is set. Equivalent to `[condition::CARRY]`, `[condition::NOT_ABOVE_EQUAL]`.
pub const BELOW: u8 = 2;
/// Carry flag is set. Equivalent to `[condition::CARRY]`, `[condition::BELOW]`.
pub const NOT_ABOVE_EQUAL: u8 = 2;
/// Overflow flag is set.
pub const OVERFLOW: u8 = 3;
/// Carry or zero flag is set. Equivalent to `[condition::NOT_ABOVE]`.
pub const BELOW_EQUAL: u8 = 5;
/// Carry or zero flag is set. Equivalent to `[condition::BELOW_EQUAL]`.
pub const NOT_ABOVE: u8 = 5;
/// Zero flag is set or sign flag is not equal to overflow flag. Equivalent to `[condition::NOT_GREATER]`.
pub const LESS_EQUAL: u8 = 6;
/// Zero flag is set or sign flag is not equal to overflow flag. Equivalent to `[condition::LESS_EQUAL]`.
pub const NOT_GREATER: u8 = 6;
/// Sign flag is not equal to overflow flag. Equivalent to `[condition::NOT_GREATER_EQUAL]`.
pub const LESS: u8 = 7;
/// Sign flag is not equal to overflow flag. Equivalent to `[condition::LESS]`.
pub const NOT_GREATER_EQUAL: u8 = 7;
/// Zero flag is clear. Equivalent to `[condition::NOT_EQUAL]`.
pub const NOT_ZERO: u8 = 8;
/// Zero flag is clear. Equivalent to `[condition::NOT_ZERO]`.
pub const NOT_EQUAL: u8 = 8;
/// Sign flag is clear.
pub const NOT_SIGN: u8 = 9;
/// Carry flag is clear. Equivalent to `[condition::ABOVE]`, `[condition::NOT_BELOW_EQUAL]`.
pub const NOT_CARRY: u8 = 10;
/// Carry flag is clear. Equivalent to `[condition::NOT_CARRY]`, `[condition::NOT_BELOW_EQUAL]`.
pub const ABOVE: u8 = 10;
/// Carry flag is clear. Equivalent to `[condition::NOT_CARRY]`, `[condition::ABOVE]`.
pub const NOT_BELOW_EQUAL: u8 = 10;
/// Overflow flag is clear.
pub const NOT_OVERFLOW: u8 = 11;
/// Carry and zero flags are clear. Equivalent to `[condition::ABOVE_EQUAL]`.
pub const NOT_BELOW: u8 = 13;
/// Carry and zero flags are clear. Equivalent to `[condition::NOT_BELOW]`.
pub const ABOVE_EQUAL: u8 = 13;
/// Zero flag is clear or sign flag is equal to overflow flag. Equivalent to `[condition::GREATER]`.
pub const NOT_LESS_EQUAL: u8 = 14;
/// Zero flag is clear or sign flag is equal to overflow flag. Equivalent to `[condition::NOT_LESS_EQUAL]`.
pub const GREATER: u8 = 14;
/// Sign flag is equal to overflow flag. Equivalent to `[condition::GREATER_EQUAL]`.
pub const NOT_LESS: u8 = 15;
/// Sign flag is equal to overflow flag. Equivalent to `[condition::NOT_LESS]`.
pub const GREATER_EQUAL: u8 = 15;
