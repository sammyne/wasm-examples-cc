# 内存监控器

演示多次函数调用复用 `Store` 的情况下，静态和非静态的全局变量
1. 只初始化一次；
1. 不会被析构；

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
[dummy-static] addr(out)=0xf34, addr(out.c_str())=0xf34)
[dummy-non-static] addr(out)=0xf40, addr(out.c_str())=0xf40)

#0 say-hello
malloc(24)=0x316d0
HELLO_WORLD.c_str=0xf34
free(0x316d0)
free(0x316f0)
say-hello returns [Record([("message", String("Hello sammyne"))])]

#1 say-hello
malloc(24)=0x316e0
HELLO_WORLD.c_str=0xf34
free(0x316e0)
free(0x31700)
say-hello returns [Record([("message", String("Hello sammyne"))])]
```

## 参考文献
- [How to wrap functions with the `--wrap` option correctly?](https://stackoverflow.com/a/46446749/10878967)
- https://sourceware.org/binutils/docs/ld/Options.html#index-g_t_002d_002dwrap
- https://component-model.bytecodealliance.org/language-support/c.html

