use std::path::PathBuf;

use anyhow::Context as _;
use bindings::sammyne::helloworld::types::Host;
use clap::Parser;
use wasmtime::component::{Component, Linker, Resource, ResourceAny, Val};
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::preview1::WasiP1Ctx;
use wasmtime_wasi::{IoView, WasiCtx, WasiCtxBuilder, WasiView};

mod bindings {
    wasmtime::component::bindgen!(in "../guest/wit");
}

use crate::bindings::sammyne::helloworld::types::{Context, HostContext};

fn main() -> anyhow::Result<()> {
    let Cli { path, name } = Cli::parse();

    let mut config = Config::default();
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;
    let mut linker = Linker::new(&engine);

    // Add the command world (aka WASI CLI) to the linker
    wasmtime_wasi::add_to_linker_sync(&mut linker).context("link command world")?;
    bindings::Helloworld::add_to_linker(&mut linker, |v: &mut MyHost| v)
        .context("helloworld add-to-linker")?;

    let mut store = Store::new(&engine, MyHost::new());

    let component = Component::from_file(&engine, path).context("Component file not found")?;

    const INTERFACE: &str = "sammyne:helloworld/greeter@1.0.0";
    const FUNC_NAME: &str = "say-hello";

    let instance = linker
        .instantiate(&mut store, &component)
        .context("instantiate")?;

    let owned_ctx: Resource<Context> = Resource::new_own(0);
    let ctx = ResourceAny::try_from_resource(owned_ctx, &mut store).context("borrow ctx")?;

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
    let params = [Val::Resource(ctx), new_hello_request(name.clone())];
    let mut results = [Val::Bool(false)];
    say_hello
        .call(&mut store, &params, &mut results)
        .context("call")?;

    // match store.data_mut().table().delete(Resource::<Context>::new_borrow(0)) {
    //     Ok(_) => {},
    //     Err(err) => anyhow::bail!("delete ctx resource: {err}"),
    // }

    // post-return 清理 say-hello 关联的状态。
    say_hello
        .post_return(&mut store)
        .with_context(|| format!("post return '{FUNC_NAME}'"))?;
    println!("say-hello returns {results:?}");

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
    /// greet 函数的入参依赖的 name 字段。
    #[clap(short, long)]
    name: String,
}

fn new_hello_request(name: String) -> Val {
    Val::Record(vec![("name".to_owned(), Val::String(name))])
}

struct MyHost {
    ctx: WasiP1Ctx,
}

impl MyHost {
    fn new() -> Self {
        Self {
            ctx: WasiCtxBuilder::new().inherit_stdout().build_p1(),
        }
    }
}

impl Host for MyHost {}

impl HostContext for MyHost {
    fn request_id(&mut self, self_: wasmtime::component::Resource<Context>) -> i64 {
        println!(
            "request-id for owned({}) and rep=({})",
            self_.owned(),
            self_.rep()
        );
        123
    }

    fn drop(&mut self, rep: wasmtime::component::Resource<Context>) -> wasmtime::Result<()> {
        println!("drop rep={}", rep.rep());
        Ok(())
    }
}

impl IoView for MyHost {
    fn table(&mut self) -> &mut wasmtime_wasi::ResourceTable {
        self.ctx.table()
    }
}

impl WasiView for MyHost {
    fn ctx(&mut self) -> &mut WasiCtx {
        self.ctx.ctx()
    }
}
