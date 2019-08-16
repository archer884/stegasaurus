mod error;
mod masks;

use image::{png::PNGEncoder, GenericImageView, PNG};
use masks::Masks;
use std::io::{BufRead, Seek, Write};

pub type Result<T, E = error::Error> = std::result::Result<T, E>;

struct Length(u64);

impl From<usize> for Length {
    fn from(u: usize) -> Self {
        Length(u as u64)
    }
}

struct Byte(u8);

pub struct Message<'a> {
    content: &'a [u8],
}

impl<'a> Message<'a> {
    pub fn new(content: &'a [u8]) -> Self {
        Message { content }
    }
}

impl Message<'_> {
    pub fn store(&self, carrier: impl BufRead + Seek, write: impl Write) -> Result<()> {
        let carrier_image = image::load(carrier, PNG)?;
        let (width, height) = carrier_image.dimensions();
        let color_type = carrier_image.color();
        let destination_image = PNGEncoder::new(write);

        let mut carrier_stream = carrier_image.raw_pixels();

        self.write_len(&mut carrier_stream[0..32]);
        self.write_message(&mut carrier_stream[32..]);
        Ok(destination_image.encode(&carrier_stream, width, height, color_type)?)
    }

    fn write_len(&self, s: &mut [u8]) {
        let masks = Length::from(self.content.len()).masks();
        for (u, mask) in s.iter_mut().zip(masks) {
            *u &= mask;
        }
    }

    fn write_message(&self, s: &mut [u8]) {
        let masks = self
            .content
            .iter()
            .cloned()
            .map(Byte)
            .flat_map(|byte| byte.masks());
        for (u, mask) in s.iter_mut().zip(masks) {
            *u &= mask;
        }
    }
}
