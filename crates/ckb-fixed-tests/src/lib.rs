use wasmer::{Instance, Store, TypedFunction, Value};

#[test]
fn test_basic() {
    use wasmer::{imports, Module};
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
        }
    };
    let instance = Instance::new(&mut store, &module, &import_object).unwrap();

    let value = from_num(&mut store, &instance, 42);
    let value2 = from_num(&mut store, &instance, 1);
    let result = i64f64_add(&mut store, &instance, value, value2);
    println!(
        "dump memory: {:?}",
        fetch_memory(&mut store, &instance, result, 32)
    );

    let rust_value = ckb_fixed::I64F64::from_num(43).unwrap();

    assert_eq!(
        rust_value.to_le_bytes(),
        to_le_bytes(&mut store, &instance, result)
    );
    let new_value = new(&mut store, &instance, &rust_value.to_le_bytes());
    assert_eq!(
        rust_value.to_le_bytes(),
        to_le_bytes(&mut store, &instance, new_value)
    );
}

#[test]
fn test_i64f64() {
    let v = ckb_fixed::types::I64F64::from_num(42);
    println!("to_le_bytes: {:?}", v.to_le_bytes());
    println!("raw: {:?}", unsafe {
        std::mem::transmute::<ckb_fixed::types::I64F64, [u8; 16]>(v)
    });
}

pub fn i64f64_add(store: &mut Store, instance: &Instance, a: i32, b: i32) -> i32 {
    // Call i64f64_add with input values
    let add: TypedFunction<(i32, i32), i32> = instance
        .exports
        .get_function("i64f64_add")
        .unwrap()
        .typed(store)
        .unwrap();
    let result = add.call(store, a, b).unwrap();
    result
}

pub fn fetch_memory(store: &mut Store, instance: &Instance, ptr: i32, length: usize) -> Vec<u8> {
    let memory = instance.exports.get_memory("memory").unwrap();
    let view = memory.view(store);

    let mut bytes = vec![0u8; length];
    view.read(ptr as u64, &mut bytes).unwrap();
    bytes
}

pub fn from_num(store: &mut Store, instance: &Instance, value: i64) -> i32 {
    // Get memory export (required for wasm-bindgen generated code)
    let memory = instance.exports.get_memory("memory").unwrap();

    // Setup the stack pointer for wasm-bindgen
    let add_to_stack_pointer = instance
        .exports
        .get_function("__wbindgen_add_to_stack_pointer")
        .unwrap();

    // Allocate 16 bytes for the return value
    let ret_ptr = add_to_stack_pointer
        .call(store, &[Value::I32(-16)])
        .unwrap()[0]
        .unwrap_i32();

    // Call i64f64_from_num with input value
    let from_num = instance.exports.get_function("i64f64_from_num").unwrap();
    from_num
        .call(store, &[Value::I32(ret_ptr), Value::I64(value)])
        .unwrap();

    let view = memory.view(store);
    let read_i32 = |ptr: i32| -> i32 {
        let mut bytes = [0u8; 4];
        view.read(ptr as u64, &mut bytes).unwrap();
        i32::from_le_bytes(bytes)
    };

    let value = read_i32(ret_ptr);
    let is_error = read_i32(ret_ptr + 8);
    if is_error != 0 {
        panic!("High bits are not zero");
    }

    // Clean up the stack (important!)
    add_to_stack_pointer.call(store, &[Value::I32(16)]).unwrap();
    value
}

pub fn to_le_bytes(store: &mut Store, instance: &Instance, ptr: i32) -> Vec<u8> {
    // Get memory export
    let memory = instance.exports.get_memory("memory").unwrap();

    // Setup the stack pointer
    let add_to_stack_pointer = instance
        .exports
        .get_function("__wbindgen_add_to_stack_pointer")
        .unwrap();

    // Allocate 16 bytes for the return value
    let ret_ptr = add_to_stack_pointer
        .call(store, &[Value::I32(-16)])
        .unwrap()[0]
        .unwrap_i32();

    // Call i64f64_to_le_bytes
    let to_le_bytes = instance.exports.get_function("i64f64_to_le_bytes").unwrap();
    to_le_bytes
        .call(store, &[Value::I32(ret_ptr), Value::I32(ptr)])
        .unwrap();

    // Read the results from memory
    let view = memory.view(store);
    let read_i32 = |ptr: i32| -> i32 {
        let mut bytes = [0u8; 4];
        view.read(ptr as u64, &mut bytes).unwrap();
        i32::from_le_bytes(bytes)
    };

    // Get pointer and length of the returned array
    let array_ptr = read_i32(ret_ptr);
    let array_len = read_i32(ret_ptr + 4);

    // Read the bytes from memory
    let mut result = vec![0u8; array_len as usize];
    view.read(array_ptr as u64, &mut result).unwrap();

    // Free the allocated memory
    let free = instance.exports.get_function("__wbindgen_free").unwrap();
    free.call(
        store,
        &[Value::I32(array_ptr), Value::I32(array_len), Value::I32(1)],
    )
    .unwrap();

    // Clean up the stack
    add_to_stack_pointer.call(store, &[Value::I32(16)]).unwrap();

    result
}

pub fn new(store: &mut Store, instance: &Instance, inner: &[u8]) -> i32 {
    // Get memory export
    let memory = instance.exports.get_memory("memory").unwrap();

    // Setup the stack pointer
    let add_to_stack_pointer = instance
        .exports
        .get_function("__wbindgen_add_to_stack_pointer")
        .unwrap();

    // Allocate 16 bytes for the return value
    let retptr = add_to_stack_pointer
        .call(store, &[Value::I32(-16)])
        .unwrap()[0]
        .unwrap_i32();

    // Allocate memory for the input array and copy the bytes
    let malloc = instance.exports.get_function("__wbindgen_malloc").unwrap();
    let ptr0 = malloc
        .call(store, &[Value::I32(inner.len() as i32), Value::I32(1)])
        .unwrap()[0]
        .unwrap_i32();

    // Copy input bytes to WASM memory
    {
        let view = memory.view(store);
        view.write(ptr0 as u64, inner).unwrap();
    }

    // Call i64f64_new
    let new_func = instance.exports.get_function("i64f64_new").unwrap();
    new_func
        .call(
            store,
            &[
                Value::I32(retptr),
                Value::I32(ptr0),
                Value::I32(inner.len() as i32),
            ],
        )
        .unwrap();

    let view = memory.view(store);
    // Read the results
    let read_i32 = |ptr: i32| -> i32 {
        let mut bytes = [0u8; 4];
        view.read(ptr as u64, &mut bytes).unwrap();
        i32::from_le_bytes(bytes)
    };

    let r0 = read_i32(retptr);
    let r1 = read_i32(retptr + 4);
    let r2 = read_i32(retptr + 8);

    // Clean up the stack
    add_to_stack_pointer.call(store, &[Value::I32(16)]).unwrap();

    // Handle error case
    if r2 != 0 {
        panic!("Error creating I64F64: {}", r1);
    }

    r0
}
