use lz4::{Decoder, EncoderBuilder};
use std::{io, fs::File};

pub fn compress(mut a: File, b: File) -> io::Result<()> {
    let mut encoder = EncoderBuilder::new().level(1).build(b)?;

    io::copy(&mut a, &mut encoder)?;

    let (_output, result) = encoder.finish();
    result
}

pub fn decompress(a: File, mut b: File) -> io::Result<()> {
    let mut decoder = Decoder::new(a)?;

    io::copy(&mut decoder, &mut b)?;

    Ok(())
}
