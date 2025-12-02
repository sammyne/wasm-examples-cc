.PHONY: all
all: run

build-guest: guest/out/app.wasm

build-host:
	cd host-rs && cargo build -r

guest/out/app.wasm:
	make -C guest

run: guest/out/app.wasm build-host
	../../target/release/expose-host-api-host -p $< -n sammyne

$(OUT_DIR):
	mkdir -p $(OUT_DIR)

.PHONY: clean
clean:
	make -C guest clean
	cd host-rs && cargo clean
