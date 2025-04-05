/// Implemented for `u8` and `u16` only.
/// 
/// This trait provides convenience methods to get, set, and assign bits in a
/// number. It is not mean to be used directly, but rather through the provided
/// generic functions `get_bit`, `set_bit`, and `assign_bit`.
pub trait BitwiseOps {
    fn get_bit(bits: Self, index: usize) -> bool;
    fn set_bit(bits: Self, index: usize, value: bool) -> Self;
    fn assign_bit(bits: &mut Self, index: usize, value: bool);
}

macro_rules! impl_bitwise_ops {
    ($t:ty) => {
        impl BitwiseOps for $t {
            fn get_bit(bits: Self, index: usize) -> bool {
                (bits >> index) & 1 != 0
            }
            
            fn set_bit(bits: Self, index: usize, value: bool) -> Self {
                if value {
                    bits | (1 << index)
                } else {
                    bits & !(1 << index)
                }
            }
            
            fn assign_bit(bits: &mut Self, index: usize, value: bool) {
                if value {
                    *bits |= 1 << index;
                } else {
                    *bits &= !(1 << index);
                }
            }
        }
    };
}

impl_bitwise_ops!(u8);
impl_bitwise_ops!(u16);

/// Get the value of a bit at a given index in a number.
pub fn get_bit<T: BitwiseOps>(bits: T, index: usize) -> bool {
    T::get_bit(bits, index)
}

/// Set the value of a bit at a given index in a number, and return the new
/// value.
pub fn set_bit<T: BitwiseOps>(bits: T, index: usize, value: bool) -> T {
    T::set_bit(bits, index, value)
}

/// Assign a value to a bit at a given index in a number. This function
/// modifies the number in place.
pub fn assign_bit<T: BitwiseOps>(bits: &mut T, index: usize, value: bool) {
    T::assign_bit(bits, index, value);
}

/// Make a 16-bit value out of two 8-bit values (the low and high bytes).
pub fn hl_to_16b(low: u8, high: u8) -> u16 {
    u16::from_le_bytes([low, high])
}

/// Get the high byte of a 16-bit value.
pub fn get_high(value: u16) -> u8 {
    value.to_le_bytes()[1]
}

/// Get the low byte of a 16-bit value.
pub fn get_low(value: u16) -> u8 {
    value.to_le_bytes()[0]
}
