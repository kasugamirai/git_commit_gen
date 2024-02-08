mkdir output
cargo build --bin bootstrap --release
cp target/release/bootstrap output
chmod +x output/*
