pub use self::fancy_slice::*;

#[cfg(feature = "debug")]
#[path = "debug.rs"]
mod fancy_slice;

#[cfg(not(feature = "debug"))]
#[path = "simple.rs"]
mod fancy_slice;
