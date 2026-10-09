# CODEBUDDY.md

This file provides guidance to CodeBuddy Code when working with code in this repository.

## Project Overview

tarOS is a minimalist x86_64 bare-metal kernel written in Rust, following the "Writing an OS in Rust" (Philipp Oppermann) approach. It currently implements a VGA text-mode driver with custom `print!`/`println!` macros. Source comments and console output are in Indonesian/English mixed.

## Commands

Prerequisites (one-time):
```bash
rustup override set nightly
rustup component add rust-src
rustup component add llvm-tools-preview
cargo install bootimage
# QEMU must be installed and on PATH
```

Build and run in QEMU:
```bash
cargo run --target x86_64-taros.json
```

Build only:
```bash
cargo build --target x86_64-taros.json
```

Note: `.cargo/config.toml` already sets `build.target = "x86_64-taros.json"` and the runner to `bootimage runner`, so the explicit `--target` flag is technically redundant but is the documented invocation.

Testing: `bootimage` test args are configured in `Cargo.toml` (`[package.metadata.bootimage]`, `isa-debug-exit` at iobase `0xf4`), but the binary declares `test = false` and there are currently no tests. There is no test infrastructure to run yet.

## Architecture

The kernel is split into two modules with a strict `#![no_std]` / `#![no_main]` bare-metal setup.

- `src/main.rs` — Kernel entry point. `_start` is the bootloader entry (`#[unsafe(no_mangle)] extern "C"`), which prints an ASCII-art logo and loops forever. Also defines the `#[panic_handler]` (currently a bare `loop {}`). New kernel subsystems are added here as modules.
- `src/vga_buffer.rs` — VGA text-mode driver. Writes directly to the memory-mapped buffer at physical address `0xb8000`.

### Key VGA driver design points

- `Writer` holds `column_position`, current `ColorCode`, and a `&'static mut Buffer`. It is exposed globally as `WRITER: Mutex<Writer>` via `lazy_static` + `spin::Mutex` (no OS, so a spinlock, not a std mutex).
- The framebuffer pointer is an `unsafe` cast: `&mut *(0xb8000 as *mut Buffer)`.
- `Buffer` wraps `[[Volatile<ScreenChar>; 80]; 25]` — the `volatile` crate prevents the compiler from reordering/optimizing hardware-mapped memory accesses.
- `ColorCode` packs foreground (low 4 bits) and background (high 4 bits) into a single `u8` (`#[repr(transparent)]`). `ScreenChar` is `#[repr(C)]` = one ASCII byte + one color byte.
- Text is always written on the bottom row (`BUFFER_HEIGHT - 1`); `new_line()` scrolls the whole buffer up by one row and clears the last row.
- `print!` / `println!` are `#[macro_export]` macros that route through `vga_buffer::_print`, which locks `WRITER` and calls `write_fmt`.
- `write_string` maps non-printable bytes to `0xfe` (blinking block); `write_at(row, col, s)` writes at an absolute position without scrolling.

### Conventions

- `#[repr(...)]` attributes are applied deliberately for exact memory layout (see comments in `vga_buffer.rs`).
- `lazy_static` uses `spin_no_std` feature (see `Cargo.toml`); `panic = "abort"` in both dev and release profiles.
- The build uses a custom target spec (`x86_64-taros.json`) with `rust-lld`, `panic-strategy: abort`, and `disable-redzone: true`; `build-std = ["core", "compiler_builtins"]` is required via `.cargo/config.toml`.

## Roadmap (from README)

Done: VGA text buffer, printing macros.
Planned: IDT / interrupt handling, keyboard driver, paging + heap allocator, preemptive multitasking, VFS, userspace (ring 3 + syscalls).
