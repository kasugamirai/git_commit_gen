mkdir output
cargo build --bin gitgen --release
cp target/release/gitgen output
chmod +x output/*
