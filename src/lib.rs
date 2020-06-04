/*!
This crate provides `has_impl!(type: Trait)` macro to check if `Trait` is implemented for `type`
at complie time

```rust
use has_impl::*;

trait Foo {}

impl Foo for i32 {}

fn main() {
	assert_eq!(has_impl!(i32: Foo), true);
}
```
*/

#![no_std]

#[macro_export]
macro_rules! has_impl {
	($ty:ty: $tr:path) => {
		{
			struct Test<T: ?Sized>(core::marker::PhantomData<T>);

			#[allow(dead_code)]
			impl<T: ?Sized + $tr> Test<T> {
				const HAS_IMPL: bool = true;
			}

			trait Fallback {
				const HAS_IMPL: bool = false;
			}

			impl<T: ?Sized> Fallback for T {}

			Test::<$ty>::HAS_IMPL
		}
	};
}

#[cfg(test)]
mod tests {
	trait Foo {}

	impl Foo for i32 {}

	trait Bar {}

	#[test]
	fn basic() {
		assert_eq!(has_impl!(i32: Foo), true);
		assert_eq!(has_impl!(i32: Bar), false);
	}

	mod path {
		pub(crate) trait Trait1 {}

		pub(crate) trait Trait2 {}
	}

	impl path::Trait1 for i32 {}

	#[test]
	fn with_path() {
		assert_eq!(has_impl!(i32: path::Trait1), true);
		assert_eq!(has_impl!(i32: path::Trait2), false);
	}
}
