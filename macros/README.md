# Ethereum Public Address Literals

This crate provides a procedural macro for compile-time verified Ethereum
address literals.

This is typically not used directly, but instead included with `ethaddr`:

```toml
[dependencies]
ethaddr = { version = "*", features = ["macros"] }
```
