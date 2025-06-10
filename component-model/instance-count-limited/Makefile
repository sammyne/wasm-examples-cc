.PHONY: all
all: run

build-guest: guest/out/app.wasm

build-host:
	cd host-rs && cargo build -r

fmt:
	cd host-rs && cargo fmt

guest/out/app.wasm: guest/src/lib.cc guest/Makefile
	make -C guest

run: guest/out/app.wasm build-host
	../../target/release/host-trap-by-oom -p $<

$(OUT_DIR):
	mkdir -p $(OUT_DIR)

.PHONY: clean
clean:
	make -C guest clean
	cd host-rs && cargo clean
