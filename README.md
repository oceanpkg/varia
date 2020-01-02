# Varia

A collection of diffing algorithms in Rust, brought to you by [@NikolaiVazquez]!

This library is developed as part of the [Ocean Package Manager][Ocean].

## Usage

This crate is available [on crates.io][crate] and can be used by adding the
following to your project's [`Cargo.toml`]:

```toml
[dependencies]
varia = "0.0.0"
```

and this to your crate root (`main.rs` or `lib.rs`):

```rust
extern crate varia;
```

This last step is optional in [Rust 2018 edition][2018].

## Why "Varia"?

It's the common root of the words

- "varias": the Spanish word for "various", as in "varias cosas"
- "variance": a synonym of "difference"

and this library contains various variance tools.

## License

Varia is released under either:

- [MIT License](https://github.com/oceanpkg/varia/blob/master/LICENSE-MIT)
- [Apache License (Version 2.0)](https://github.com/oceanpkg/varia/blob/master/LICENSE-APACHE)

at your choosing.

[`Cargo.toml`]: https://doc.rust-lang.org/cargo/reference/manifest.html
[2018]: https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html#rust-2018
[crate]: https://crates.io/crates/varia

[@NikolaiVazquez]: https://twitter.com/NikolaiVazquez
[Ocean]: https://www.oceanpkg.org
