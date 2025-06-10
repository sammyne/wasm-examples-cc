use std::path::PathBuf;

use anyhow::Context;
use clap::Parser;
use wasmtime::component::{Component, Linker};
use wasmtime::{Config, Engine, Store};
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

    // 默认 instance 上限为 10000。
    // 限制可以借助 Store::limiter 函数调整。
    // 源码参见 https://github.com/bytecodealliance/wasmtime/blob/release-33.0.0/crates/wasmtime/src/runtime/limits.rs#L4。
    for i in 0.. {
        let _instance = linker
            .instantiate(&mut store, &component)
            .with_context(|| format!("#{i} instantiate"))?;
    }

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
