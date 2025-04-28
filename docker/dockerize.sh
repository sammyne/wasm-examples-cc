#!/bin/bash

set -e

build_arg_opts="$build_arg_opts --build-arg WASI_SDK_MAJOR_VERSION=25"
build_arg_opts="$build_arg_opts --build-arg WASI_SDK_MINOR_VERSION=0"
build_arg_opts="$build_arg_opts --build-arg WASMTIME_VERSION=30.0.0"
build_arg_opts="$build_arg_opts --build-arg WASM_TOOLS_VERSION=1.227.0"
build_arg_opts="$build_arg_opts --build-arg WIT_BINDGEN_VERSION=0.40.0"

repo_tag=sammyne/wasm-studio-cc:`git rev-parse --short HEAD`

docker build $build_arg_opts -t $repo_tag .

#docker push $repo_tag
