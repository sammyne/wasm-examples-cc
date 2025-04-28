#!/bin/bash

cd `dirname ${BASH_SOURCE[0]}`

repo_tag=sammyne/wasm-studio-cc:9543549
cargo_home=$PWD/_cargo
workdir=/workspace

#docker run --name wasm-studio-cc -td --rm -v $PWD:/workspace -w /workspace sammyne/wasm-studio-cc:fe8c204 bash

docker run -it --rm                     \
  --name github-wasm-studio-cc          \
  -e CARGO_HOME=/root/.cargo      \
  -v $PWD:$workdir                \
  -v $cargo_home/registry:/root/.cargo/registry   \
  -v $cargo_home/git:/root/.cargo/git             \
  -w $workdir                     \
  $repo_tag bash