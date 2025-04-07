# rustos

I am learning Rust by building an operating system kernel in Rust. This project is a hands-on exploration of low-level programming and systems development using Rust's powerful features.

## About the Project

This project is inspired by and follows the excellent tutorial series by Philipp Oppermann:  
[Writing an OS in Rust](https://os.phil-opp.com/freestanding-rust-binary/)

The goal is to create a minimal, freestanding Rust binary that can serve as the foundation for an operating system kernel.

## Current Progress

- Set up a bare-metal Rust environment.
- Configured the project for a `no_std` environment.
- Exploring kernel development concepts.

## How to Build

To build the project for the target architecture, run:
```bash
cargo build --target thumbv7em-none-eabihf
