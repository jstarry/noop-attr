# noop-attr
Rust proc macro that does nothing

A use-case wrapping a feature switched macro:
```rust
#[cfg(feature = "enable")]
extern crate real_impl;
#[cfg(feature = "enable")]
pub use self::real_impl::real_attr as wrapped_attr;

#[cfg(not(feature = "enable"))]
extern crate noop_attr;
#[cfg(not(feature = "enable"))]
pub use self::noop_attr::noop as wrapped_attr;
```
