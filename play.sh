#!/bin/bash

cd `dirname ${BASH_SOURCE[0]}`

repo_tag=sammyne/wasm-studio-cc:9543549
cargo_home=$PWD/_cargo
workdir=/github.com/sammyne/wasm-examples-cc

name=github-wasm-studio-cc

docker rm -f $name

docker run -td --rm                     \
  --name $name          \
  -e CARGO_HOME=/root/.cargo      \
  -v $PWD:$workdir                \
  -v $cargo_home/registry:/root/.cargo/registry   \
  -v $cargo_home/git:/root/.cargo/git             \
  -w $workdir                     \
  $repo_tag bash

docker exec -it $name bash
