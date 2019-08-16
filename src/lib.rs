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
            *u >>= 2;
            *u <<= 2;
            *u |= mask;
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
            *u >>= 2;
            *u <<= 2;
            *u |= mask;
        }
    }
}

pub fn recover(carrier: impl BufRead + Seek, mut write: impl Write) -> Result<()> {
    let carrier_image = image::load(carrier, PNG)?;
    let carrier_stream = carrier_image.raw_pixels();

    let len = dbg!(read_len(&carrier_stream[0..32]));
    if len != 13 {
        panic!("Doh!");
    }
    
    let bytes: Vec<_> = read_bytes(&carrier_stream[32..], len).collect();

    Ok(write.write_all(&bytes)?)
}

fn read_len(s: &[u8]) -> u64 {
    println!("{:?}", s);
    let mut len = 0u64;
    for &byte in s.iter().rev() {
        len <<= 2;
        len = apply_to_u64(len, byte);
    }
    len
}

fn read_bytes<'a>(s: &'a [u8], len: u64) -> impl Iterator<Item = u8> + 'a {
    s.chunks_exact(4).take(len as usize).map(|chunk| {
        let mut u = 0;
        u = apply_to_u8(u, chunk[3]);
        u <<= 2;
        u = apply_to_u8(u, chunk[2]);
        u <<= 2;
        u = apply_to_u8(u, chunk[1]);
        u <<= 2;
        u = apply_to_u8(u, chunk[0]);
        u
    })
}

fn apply_to_u64(target: u64, bits: u8) -> u64 {
    target | u64::from(bits & 0b0000_0011)
}

fn apply_to_u8(target: u8, bits: u8) -> u8 {
    target | (bits & 0b0000_0011)
}
