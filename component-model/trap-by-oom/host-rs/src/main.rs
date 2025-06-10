use std::path::PathBuf;

use anyhow::Context;
use clap::Parser;
use wasmtime::component::{Component, Linker, Val};
use wasmtime::{Config, Engine, Store, Trap};
use wasmtime_wasi::WasiCtxBuilder;

fn main() -> anyhow::Result<()> {
    let Cli { path } = Cli::parse();

    let mut config = Config::default();
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;
    let mut linker = Linker::new(&engine);

    // Add the command world (aka WASI CLI) to the linker
    wasmtime_wasi::add_to_linker_sync(&mut linker).context("link command world")?;

    let ctx = WasiCtxBuilder::new().inherit_stdout().build_p1();
    let mut store = Store::new(&engine, ctx);

    let component = Component::from_file(&engine, path).context("Component file not found")?;

    const INTERFACE: &str = "sammyne:helloworld/greeter@1.0.0";
    const FUNC_NAME: &str = "say-hello";

    let instance = linker
        .instantiate(&mut store, &component)
        .context("instantiate")?;

    // ref: https://docs.rs/wasmtime/30.0.2/wasmtime/component/struct.Instance.html#method.get_func
    let say_hello = {
        let instance_idx = instance
            .get_export(&mut store, None, INTERFACE)
            .with_context(|| format!("miss interface: {INTERFACE}"))?;
        let func_idx = instance
            .get_export(&mut store, Some(&instance_idx), FUNC_NAME)
            .with_context(|| format!("locate func '{FUNC_NAME}'"))?;
        instance
            .get_func(&mut store, &func_idx)
            .with_context(|| format!("load func '{FUNC_NAME}'"))?
    };

    let name = "a".repeat(1024 * 1024 * 1024);

    for i in 0.. {
        let params = [new_hello_request(name.clone())];
        let mut results = [Val::Bool(false)];
        match say_hello.call(&mut store, &params, &mut results) {
            Ok(_) => {}
            Err(err) => {
                match err.downcast_ref::<Trap>() {
                    None => println!("#{i} non-trap err: {err}"),
                    Some(c) => println!("#{i} trap err: {c}\n{err}"),
                }
                break;
            }
        }
        // post-return 清理 say-hello 关联的状态。
        say_hello
            .post_return(&mut store)
            .with_context(|| format!("#{i} post return '{FUNC_NAME}'"))?;
    }

    let say_hello = {
        let instance_idx = instance
            .get_export(&mut store, None, INTERFACE)
            .with_context(|| format!("miss interface: {INTERFACE}"))?;
        let func_idx = instance
            .get_export(&mut store, Some(&instance_idx), FUNC_NAME)
            .with_context(|| format!("locate func '{FUNC_NAME}'"))?;
        instance
            .get_func(&mut store, &func_idx)
            .with_context(|| format!("load func '{FUNC_NAME}'"))?
    };

    let params = [new_hello_request(name.clone())];
    let mut results = [Val::Bool(false)];
    say_hello
        .call(&mut store, &params, &mut results)
        .context("call after trap")?;

    println!("hello world");

    Ok(())
}

/// A CLI for executing WebAssembly components that
/// implement the `example` world.
#[derive(Parser)]
#[clap(name = "hello-world-host", version = env!("CARGO_PKG_VERSION"))]
struct Cli {
    /// WASM 组件的路径
    #[clap(short, long)]
    path: PathBuf,
}

fn new_hello_request(name: String) -> Val {
    Val::Record(vec![("name".to_owned(), Val::String(name))])
}
