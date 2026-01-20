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
[dummy-static] addr(out)=0xf84, addr(out.c_str())=0xf84)
[dummy-non-static] addr(out)=0xf90, addr(out.c_str())=0xf90)

#0 say-hello
malloc(24)=0x31720
addr(HELLO_WORLD)=0xf84, HELLO_WORLD='dummy'
malloc(24)=0x31740
free(0x31720)
free(0x31760)
say-hello returns [Record([("message", String("Hello sammyne"))])]

#1 say-hello
malloc(24)=0x31760
addr(HELLO_WORLD)=0x31740, HELLO_WORLD='Hello sammyne :)'
malloc(24)=0x31780
free(0x31740)
free(0x31760)
free(0x31730)
say-hello returns [Record([("message", String("Hello sammyne"))])]

#2 say-hello
malloc(24)=0x31740
addr(HELLO_WORLD)=0x31780, HELLO_WORLD='Hello sammyne :)'
malloc(24)=0x31760
free(0x31780)
free(0x31740)
free(0x31780)
say-hello returns [Record([("message", String("Hello sammyne"))])]
```

## 参考文献
- [How to wrap functions with the `--wrap` option correctly?](https://stackoverflow.com/a/46446749/10878967)
- https://sourceware.org/binutils/docs/ld/Options.html#index-g_t_002d_002dwrap
- https://component-model.bytecodealliance.org/language-support/c.html

