has_impl
========
[![Build Status](https://travis-ci.org/Frago9876543210/has_impl.svg?branch=master)](https://travis-ci.org/Frago9876543210/has_impl)
[![Latest Version](https://img.shields.io/crates/v/has_impl.svg)](https://crates.io/crates/has_impl)
[![Documentation](https://docs.rs/has_impl/badge.svg)](https://docs.rs/has_impl/)

Check if trait is implemented for type at compile time

### Example

```rust
use has_impl::*;

trait Foo {}

impl Foo for i32 {}

fn main() {
	assert_eq!(has_impl!(i32: Foo), true);
}
```

### Installing
```toml
[dependencies]
has_impl = "0.1"
```
