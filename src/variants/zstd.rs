use std::{io, fs::File};
use zstd::{Decoder, Encoder};

pub fn compress(mut a: File, b: File) -> io::Result<()> {
    let mut encoder = Encoder::new(b, 1)?;

    io::copy(&mut a, &mut encoder)?;
    encoder.finish()?;

    Ok(())
}

pub fn decompress(a: File, mut b: File) -> io::Result<()> {
    let mut decoder = Decoder::new(a)?;

    io::copy(&mut decoder, &mut b)?;

    Ok(())
}