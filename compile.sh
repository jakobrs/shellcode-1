#!/usr/bin/env bash

args=(
  +nightly                            # Unstable features
  -C opt-level=z                      # Optimize for size
  -C panic=abort                      # We really don't want to use the unwinding machinery
  -C link-arg=-nostartfiles           # Don't include libc stuff
  -C llvm-args=-x86-asm-syntax=intel  # obvious
  --cfg include_panic_handler         # Workaround rust-analyzer being weird
  -C relocation-model=pic             # Generate position-independent code (static also
                                      # works if you know where the code will be loaded)
  -C target-feature=+crt-static       # unsure if this does anything here
  --target=x86_64-unknown-none        # unsure if this does anytihng here either
  -C linker=gcc                       # Use gcc's linker
  -v                                  # Verbose
  -C overflow-checks=off              # Obvious
  -C lto=true                         # LTO
  -C incremental=false                # Incremental build scan break things
  -C codegen-units=1                  # parallel builds can theoretically produce suboptimal code
  -C link-args=-Tscript.ld            # Use custom linker script
)

rustc "${args[@]}" "$@"
