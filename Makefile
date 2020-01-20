all: pkg/veedoom.js

pkg/veedoom.js: src/lib.rs
	wasm-pack build --target web

