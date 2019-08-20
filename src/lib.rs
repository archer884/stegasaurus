mod error;
mod masks;

use error::Error;
use image::{png::PNGEncoder, GenericImageView};
use masks::Masks;
use std::io::Write;

pub type Result<T, E = error::Error> = std::result::Result<T, E>;

/// Store an arbitrary message within a PNG image.
/// 
/// The type of the source image probably doesn't matter, but the output image will be stored as
/// PNG in order to avoid potential data loss due to compression.
pub fn store(message: &[u8], carrier: &[u8], write: impl Write) -> Result<()> {
    static MESSAGE_METADATA_LEN: usize = 160;

    let carrier_image = image::load_from_memory(carrier)?;
    let (width, height) = carrier_image.dimensions();
    let color_type = carrier_image.color();
    let destination_image = PNGEncoder::new(write);

    let mut carrier_stream = carrier_image.raw_pixels();

    if carrier_stream.len() < MESSAGE_METADATA_LEN + message.len() * 4 {
        return Err(Error::Length);
    }

    write_masks(message.len(), &mut carrier_stream[0..32]);
    write_masks(hash(message).as_ref(), &mut carrier_stream[32..160]);
    write_masks(message, &mut carrier_stream[160..]);

    Ok(destination_image.encode(&carrier_stream, width, height, color_type)?)
}

/// Recover a message stored using the library's store routine.
pub fn recover(carrier: &[u8], mut write: impl Write) -> Result<()> {
    let carrier_image = image::load_from_memory(carrier)?;
    let carrier_stream = carrier_image.raw_pixels();

    let len = read_len(&carrier_stream[0..32]) as usize;

    let expected_hash: Vec<_> = read_bytes(&carrier_stream[32..160]).collect();
    let bytes: Vec<_> = read_bytes(&carrier_stream[160..(160 + len * 4)]).collect();

    if expected_hash != hash(&bytes) {
        return Err(Error::Checksum);
    }

    Ok(write.write_all(&bytes)?)
}

fn write_masks(content: impl Masks, carrier: &mut [u8]) {
    for (u, mask) in carrier.iter_mut().zip(content.masks()) {
        *u >>= 2;
        *u <<= 2;
        *u |= mask;
    }
}

fn read_len(s: &[u8]) -> u64 {
    let mut len = 0u64;
    for &byte in s.iter().rev() {
        len <<= 2;
        len = apply_to_u64(len, byte);
    }
    len
}

fn read_bytes<'a>(s: &'a [u8]) -> impl Iterator<Item = u8> + 'a {
    s.chunks_exact(4).map(|chunk| {
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

fn hash(s: &[u8]) -> [u8; 32] {
    use sha3::{Digest, Sha3_256};
    let mut hasher = Sha3_256::new();
    hasher.input(s);
    hasher.result().into()
}
