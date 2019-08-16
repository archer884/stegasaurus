use crate::{Byte, Length};

pub trait Masks {
    type Iter: Iterator<Item = u8>;
    fn masks(&self) -> Self::Iter;
}

impl Masks for Byte {
    type Iter = ByteMasks;

    fn masks(&self) -> Self::Iter {
        ByteMasks(self.0)
    }
}

impl Masks for Length {
    type Iter = LengthMasks;

    fn masks(&self) -> Self::Iter {
        LengthMasks(self.0)
    }
}

pub struct ByteMasks(u8);

impl Iterator for ByteMasks {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            0 => None,
            n => {
                self.0 >>= 2;
                Some((0b00000011 & n as u8) | 0b11111100)
            }
        }
    }
}

pub struct LengthMasks(u64);

impl Iterator for LengthMasks {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            0 => None,
            n => {
                self.0 >>= 2;
                Some((0b00000011 & n as u8) | 0b11111100)
            }
        }
    }
}
