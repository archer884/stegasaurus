use std::slice;

pub trait Masks {
    type Iter: Iterator<Item = u8>;
    fn masks(&self) -> Self::Iter;
}

impl Masks for usize {
    type Iter = U64MaskIter;

    fn masks(&self) -> Self::Iter {
        U64MaskIter(0, *self as u64)
    }
}

impl<'a> Masks for &'a [u8] {
    type Iter = ByteSliceMaskIter<'a>;

    fn masks(&self) -> Self::Iter {
        ByteSliceMaskIter {
            stage: 5,
            current: 0,
            source: self.iter(),
        }
    }
}

pub struct ByteSliceMaskIter<'a> {
    stage: u8,
    current: u8,
    source: slice::Iter<'a, u8>,
}

impl Iterator for ByteSliceMaskIter<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.stage, self.current) {
            (stage, x) if stage < 4 => {
                self.stage += 1;
                self.current >>= 2;
                Some(0b0000_0011 & x as u8)
            }

            (_, _) => {
                self.stage = 0;
                self.current = *self.source.next()?;
                self.next()
            }
        }
    }
}

pub struct U64MaskIter(u8, u64);

impl Iterator for U64MaskIter {
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
