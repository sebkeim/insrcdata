

100à LIGNES 500K
..//target/debug/insrcdata bench/insrcdata/insrcdata.toml  0,01s user 0,00s system 41% cpu 0,029 total
cargo build --manifest-path ./bench/rust-bench/Cargo.toml  0,05s user 0,02s system 49% cpu 0,138 total
./bench/rust-bench/target/debug/bench  0,00s user 0,00s system 2% cpu 0,082 total

time ..//target/debug/insrcdata bench/insrcdata/insrcdata.toml 
time cargo build --manifest-path ./bench/rust-bench/Cargo.toml
time ./bench/rust-bench/target/debug/bench 