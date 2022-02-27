#!/usr/bin/env sh

set -e

# 更新算法核心库
cargo update -p yarism
# 构建PKG
wasm-pack build --out-name index
# 测试
wasm-pack test --headless --firefox


