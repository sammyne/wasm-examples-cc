
OUT_DIR := out
BINDGEN_DIR := $(OUT_DIR)/bindgen

WASIP2_ADAPTER_PATH := /opt/wasmtime/adapter-modules/wasi_snapshot_preview1.reactor.wasm
WORLD_NAME := helloworld

C = /opt/wasi-sdk/bin/wasm32-wasip2-clang
CFLAGS := -fno-exceptions -Wall -I$(BINDGEN_DIR)

CC := /opt/wasi-sdk/bin/wasm32-wasip2-clang++
CCFLAGS := -std=c++20 -fno-exceptions -Wall -I$(BINDGEN_DIR)
LDFLAGS := -mexec-model=reactor

all: $(OUT_DIR)/app.wasm

$(OUT_DIR):
	mkdir -p $(OUT_DIR)

$(BINDGEN_DIR)/$(WORLD_NAME).o: $(BINDGEN_DIR)/$(WORLD_NAME).c
	$(C) $(CFLAGS) -c $< -o $@

$(OUT_DIR)/app.wasm: src/lib.cc $(BINDGEN_DIR) $(OUT_DIR) $(BINDGEN_DIR)/$(WORLD_NAME).o
	$(CC) $(CCFLAGS) $< $(BINDGEN_DIR)/$(WORLD_NAME).o $(BINDGEN_DIR)/$(WORLD_NAME)_component_type.o -o $@ -mexec-model=reactor

$(BINDGEN_DIR): wit/world.wit $(OUT_DIR)
	@rm -rf $@
	@mkdir $@
	@wit-bindgen c --out-dir $@ $<

.PHONY: clean
clean:
	rm -rf $(BINDGEN_DIR) $(OUT_DIR)
