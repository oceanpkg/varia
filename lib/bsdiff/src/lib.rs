//! # Varia: `bsdiff`
//!
//! A [`bsdiff`] implementation for [Varia], brought to you by
//! [@NikolaiVazquez]!
//!
//! This library is developed as part of the [Ocean Package Manager][Ocean].
//!
//! The binary diffing algorithm by Colin Percival is described in his
//! ["Naïve Differences of Executable Code"][paper] paper.
//!
//! ## Usage
//!
//! This crate is available [on crates.io][crate] and can be used by adding the
//! following to your project's [`Cargo.toml`]:
//!
//! ```toml
//! [dependencies]
//! varia-bsdiff = "0.0.0"
//! ```
//!
//! and this to your crate root (`main.rs` or `lib.rs`):
//!
//! ```rust
//! extern crate varia_bsdiff;
//! ```
//!
//! This last step is optional in [Rust 2018 edition][2018].
//!
//! ## License
//!
//! Varia is released under either:
//!
//! - [MIT License](https://github.com/oceanpkg/varia/blob/master/LICENSE-MIT)
//! - [Apache License (Version 2.0)](https://github.com/oceanpkg/varia/blob/master/LICENSE-APACHE)
//!
//! at your choosing.
//!
//! [`Cargo.toml`]: https://doc.rust-lang.org/cargo/reference/manifest.html
//! [2018]: https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html#rust-2018
//! [crate]: https://crates.io/crates/varia-bsdiff
//!
//! [@NikolaiVazquez]: https://twitter.com/NikolaiVazquez
//! [Ocean]: https://www.oceanpkg.org
//!
//! [`bsdiff`]: http://www.daemonology.net/bsdiff
//! [paper]: http://www.daemonology.net/papers/bsdiff.pdf
//! [Varia]: https://github.com/oceanpkg/varia

#![deny(missing_docs)]
