# 内存监控器

演示基于连接器 wasm-ld（和 GNU ld 类似）的 `-Wl,--wrap` 选项在真正的 `malloc/free` 调用之前插入内存监控逻辑，一旦插入的逻辑调用 host 接口会失败的问题。

## 环境
- 参见项目根目录的 [Dockerfile](../../docker/Dockerfile)。

## 简介

文件 | 说明
-----|-------
guest | C++ 编写的 WASM 组件源码
host-rs | Rust 编写用于调用 guest 组件的逻辑

## 快速开始
```bash
make run
```

样例输出如下
```bash
Error: call

Caused by:
    0: error while executing at wasm backtrace:
           0: 0x29cd - greeter.wasm!sammyne_hellohost_api_inspect
           1:  0xe52 - greeter.wasm!inspect(void*, bool)
           2:  0xfed - greeter.wasm!__wrap_realloc
           3: 0x2838 - greeter.wasm!cabi_realloc
       note: using the `WASMTIME_BACKTRACE_DETAILS=1` environment variable may show more debugging information
    1: cannot leave component instance
make: *** [Makefile:13: run] Error 1
```

## TODO
- 解决 `__wrap_realloc` 加打印日志会导致失败的问题

## 参考文献
- [How to wrap functions with the `--wrap` option correctly?](https://stackoverflow.com/a/46446749/10878967)
- https://sourceware.org/binutils/docs/ld/Options.html#index-g_t_002d_002dwrap
- https://component-model.bytecodealliance.org/language-support/c.html

