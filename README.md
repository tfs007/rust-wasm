```
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli

cargo new nft_price_fetcher --lib

cd nft_price_fetcher

```

After Rust code is written 

` wasm-pack build --target web ` 
` cp -r /path/to/your/rust/project/pkg ./public/ `

