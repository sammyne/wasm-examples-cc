use wasmtime::*;

fn main() -> Result<()> {
    let path = match std::env::args().skip(1).next() {
        Some(v) => v,
        None => panic!("miss wasm file"),
    };

    // Load our previously compiled wasm file (built previously with Cargo) and
    // also ensure that we generate debuginfo so this executable can be
    // debugged in GDB.
    let engine = Engine::new(
        Config::new()
            .debug_info(true)
            .cranelift_opt_level(OptLevel::None),
    )?;
    let mut store = Store::new(&engine, ());

    let module = Module::from_file(&engine, path)?;
    let instance = Instance::new(&mut store, &module, &[])?;

    // Invoke `fib` export
    let fib = instance.get_typed_func::<i32, i32>(&mut store, "fib-cc")?;
    println!("fib(6) = {}", fib.call(&mut store, 6)?);
    Ok(())
}