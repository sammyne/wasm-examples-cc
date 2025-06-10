# hello-world

演示 wasmtime 的 `Store` 内部有追踪创建的实例、表、资源等总数，且对这个总数有默认上限，相关实例被内存回收时也不会将这个计数减少。

相关 issue 参见 https://github.com/bytecodealliance/wasmtime/issues/2751。

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

## 参考文献
- https://component-model.bytecodealliance.org/language-support/c.html

