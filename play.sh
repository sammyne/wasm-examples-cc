#!/bin/bash

docker run --name wasm-studio-cc -td --rm -v $PWD:/workspace -w /workspace sammyne/wasm-studio-cc:fe8c204 bash

