use wasmtime_wasi::{preview1::WasiP1Ctx, IoView, WasiCtx, WasiCtxBuilder, WasiView};

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
    fn inspect(&mut self, addr: u64, alloc_or_free: bool) {
        println!("addr={addr},alloc-or-free={alloc_or_free}");
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
