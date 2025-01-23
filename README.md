# Crasher

Tries to crash your pc!

### Prerequisites

Rust, Cargo and a C compiler.

You need to compile the C file to produce either a static or dynamic library, whose name is liballocmem.a or liballocmem.lib, etc.

The cargo build will fail without the library. You will also need to configure paths in config.toml

You can also change the library name, but you will have to change few more files.

Its time to crash some PC's!
