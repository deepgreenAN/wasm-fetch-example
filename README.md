# WASMでfetch-API(gloo-net)
## テスト
```
cargo run --bin test_server --features server
```
そして
```
wasm-pack test --chrome --headless
```

## ビルドしてブラウザで利用する
wasmのビルド
```
wasm-pack build --target web
```
サーバーの起動
```
cargo run --bin app --features server
```
そして`localhost:3000`に接続するとコンソールに表示される。
