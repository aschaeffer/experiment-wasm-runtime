use std::error::Error;
use wasmtime::*;

fn main() -> Result<(), Box<dyn Error>> {
    // An engine stores and configures global compilation settings like
    // optimization level, enabled wasm features, etc.
    let engine = Engine::default();

    // ===========================================================================================
    // 3-rust-wasm-library
    // ===========================================================================================

    // We start off by creating a `Module` which represents a compiled form
    // of our input wasm module. In this case it'll be JIT-compiled after
    // we parse the text format.
    let module = Module::from_file(
        &engine,
        "target/wasm32-unknown-unknown/debug/rust_wasm_library.wasm",
    )?;

    // A `Store` is what will own instances, functions, globals, etc. All wasm
    // items are stored within a `Store`, and it's what we'll always be using to
    // interact with the wasm world. Custom data can be stored in stores but for
    // now we just use `()`.
    let mut store = Store::new(&engine, ());

    // With a compiled `Module` we can then instantiate it, creating
    // an `Instance` which we can actually poke at functions on.
    let instance = Instance::new(&mut store, &module, &[])?;

    // The `Instance` gives us access to various exported functions and items,
    // which we access here to pull out our `answer` exported function and
    // run it.
    let answer = instance
        .get_func(&mut store, "answer")
        .expect("`answer` was not an exported function");

    // There's a few ways we can call the `answer` `Func` value. The easiest
    // is to statically assert its signature with `typed` (in this case
    // asserting it takes no arguments and returns one i32) and then call it.
    let answer = answer.typed::<(), i32, _>(&store)?;

    // And finally we can call our function! Note that the error propagation
    // with `?` is done to handle the case where the wasm function traps.
    let result = answer.call(&mut store, ())?;
    println!("Answer: {:?}", result);

    // ===========================================================================================
    // 4-use-host-functionality
    // ===========================================================================================

    // A `Store` is what will own instances, functions, globals, etc. All wasm
    // items are stored within a `Store`, and it's what we'll always be using to
    // interact with the wasm world. Custom data can be stored in stores but for
    // now we just use `()`.
    let mut store2 = Store::new(&engine, ());

    // For host-provided functions it's recommended to use a `Linker` which does
    // name-based resolution of functions.
    let mut linker = Linker::<()>::new(&engine);

    // First we create our simple "double" function which will only multiply its
    // input by two and return it.
    linker.func_wrap("host", "two_times", |param: i32| param * 2);

    // We start off by creating a `Module` which represents a compiled form
    // of our input wasm module. In this case it'll be JIT-compiled after
    // we parse the text format.
    let module2 = Module::from_file(
        &engine,
        "target/wasm32-unknown-unknown/debug/use_host_functionality.wasm",
    )?;

    // With a compiled `Module` we can then instantiate it, creating
    // an `Instance` which we can actually poke at functions on.
    // let instance2 = Instance::new(&mut store2, &module2, &[])?;
    let instance2 = linker.instantiate(&mut store2, &module2)?;

    // The `Instance` gives us access to various exported functions and items,
    // which we access here to pull out our `answer` exported function and
    // run it.
    //
    // Call four_times (defined in use_host_functionality.wasm) which calls two_times (defined here)
    // ---> This means it's both possible: call from host to wasm and from wasm to host

    let four_times_fn = instance2
        .get_func(&mut store2, "four_times")
        .expect("`four_times` was not an exported function");

    // There's a few ways we can call the `answer` `Func` value. The easiest
    // is to statically assert its signature with `typed` (in this case
    // asserting it takes no arguments and returns one i32) and then call it.
    // let four_times_typed = four_times_fn.typed::<(i32), i32, _>(&store2)?;
    let four_times_typed = four_times_fn.typed::<i32, i32, _>(&store2)?;

    // And finally we can call our function! Note that the error propagation
    // with `?` is done to handle the case where the wasm function traps.
    // let result_four_times = four_times_typed.call(&mut store2, (2))?;
    let result_four_times = four_times_typed.call(&mut store2, 2)?;
    println!("2 * 2 * 2 = {:?}", result_four_times);

    // ===========================================================================================
    // 5-compile-wasm-at-runtime
    // ===========================================================================================

    Ok(())
}
