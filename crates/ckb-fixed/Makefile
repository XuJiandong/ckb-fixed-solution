

wasm-pack:
	wasm-pack build --target web --features std

debug-wasm-pack:
	wasm-pack build --target web --debug --features std
	wasm2wat pkg/ckb_fixed_bg.wasm -o pkg/ckb_fixed_bg.wat
