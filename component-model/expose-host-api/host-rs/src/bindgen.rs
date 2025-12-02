use wasmtime_wasi::{IoView, WasiCtx, WasiCtxBuilder, WasiView, preview1::WasiP1Ctx};

use crate::bindgen::sammyne::hellohost::api::Host;

wasmtime::component::bindgen!({
    world: "hellohost-world",
    path: "wit",
});

pub struct MyHost {
    ctx: WasiP1Ctx,
}

impl MyHost {
    pub fn new() -> Self {
        Self {
            ctx: WasiCtxBuilder::new().inherit_stdout().build_p1(),
        }
    }
}

impl Host for MyHost {
    fn new_greeting(&mut self, who: String) -> String {
        format!("hello {who}")
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