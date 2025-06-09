#!/bin/bash

cd `dirname ${BASH_SOURCE[0]}`

repo_tag=sammyne/wasm-studio-cc:02b7b1a

cache_dir=$PWD/_cache
workdir=/github.com/sammyne/wasm-examples-cc

volume_opts="-v $PWD:$workdir"
if [[ -d $PWD/../.. ]]; then
  volume_opts="$volume_opts -v $PWD/../..:/github.com"
fi

name=github-wasm-studio-cc

docker rm -f $name

docker run -td --rm                                   \
  --name $name                                        \
  -e CARGO_HOME=$workdir/_cache/cargo                 \
  -e SCCACHE_CONF=$workdir/_cache/sccache/config.toml \
  $volume_opts                                        \
  -w $workdir                                         \
  $repo_tag bash

if [ -d $cache_dir/ssh ]; then
  docker cp $cache_dir/ssh $name:/root/.ssh
fi

if [ -f $cache_dir/gitconfig ]; then
  docker cp $cache_dir/gitconfig $name:/root/.gitconfig
fi

docker exec -it $name bash
