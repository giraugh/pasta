#! /bin/sh
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
cargo check
wasm-pack build
wasm-pack build --target nodejs --out-dir pkg-node
rm ./pkg/package.json
yarn changeset publish
