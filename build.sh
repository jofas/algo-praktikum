#cargo build --manifest-path ./graphs/Cargo.toml
cd ui
cargo web build --target-webasm
cd ../

cp -f ui/target/wasm32-unknown-unknown/release/ui.js www/ui.js
echo "ui.js copied"
cp -f ui/target/wasm32-unknown-unknown/release/ui.wasm www/ui.wasm
echo "ui.wasm copied"
