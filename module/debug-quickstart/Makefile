debugger = debugger/target/release/debugger

fibcc = fib-cc/build/fib-cc.wasm

.PHONY: all
all: $(debugger) $(fibcc)

$(debugger): debugger/src/main.rs
	cd debugger && cargo build -r

$(fibcc): fib-cc/lib.cc
	$(MAKE) -C fib-cc

lldb: $(debugger) $(fibcc)
	lldb -O 'settings set target.disable-aslr false' \
		-o 'breakpoint set -n main' \
		-o 'r' \
		-o "breakpoint set -n 'fib'" \
		-o 'c' \
		-o 'n' \
		-o 'n' \
		-o 'n' \
		-o 'expr (void)__vmctx->set()' \
		-o 'p *h.ptr' \
		-o 'expr *(char(*)[12])(&(__vmctx.memory[1024]))' \
		-o 'p *(char(*)[12])(&*h.ptr)' \
		-- $(debugger) $(fibcc)

.PHONY: clean
clean:
	cd debugger && cargo clean
	$(MAKE) -C fib-cc clean