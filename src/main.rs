use sha2::{Digest, Sha256};
use std::env;
use std::fs;
use std::fs::File;
use std::time::Instant;

mod variants;

#[derive(Debug, PartialEq, Eq)]
enum Mode {
    Compress,
    Decompress,
    Verify,
}

enum Compressor {
    Lz4,
    Zstd,
}

fn parse_mode(input: &str) -> Result<Mode, ()> {
    match input {
        "compress" => Ok(Mode::Compress),
        "decompress" => Ok(Mode::Decompress),
        "verify" => Ok(Mode::Verify),
        _ => Err(()),
    }
}

fn parse_compressor_type(input: &str) -> Result<Compressor, ()> {
    match input {
        "lz4" => Ok(Compressor::Lz4),
        "zstd" => Ok(Compressor::Zstd),
        _ => Err(()),
    }
}

fn get_sha256(path: &String) -> Vec<u8> {
    let buffer = fs::read(path).unwrap();

    let mut hasher = Sha256::new();
    hasher.update(buffer);

    hasher.finalize().to_vec()
}

fn get_compression_func(compressor: Compressor) -> fn(File, File) -> Result<(), std::io::Error> {
    match compressor {
        Compressor::Lz4 => variants::lz4::compress,
        Compressor::Zstd => variants::zstd::compress,
    }
}

fn get_decompression_func(compressor: Compressor) -> fn(File, File) -> Result<(), std::io::Error> {
    match compressor {
        Compressor::Lz4 => variants::lz4::decompress,
        Compressor::Zstd => variants::zstd::decompress,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mode = parse_mode(&args[1]).expect("Supplied mode not available.");

    let in_file_path = &args[2];
    let out_file_path = &args[3];

    if mode == Mode::Verify {
        assert_eq!(get_sha256(in_file_path), get_sha256(out_file_path));
        println!("Success");
        return;
    }

    let compressor_type = parse_compressor_type(&args[4]).expect("Supplied compressor type not available");

    let in_file = File::open(in_file_path).expect("Unable to open in file.");
    let out_file = File::create(out_file_path).expect("Unable to create out file.");

    let start_time = Instant::now();

    match mode {
        Mode::Compress => get_compression_func(compressor_type)(in_file, out_file).unwrap(),
        Mode::Decompress => get_decompression_func(compressor_type)(in_file, out_file).unwrap(),
        Mode::Verify => (),
    }

    println!("{:?} took: {:?}", mode, start_time.elapsed());
}
