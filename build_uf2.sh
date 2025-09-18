#!/bin/sh

cargo build --release --target thumbv6m-none-eabi

elf2uf2-rs target/thumbv6m-none-eabi/release/myrp target/myrp.uf2
