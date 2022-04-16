# arith_wrappers

Rust's `std` offers the [`Wrapping`](https://doc.rust-lang.org/std/num/struct.Wrapping.html) type for "intentionally 
wrapping" arithmetic, but curiously does not provide analogous implementations for intentionally saturating, checked, 
overflowing or panicking arithmetic.  This crate addresses these missing wrappers as well as exports an alternative to 
`std`'s [`Wrapping`](https://doc.rust-lang.org/std/num/struct.Wrapping.html), providing a consistent interface across 
all wrappers in this crate as well as some benefits like `trait`-enforced access to `T::MIN` and `T::MAX` where `T`
implements any of the wrappers defined in this crate.

Without these wrappers, performing correct arithmetic is verbose, error-prone and pedantic, leading
to ignoring the problems altogether or hard-to-maintain implementations.

This crate is at a very early pre-alpha stage of implementation and should be considered a PoC.

## License
Licensed under either:
* MIT license (see LICENSE-MIT file)
* Apache License, Version 2.0 (see LICENSE-APACHE file)
at your option.

## Contributions
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you shall be dual licensed as above, without any additional terms or conditions.
