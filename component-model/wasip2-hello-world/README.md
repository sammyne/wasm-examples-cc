# hello-world

演示一个简单的 C++-WASM 组件的构建。

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

## 注意事项
- 与 ../hello-world 不同，这个示例不再需要显式调用 wasm-tools 将 module 组件化
- wasi-sdk 25.0/22.0 的 wasm32-wasip2-clang++ 编译 c 源文件得到的 wasm 文件无法被正确识别为 wasm component，具体报错如下
    ```bash
    /opt/wasi-sdk/bin/wasm32-wasip2-clang++ -std=c++20 -fno-exceptions -Wall -Iout/bindgen src/lib.cc out/bindgen/helloworld.c out/bindgen/helloworld_component_type.o -o out/app.wasm -mexec-model=reactor
    wasm32-wasip2-clang++: warning: treating 'c' input as 'c++' when in C++ mode, this behavior is deprecated [-Wdeprecated]
    error: failed to encode component

    Caused by:
        0: failed to decode world from module
        1: module was not valid
        2: duplicate export name `cabi_realloc` already defined (at offset 0x3e8)
    wasm32-wasip2-clang++: error: linker command failed with exit code 1 (use -v to see invocation)
    ```

## 参考文献
- https://component-model.bytecodealliance.org/language-support/c.html

