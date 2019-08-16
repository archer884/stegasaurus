use crate::{Byte, Length};

pub trait Masks {
    type Iter: Iterator<Item = u8>;
    fn masks(&self) -> Self::Iter;
}

impl Masks for Byte {
    type Iter = ByteMasks;

    fn masks(&self) -> Self::Iter {
        ByteMasks(0, self.0)
    }
}

impl Masks for Length {
    type Iter = LengthMasks;

    fn masks(&self) -> Self::Iter {
        LengthMasks(0, self.0)
    }
}

pub struct ByteMasks(u8, u8);

impl Iterator for ByteMasks {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.0, self.1) {
            (idx, x) if idx < 4 => {
                self.0 += 1;
                self.1 >>= 2;
                Some(0b0000_0011 & x)
            }

            _ => None,
        }
    }
}

pub struct LengthMasks(u8, u64);

impl Iterator for LengthMasks {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.0, self.1) {
            (idx, x) if idx < 32 => {
                self.0 += 1;
                self.1 >>= 2;
                Some(0b0000_0011 & x as u8)
            }

            _ => None,
        }
    }
}
