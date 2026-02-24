# Installation

## Binaries
In order to install the `marc21` binary, [archives with a precompiled
binary] are available for Windows, macOS and Linux.

Two variants are available for Linux: a dynamically linked version and
a fully statically linked version (MUSL). In most cases, the statically
linked version should be preferred, as it is independent of the glibc
version of the host system. The following commands install the binary
into the `/usr/local/bin` directory:

```console
$ tar xfz marc21-0.1.0-x86_64-unknown-linux-musl.tar.gz
$ sudo install -Dm755 marc21-0.1.0-x86_64-unknown-linux-musl/marc21 \
   /usr/local/bin/marc21
```

## From Source

If a Rust toolchain is available, `marc21` can also be installed using
the Rust package manager [cargo]. The project requires a Rust compiler
with a minimum version of 1.93. Use the following command to install
the program with the default features:

```console
$ cargo install marc21-cli  
```

The binary can be built with the following features as needed:

`performant`
: This feature activates optimizations aimed at improving performance.
This includes, for example, the activation of SIMD or a more aggressive
inline strategy. Since the main goal of the project is high performance,
the feature is enabled by default.

`unstable`
: New features that are still in the testing phase can be activated
using the `unstable` feature. Keep in mind that these functions may
change at any time.

[archives with a precompiled binary]: https://github.com/deutsche-nationalbibliothek/marc21-rs/releases
[cargo]: https://doc.rust-lang.org/cargo/
