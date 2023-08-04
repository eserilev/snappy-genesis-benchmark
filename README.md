# snappy compression of genesis.ssz files for Lighthouse

The following is a summary of an issue written by paulhauner

Lighthouse currently includes the uncompressed `genesis.ssz` for several supported networks as part of their binary. The total size of the genesis files is almost 60M, which makes up more than half of the total binary size (~110M). We could decrease the size of the binary by storing the compressed `genesis.ssz` bytes in the binary and decompressing them at start-up.

This repo focuses on using snappy compression/decompression on `genesis.ssz` for mainnet and prater. The goal here is to understand the time it takes to decompress these files as we don't want to dramatically slow-down BN/VC startup.

Running the project via `cargo run` will provide logs of the time it takes to compress and decompress each file. Running on my local machine I got the following results:

```
Time elapsed in compress_genesis_mainnet() is: 1.633691375s
Time elapsed in decompress_genesis_mainnet() is: 1.50549625s

Time elapsed in compress_genesis_prater() is: 10.20794425s
Time elapsed in decompress_genesis_prater() is: 9.845210542s
```

The file sizes for the compressed and decompressed ssz are listed below

```
decompressed mainnet: 5.4M
compressed mainnet: 1.8M

decompressed prater: 29.8M
compressed prater: 18.1M
```

Snappy compression seems to reduce file size by ~50%, while increasing start-up time by potentially 10s of seconds.