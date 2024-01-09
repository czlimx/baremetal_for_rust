#!/bin/bash
cargo build     --target aarch64-unknown-none
cargo objdump   --target aarch64-unknown-none --bin demo -- --disassemble --no-show-raw-insn > target/aarch64-unknown-none/debug/demo.dis
cargo objcopy   --target aarch64-unknown-none --bin demo -- -O binary target/aarch64-unknown-none/debug/demo.hex
cargo size      --target aarch64-unknown-none --bin demo
