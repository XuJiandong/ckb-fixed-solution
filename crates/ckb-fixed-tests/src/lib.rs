use wasmer::{Instance, Store, TypedFunction};

#[cfg(test)]
mod tests;

pub fn initialize_wasmer() -> (Store, Instance) {
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

    (store, instance)
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

pub fn i64f64_ln(store: &mut Store, instance: &Instance, a: i32) -> i32 {
    // Get memory export
    let memory = instance.exports.get_memory("memory").unwrap();

    // Setup the stack pointer
    let add_to_stack_pointer: TypedFunction<i32, i32> = instance
        .exports
        .get_function("__wbindgen_add_to_stack_pointer")
        .unwrap()
        .typed(store)
        .unwrap();

    // Allocate 16 bytes for the return value
    let ret_ptr = add_to_stack_pointer.call(store, -16).unwrap();

    // Call i64f64_ln with input value
    let ln: TypedFunction<(i32, i32), ()> = instance
        .exports
        .get_function("i64f64_ln")
        .unwrap()
        .typed(store)
        .unwrap();
    ln.call(store, ret_ptr, a).unwrap();

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

    add_to_stack_pointer.call(store, 16).unwrap();
    value
}

pub fn to_le_bytes(store: &mut Store, instance: &Instance, ptr: i32) -> Vec<u8> {
    // Get memory export
    let memory = instance.exports.get_memory("memory").unwrap();

    // Setup the stack pointer
    let add_to_stack_pointer: TypedFunction<i32, i32> = instance
        .exports
        .get_function("__wbindgen_add_to_stack_pointer")
        .unwrap()
        .typed(store)
        .unwrap();

    // Allocate 16 bytes for the return value
    let ret_ptr = add_to_stack_pointer.call(store, -16).unwrap();

    // Call i64f64_to_le_bytes
    let to_le_bytes: TypedFunction<(i32, i32), ()> = instance
        .exports
        .get_function("i64f64_to_le_bytes")
        .unwrap()
        .typed(store)
        .unwrap();
    to_le_bytes.call(store, ret_ptr, ptr).unwrap();

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
    let free: TypedFunction<(i32, i32, i32), ()> = instance
        .exports
        .get_function("__wbindgen_free")
        .unwrap()
        .typed(store)
        .unwrap();
    free.call(store, array_ptr, array_len, 1).unwrap();

    // Clean up the stack
    add_to_stack_pointer.call(store, 16).unwrap();

    result
}

pub fn new(store: &mut Store, instance: &Instance, inner: &[u8]) -> i32 {
    // Get memory export
    let memory = instance.exports.get_memory("memory").unwrap();

    // Setup the stack pointer
    let add_to_stack_pointer: TypedFunction<i32, i32> = instance
        .exports
        .get_function("__wbindgen_add_to_stack_pointer")
        .unwrap()
        .typed(store)
        .unwrap();

    // Allocate 16 bytes for the return value
    let retptr = add_to_stack_pointer.call(store, -16).unwrap();

    // Allocate memory for the input array and copy the bytes
    let malloc: TypedFunction<(i32, i32), i32> = instance
        .exports
        .get_function("__wbindgen_malloc")
        .unwrap()
        .typed(store)
        .unwrap();
    let ptr0 = malloc.call(store, inner.len() as i32, 1).unwrap();

    // Copy input bytes to WASM memory
    {
        let view = memory.view(store);
        view.write(ptr0 as u64, inner).unwrap();
    }

    // Call i64f64_new
    let new_func: TypedFunction<(i32, i32, i32), ()> = instance
        .exports
        .get_function("i64f64_new")
        .unwrap()
        .typed(store)
        .unwrap();
    new_func
        .call(store, retptr, ptr0, inner.len() as i32)
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
    add_to_stack_pointer.call(store, 16).unwrap();

    // Handle error case
    if r2 != 0 {
        panic!("Error creating I64F64: {}", r1);
    }

    r0
}

pub fn from_str(store: &mut Store, instance: &Instance, s: &str) -> i32 {
    // Get memory export
    let memory = instance.exports.get_memory("memory").unwrap();

    // Setup the stack pointer
    let add_to_stack_pointer: TypedFunction<i32, i32> = instance
        .exports
        .get_function("__wbindgen_add_to_stack_pointer")
        .unwrap()
        .typed(store)
        .unwrap();

    // Allocate 16 bytes for the return value
    let retptr = add_to_stack_pointer.call(store, -16).unwrap();

    // Allocate memory for the input string and copy the bytes
    let malloc: TypedFunction<(i32, i32), i32> = instance
        .exports
        .get_function("__wbindgen_malloc")
        .unwrap()
        .typed(store)
        .unwrap();
    let ptr0 = malloc.call(store, s.len() as i32, 1).unwrap();

    // Copy input string bytes to WASM memory
    {
        let view = memory.view(store);
        view.write(ptr0 as u64, s.as_bytes()).unwrap();
    }

    // Call i64f64_from_str
    let from_str: TypedFunction<(i32, i32, i32), ()> = instance
        .exports
        .get_function("i64f64_from_str")
        .unwrap()
        .typed(store)
        .unwrap();
    from_str.call(store, retptr, ptr0, s.len() as i32).unwrap();

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
    add_to_stack_pointer.call(store, 16).unwrap();

    // Handle error case
    if r2 != 0 {
        panic!("Error parsing string: {}", r1);
    }

    r0
}

pub fn i64f64_pow(store: &mut Store, instance: &Instance, base: i32, exponent: i32) -> i32 {
    // Get memory export
    let memory = instance.exports.get_memory("memory").unwrap();

    // Setup the stack pointer
    let add_to_stack_pointer: TypedFunction<i32, i32> = instance
        .exports
        .get_function("__wbindgen_add_to_stack_pointer")
        .unwrap()
        .typed(store)
        .unwrap();

    // Allocate 16 bytes for the return value
    let ret_ptr = add_to_stack_pointer.call(store, -16).unwrap();

    // Call i64f64_pow with input values
    let pow: TypedFunction<(i32, i32, i32), ()> = instance
        .exports
        .get_function("i64f64_pow")
        .unwrap()
        .typed(store)
        .unwrap();
    pow.call(store, ret_ptr, base, exponent).unwrap();

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

    add_to_stack_pointer.call(store, 16).unwrap();
    value
}

pub fn from_num(store: &mut Store, instance: &Instance, n: i64) -> i32 {
    // Setup the stack pointer
    let add_to_stack_pointer: TypedFunction<i32, i32> = instance
        .exports
        .get_function("__wbindgen_add_to_stack_pointer")
        .unwrap()
        .typed(store)
        .unwrap();

    // Allocate 16 bytes for the return value
    let retptr = add_to_stack_pointer.call(store, -16).unwrap();

    // Call i64f64_from_num
    let from_num: TypedFunction<(i32, i64), ()> = instance
        .exports
        .get_function("i64f64_from_num")
        .unwrap()
        .typed(store)
        .unwrap();
    from_num.call(store, retptr, n).unwrap();

    // Get memory export and read results
    let memory = instance.exports.get_memory("memory").unwrap();
    let view = memory.view(store);
    let read_i32 = |ptr: i32| -> i32 {
        let mut bytes = [0u8; 4];
        view.read(ptr as u64, &mut bytes).unwrap();
        i32::from_le_bytes(bytes)
    };

    let r0 = read_i32(retptr);
    let r1 = read_i32(retptr + 4);
    let r2 = read_i32(retptr + 8);

    // Clean up the stack
    add_to_stack_pointer.call(store, 16).unwrap();

    // Handle error case
    if r2 != 0 {
        panic!("Error converting number: {}", r1);
    }

    r0
}
