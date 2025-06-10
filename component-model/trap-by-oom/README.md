# Trap by OOM

演示 WASM 内部异常（例如 OOM）会导致 host 侧的实例句柄不可用。

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

