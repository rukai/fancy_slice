# Fancy Slice
[![Build Status](https://travis-ci.com/rukai/fancy_slice.svg?branch=master)](https://travis-ci.com/rukai/fancy_slice) [![dependency status](https://deps.rs/repo/github/rukai/fancy_slice/status.svg)](https://deps.rs/repo/github/rukai/fancy_slice) [![Crates.io](https://img.shields.io/crates/v/fancy_slice.svg)](https://crates.io/crates/fancy_slice) [![Docs](https://docs.rs/fancy_slice/badge.svg)](https://docs.rs/fancy_slice)

Wraps an `&[u8]` slice to provide a kitchen sink worth of tools.
Useful for writing a binary format parser that needs to be reverse engineered as you go.

Mostly untested, so gauranteed to have off-by-one errors. :P

Enable the `debug` feature to add extra functions.
You should only enable the debug feature during development as it comes with a performance hit due to storing extra data in FancySlice.

```rust
use fancy_slice::FancySlice;

fn main() {
    let data = vec!(4, 1, 3);
    let fancy_slice = FancySlice::new(&data);
    assert_eq!(fancy_slice.u8(0), 4);
    assert_eq!(fancy_slice.u8(1), 1);
    assert_eq!(fancy_slice.u8(2), 3);
    assert_eq!(fancy_slice.u16_be(0), 0x0401);
    assert_eq!(fancy_slice.u16_be(1), 0x0103);

    let inner_fancy_slice = fancy_slice.relative_fancy_slice(1..);
    assert_eq!(inner_fancy_slice.u8(0), 1);
    assert_eq!(inner_fancy_slice.u8(1), 3);
    assert_eq!(inner_fancy_slice.u16_be(0), 0x0103);
}
```
