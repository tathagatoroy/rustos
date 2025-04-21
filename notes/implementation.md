# Implementation Notes

By default, the linker in Rust assumes an operating system and certain other details. For example, on my system, it assumes Linux x64 and defaults to a C runtime. However, since we are building an OS, we need a barebones target that assumes no OS. Such a bare-metal environment can be described using the `thumbv7em-none-eabihf` target triple, which is for an embedded ARM system.

To set this up, run the following:

```bash
rustup target add thumbv7em-none-eabihf
```

To use the nightly version to access certain experimental features, run:

```bash
rustup override set nightly
```

One needs to recompile `core` to use it in a bare-metal environment, as the existing installation is precompiled for the OS. To do that, add the following to `.cargo/config.toml` in the root directory:

```toml
[unstable]
build-std = ["core", "compiler_builtins"]
```

Then, run:

```bash
rustup component add rust-src
```

to attach a bootable to our kernel we need to use bootimage tool 
go to your home directory and run 

```bash 
cargo install bootimage
```

to run this we need to add llvm-tools-preview. Go to project root and run -> 
```bash
rustup component add llvm-tools-preview
```

then to create bootable disk image go to your project root and run 
```bash 
cargo bootimage 
```


Now, to build in this environment, run:

```bash
cargo build --target x86_64_rustos_config.json
```

---

## Documentation

### Build Configuration for the Kernel

### Configuration File Explanation (`x86_64_rustos_config.json`)

This configuration file defines a custom target for building a Rust project for a bare-metal `x86_64` architecture. Below is an explanation of each field and its purpose:

#### 1. **`llvm_target`**
   - **Description**: Specifies the LLVM target triple for the architecture.
   - **Value**: `"x86_64-unknown-none"`
   - **Purpose**: Indicates that the target is for a 64-bit x86 architecture with no operating system (`none`).

#### 2. **`data-layout`**
   - **Description**: Defines the memory layout for the target, including endianness, pointer sizes, and alignment.
   - **Value**: `"e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"`
   - **Purpose**: Ensures compatibility with the x86_64 architecture and specifies how data is laid out in memory.

#### 3. **`arch`**
   - **Description**: Specifies the target architecture.
   - **Value**: `"x86_64"`
   - **Purpose**: Indicates that the target is for the 64-bit x86 architecture.

#### 4. **`target-endian`**
   - **Description**: Specifies the endianness of the target.
   - **Value**: `"little"`
   - **Purpose**: Indicates that the target uses little-endian byte order, which is standard for x86 architectures.

#### 5. **`target-pointer-width`**
   - **Description**: Specifies the width of pointers in bits.
   - **Value**: `"64"`
   - **Purpose**: Indicates that the target uses 64-bit pointers, consistent with the x86_64 architecture.

#### 6. **`target-c-int-width`**
   - **Description**: Specifies the width of C `int` types in bits.
   - **Value**: `"32"`
   - **Purpose**: Ensures compatibility with C code, where `int` is typically 32 bits on x86_64.

#### 7. **`os`**
   - **Description**: Specifies the operating system for the target.
   - **Value**: `"none"`
   - **Purpose**: Indicates that the target does not rely on an operating system, suitable for bare-metal development.

#### 8. **`executables`**
   - **Description**: Indicates whether the target produces an executable binary.
   - **Value**: `true`
   - **Purpose**: Specifies that the output is an executable, not a library or other artifact.

#### 9. **`linker-flavor`**
   - **Description**: Specifies the type of linker to use.
   - **Value**: `"ld.lld"`
   - **Purpose**: Uses the LLVM linker (`lld`) for linking, which is fast and supports bare-metal targets.

#### 10. **`linker`**
   - **Description**: Specifies the linker binary to use.
   - **Value**: `"rust-lld"`
   - **Purpose**: Ensures that the Rust-provided `lld` linker is used for compatibility with the target.

#### 11. **`panic-strategy`**
   - **Description**: Defines the behavior when a panic occurs.
   - **Value**: `"abort"`
   - **Purpose**: Ensures that the program aborts on a panic, as unwinding is not supported in a `no_std` environment.

#### 12. **`disable-redzone`**
   - **Description**: Disables the red zone in the stack.
   - **Value**: `true`
   - **Purpose**: Prevents the use of the red zone, which is a small area of stack memory below the stack pointer. This is important for bare-metal systems where interrupts may overwrite this memory.

#### 13. **`features`**
   - **Description**: Specifies CPU features to enable or disable.
   - **Value**: `"-mmx,-sse,+soft-float"`
   - **Purpose**: Disables MMX and SSE instructions and enables software floating-point operations, which may be necessary for environments without hardware floating-point support.

---

### Why Use This Configuration?

This configuration is tailored for bare-metal development on the `x86_64` architecture. It ensures compatibility with the hardware while avoiding dependencies on an operating system or standard library. The settings are optimized for low-level programming, where control over memory layout, stack behavior, and floating-point operations is critical.