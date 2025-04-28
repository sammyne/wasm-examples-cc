# Debug module

## 1. 快速开始
```bash
make lldb
```

## 注意事项
- 可设置断点的符号是 C 的函数名，而不是 `__attribute__((__export_name__("xxx")))`/`__attribute__((export_name("xxx")))` 
属性添加的顾符号。
- `WebAssemblyPtrWrapper<const 具体类型>` 的变量指针 `x` 可使用 `&*x.ptr` 获取其指针地址
