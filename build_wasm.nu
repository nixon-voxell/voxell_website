cargo build --no-default-features --release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --target web --out-dir ./ --out-name "voxell_website" ./target/wasm32-unknown-unknown/release/voxell_website.wasm
