.PHONY: all
all: run

build-guest: guest/out/app.wasm

build-host:
	cd host-rs && cargo build -r

guest/out/app.wasm:
	make -C guest

name='alice'

# 5 是 name 的长度
lldb: guest/out/app.wasm build-host
	lldb -O 'settings set target.disable-aslr false' \
		-o 'breakpoint set -n main' \
		-o 'r' \
		-o 'breakpoint set -n __wasm_export_exports_helloworld_say_hello_again' \
		-o 'c' \
		-o 'expr (void)__vmctx->set()' \
		-o 'n' \
		-o 'p *(char(*)[5])(&*arg1.name.ptr)' \
		-o 'q' \
		-- host-rs/target/release/host -p $< -n $(name)

run: guest/out/app.wasm build-host
	host-rs/target/release/host -p $< -n sammyne

$(OUT_DIR):
	mkdir -p $(OUT_DIR)

.PHONY: clean
clean:
	make -C guest clean
	cd host-rs && cargo clean
