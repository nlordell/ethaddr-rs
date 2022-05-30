# Implementation of Ethereum public addresses for Rust.

This crate provides an `Address` type for representing Ethereum public
addresses.

## Usage

Just add a dependency to your `Cargo.toml`:

```toml
[dependencies]
ethaddr = "*"
```

For complete documentation checkout [`docs.rs`](https://docs.rs/ethaddr).

## Features

This crate provides a few features for fine-grained control of what gets
included with the crate.

> I want `#[no_std]`!

```toml
[dependencies]
ethaddr = { version = "*", default-features = false, features = ["checksum"] }
```

> I don't want to build an additional dependency for address checksums!

```toml
[dependencies]
ethaddr = { version = "*", default-features = false }
```

> I want all the bells and whisles, including a macro for compile-time verified
> address literals and `serde` support!

```toml
[dependencies]
ethaddr = { version = "*", features = ["macros", "serde"] }
```
