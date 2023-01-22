pub fn get_bit_8b(bits: u8, index: u8) -> bool {
    return ((bits >> index) & 1) != 0;
}

pub fn set_bit_8b(bits: u8, index: u8, value: bool) -> u8 {
    if value {
        return bits | (1 << index);
    }
    else {
        return bits & !(1 << index);
    }
}

pub fn assign_bit_8b(bits: &mut u8, index: u8, value: bool) {
    if value {
        *bits |= 1 << index;
    }
    else {
        *bits &= !(1 << index);
    }
}

pub fn get_bit_16b(bits: u16, index: u8) -> bool {
    return ((bits >> index) & 1) == 1;
}

pub fn set_bit_16b(bits: u16, index: u8, value: bool) -> u16 {
    if value {
        return bits | (1 << index);
    }
    else {
        return bits & !(1 << index);
    }
}

pub fn assign_bit_16b(bits: &mut u16, index: u8, value: bool) {
    if value {
        *bits |= 1 << index;
    }
    else {
        *bits &= !(1 << index);
    }
}

// Make a 16-bit value out of two 8-bit values (the low and high bytes).
pub fn get_16b_from_hl(low: u8, high: u8) -> u16 {
    return low as u16 | (high as u16) << 8;
}

pub fn get_high(value: u16) -> u8 {
    return (value >> 8) as u8;
}

pub fn get_low(value: u16) -> u8 {
    return value as u8;
}