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

### Setup
```bash
rustup target add thumbv7em-none-eabihf
```

From [Writing an OS in Rust](https://os.phil-opp.com/freestanding-rust-binary/)
"An example of such a bare metal environment is the thumbv7em-none-eabihf target triple, which describes an embedded ARM system. The details are not important, all that matters is that the target triple has no underlying operating system, which is indicated by the none in the target triple. To be able to compile for this target, we need to add it in rustup"




To build the project for the target architecture, run:
```bash
cargo build --target thumbv7em-none-eabihf
```
