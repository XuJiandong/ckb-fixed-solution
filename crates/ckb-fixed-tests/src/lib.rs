pub use ckb_fixed;
pub use wasmer::{Instance, Store, TypedFunction, Value};

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub enum Error {
    General,
    Ln,
    Exp,
    Pow,
    FromNum,
    FromStr,
    New,
}

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

pub fn i64f64_ln(store: &mut Store, instance: &Instance, a: i32) -> Result<i32, Error> {
    call_with_result(store, instance, "i64f64_ln", vec![Value::I32(a)]).map_err(|_| Error::Ln)
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
        .get_function("__wbindgen_export_2")
        .unwrap()
        .typed(store)
        .unwrap();
    free.call(store, array_ptr, array_len, 1).unwrap();

    // Clean up the stack
    add_to_stack_pointer.call(store, 16).unwrap();

    result
}

pub fn new(store: &mut Store, instance: &Instance, inner: &[u8]) -> Result<i32, Error> {
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
        .get_function("__wbindgen_export_0")
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
    let _r1 = read_i32(retptr + 4);
    let r2 = read_i32(retptr + 8);

    // Clean up the stack
    add_to_stack_pointer.call(store, 16).unwrap();

    // Handle error case
    if r2 != 0 {
        return Err(Error::New);
    }

    Ok(r0)
}

pub fn from_str(store: &mut Store, instance: &Instance, s: &str) -> Result<i32, Error> {
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
        .get_function("__wbindgen_export_0")
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
    let _r1 = read_i32(retptr + 4);
    let r2 = read_i32(retptr + 8);

    // Clean up the stack
    add_to_stack_pointer.call(store, 16).unwrap();

    // Handle error case
    if r2 != 0 {
        return Err(Error::FromStr);
    }

    Ok(r0)
}

pub fn i64f64_pow(
    store: &mut Store,
    instance: &Instance,
    base: i32,
    exponent: i32,
) -> Result<i32, Error> {
    call_with_result(
        store,
        instance,
        "i64f64_pow",
        vec![Value::I32(base), Value::I32(exponent)],
    )
    .map_err(|_| Error::Pow)
}

pub fn from_num(store: &mut Store, instance: &Instance, n: i64) -> Result<i32, Error> {
    call_with_result(store, instance, "i64f64_from_num", vec![Value::I64(n)])
        .map_err(|_| Error::FromNum)
}

pub fn call_with_result(
    store: &mut Store,
    instance: &Instance,
    fn_name: &'static str,
    args: Vec<Value>,
) -> Result<i32, Error> {
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
    let mut new_args = vec![Value::I32(ret_ptr)];
    new_args.extend(args);

    let func = instance.exports.get_function(fn_name).unwrap();
    func.call(store, new_args.as_slice()).unwrap();

    let view = memory.view(store);
    let read_i32 = |ptr: i32| -> i32 {
        let mut bytes = [0u8; 4];
        view.read(ptr as u64, &mut bytes).unwrap();
        i32::from_le_bytes(bytes)
    };

    let value = read_i32(ret_ptr);
    let is_error = read_i32(ret_ptr + 8);
    if is_error != 0 {
        return Err(Error::General);
    }

    add_to_stack_pointer.call(store, 16).unwrap();
    Ok(value)
}
