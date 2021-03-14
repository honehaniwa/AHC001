cargo build --release --offline --quiet
RUST_BACKTRACE=1 ./target/release/ahc001 < in > out   