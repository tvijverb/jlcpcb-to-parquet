# Convert JLCPCB parts from https://github.com/yaqwsx/jlcparts to parquet

## Requirements
Rust installed on your system
```
wget and p7zip-full
```

## Steps
1. First download the JLCPCB parts sqlite3 database
```bash
wget https://yaqwsx.github.io/jlcparts/data/cache.zip https://yaqwsx.github.io/jlcparts/data/cache.z0{1..8}
7z x cache.zip
```

2. Create parquet files using this repository

```bash
cargo run --release
```