# mokuya

[![Crates.io](https://img.shields.io/crates/v/mokuya.svg)](https://crates.io/crates/mokuya)
[![Docs.rs](https://docs.rs/mokuya/badge.svg)](https://docs.rs/mokuya)
[![License](https://img.shields.io/crates/l/mokuya.svg)](https://github.com/pas2rust/mokuya/blob/master/LICENSE.md)

`mokuya` is a utility crate that provides adapters and helpers for writing Rust procedural macros. It simplifies common tasks when working with `syn`, `quote`, and `proc_macro2`, and is designed to help authors build robust and reusable macro code.

## Features

- Utilities for extracting types, arguments, and attributes from `syn`.
- Simple extension utilities for `TokenStream`.
- Reusable and ergonomic error handling structure.
- Great for use in derive, attribute, or function-like procedural macros.
- Zero dependencies outside the core macro ecosystem.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
mokuya = "0.1.0"
