use crate::offset::Offset;

pub struct GregorianDateTime {
    year: i32,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
    // decimal fraction of second?
    offset: Offset,
}

impl GregorianDateTime {}
