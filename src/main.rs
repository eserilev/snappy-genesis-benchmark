use criterion::{criterion_group, criterion_main, Criterion};
use std::{
    env,
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
};

fn main() {
    env::current_dir().unwrap();

    let start = std::time::Instant::now();
    compress_genesis_file(
        "genesis/genesis_mainnet.ssz",
        "genesis/compressed_genesis_mainnet.snappy",
    );
    let duration = start.elapsed();
    println!("Time elapsed in compress_genesis_mainnet() is: {:?}", duration);

    let start = std::time::Instant::now();
    compress_genesis_file(
        "genesis/genesis_prater.ssz",
        "genesis/compressed_genesis_prater.snappy",
    );
    let duration = start.elapsed();
    println!("Time elapsed in compress_genesis_prater() is: {:?}", duration);

    let start = std::time::Instant::now();
    decompress_genesis_file(
        "genesis/compressed_genesis_mainnet.snappy",
        "genesis/genesis_mainnet_decompressed.ssz",
    );
    let duration = start.elapsed();
    println!("Time elapsed in decompress_genesis_mainnet() is: {:?}", duration);

    let start = std::time::Instant::now();
    decompress_genesis_file(
        "genesis/compressed_genesis_prater.snappy",
        "genesis/genesis_prater_decompressed.ssz",
    );
    let duration = start.elapsed();
    println!("Time elapsed in decompress_genesis_prater() is: {:?}", duration);
}

fn compress_genesis_file(source_filename: &str, target_filename: &str) {
    let file: File = File::open(source_filename).unwrap();
    let mut reader = BufReader::new(file);

    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).unwrap();

    let output_file = File::create(target_filename).unwrap();
    let mut writer = BufWriter::new(output_file);

    // Wrap writer with FrameEncoder to compress the data
    let mut encoder = snap::write::FrameEncoder::new(&mut writer);

    // Write compressed data into the file
    encoder.write_all(&buffer).unwrap();

    // Ensure all data is flushed
    encoder.flush().unwrap();
}

fn decompress_genesis_file(source_filename: &str, target_filename: &str) {
    // Open the compressed file to read
    let file = File::open(source_filename).unwrap();
    let mut reader = BufReader::new(file);

    // Wrap the reader with a FrameDecoder to decompress the data
    let mut decoder = snap::read::FrameDecoder::new(reader);

    // Read decompressed data into a Vec<u8>
    let mut buffer = Vec::new();
    decoder.read_to_end(&mut buffer).unwrap();

    // Open a file to write
    let output_file = File::create(target_filename).unwrap();
    let mut writer = BufWriter::new(output_file);

    // Write decompressed data into the file
    writer.write_all(&buffer).unwrap();

    // Ensure all data is flushed
    writer.flush().unwrap();
}
