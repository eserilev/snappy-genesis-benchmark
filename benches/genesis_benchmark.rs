use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::{
    env,
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
};

fn compress_genesis_file(source_filename: &str, target_filename: &str) {

    env::current_dir().unwrap();

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

pub fn bench_compress_prater(c: &mut Criterion) {  
    c.bench_function("compress_prater", |b| {
        b.iter(|| {
            env::current_dir().unwrap();
            compress_genesis_file(
                black_box("genesis/genesis_prater.ssz"),
                black_box("genesis/compressed_genesis_prater.snappy"),
            )
        })
    });
}

fn bench_compress_mainnet(c: &mut Criterion) {
    env::current_dir().unwrap();
    c.bench_function("compress_mainnet", |b| {
        b.iter(|| {
            compress_genesis_file(
                black_box("genesis/genesis_mainnet.ssz"),
                black_box("genesis/compressed_genesis_mainnet.snappy"),
            )
        })
    });
}


fn bench_decompress_mainnet(c: &mut Criterion) {
    env::current_dir().unwrap();
    c.bench_function("decompress_mainnet", |b| {
        b.iter(|| {
            decompress_genesis_file(
                black_box("genesis/compressed_genesis_mainnet.snappy"),
                black_box("genesis/genesis_mainnet_decompressed.ssz"),
            )
        })
    });
}

fn bench_decompress_prater(c: &mut Criterion) {
    env::current_dir().unwrap();
    c.bench_function("decompress_prater", |b| {
        b.iter(|| {
            decompress_genesis_file(
                black_box("genesis/compressed_genesis_prater.snappy"),
                black_box("genesis/genesis_prater_decompressed.ssz"),
            )
        })
    });
}

criterion_group!(benches, bench_compress_mainnet, bench_compress_prater, bench_decompress_mainnet, bench_decompress_prater);
criterion_main!(benches);
