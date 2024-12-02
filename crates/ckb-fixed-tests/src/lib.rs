#[test]
fn test_basic() {
    use wasmer::{imports, Instance, Module, Store, Value};

    // Initialize wasmer store and load wasm module
    let mut store = Store::default();
    let bytes = include_bytes!("../../ckb-fixed/pkg/ckb_fixed_bg.wasm");
    let module = Module::new(&store, bytes).unwrap();

    // Create import object with required wasm-bindgen functions
    let import_object = imports! {
        "wbg" => {
            "__wbindgen_string_new" => wasmer::Function::new_typed(
                &mut store,
                |_ptr: i32, _len: i32| -> i32 { 0 }
            ),
            "__wbindgen_throw" => wasmer::Function::new_typed(
                &mut store,
                |_ptr: i32, _len: i32| {}
            ),
            // Add other required wasm-bindgen imports as needed
        }
    };
    let instance = Instance::new(&mut store, &module, &import_object).unwrap();

    // Get memory export (required for wasm-bindgen generated code)
    let memory = instance.exports.get_memory("memory").unwrap();

    // Setup the stack pointer for wasm-bindgen
    let add_to_stack_pointer = instance
        .exports
        .get_function("__wbindgen_add_to_stack_pointer")
        .unwrap();

    // Allocate 16 bytes for the return value
    let ret_ptr = add_to_stack_pointer
        .call(&mut store, &[Value::I32(-16)])
        .unwrap()[0]
        .unwrap_i32();

    // Call i64f64_from_num with input 42
    let from_num = instance.exports.get_function("i64f64_from_num").unwrap();
    from_num
        .call(&mut store, &[Value::I32(ret_ptr), Value::I64(42)])
        .unwrap();

    // Read the result from memory (i64f64 is represented as two i64s)
    let view = memory.view(&store);
    let read_i64 = |ptr: i32| -> i64 {
        let mut bytes = [0u8; 8];
        view.read(ptr as u64, &mut bytes).unwrap();
        i64::from_le_bytes(bytes)
    };

    let low_bits = read_i64(ret_ptr);
    let high_bits = read_i64(ret_ptr + 8);

    println!("Result: low_bits={}, high_bits={}", low_bits, high_bits);

    // Clean up the stack (important!)
    add_to_stack_pointer
        .call(&mut store, &[Value::I32(16)])
        .unwrap();
}
