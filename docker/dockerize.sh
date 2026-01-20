#!/bin/bash

set -e

build_arg_opts="$build_arg_opts --build-arg WASI_SDK_MAJOR_VERSION=22"
build_arg_opts="$build_arg_opts --build-arg WASI_SDK_MINOR_VERSION=0"
build_arg_opts="$build_arg_opts --build-arg WASMTIME_VERSION=20.0.1"

# 1.212.0 版的 wasm-tools 将 wasip1 的组件适配为 wasip2 时有问题，导致 C++ 的全局变量没能正常初始化。
build_arg_opts="$build_arg_opts --build-arg WASM_TOOLS_VERSION=1.227.0"

build_arg_opts="$build_arg_opts --build-arg WIT_BINDGEN_VERSION=0.40.0"
build_arg_opts="$build_arg_opts --build-arg RUST_TOOLCHAIN=1.85.0"

repo_tag=sammyne/wasm-studio-cc:wt20-ws22-rs85-`git rev-parse --short HEAD`

docker build $build_arg_opts -t $repo_tag .

docker push $repo_tag
