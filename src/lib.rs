//! Utility macros, stubs, and wrappers useful for various fuzzers and provers

/// Stub out dbg because it slows down fuzzing and tends to explode Kani
// #[allow(unused_macros)]
// #[cfg(any(kani, fuzzing))]
#[macro_export]
macro_rules! dbg {
    ($($tt:tt)*) => {};
}

// Re-export kani::cover so only one, non-cfg, import is needed
#[cfg(kani)]
pub use ::kani::cover;

#[cfg(not(kani))]
// #[allow(dead_code)]
pub mod kani {
    //! Support module for the kani fuzzer, for use in normal code,
    //! allowing for intellisense and normal static analysis
    //!
    //! Partially stubs out parts of the kani API and macros for intellisense
    //! purposes. Most stubs returning a value will panic if actually executed.
    #![macro_use]

    /// This allows using `kani::cover` without `cfg(kani)` all the time.
    ///
    /// Has limited support for rust-analyzer and cargo check.
    /// Will not actually execute anything.
    ///
    /// `kani::cover` must be `cfg(kani)` imported within modules needing it
    #[macro_export]
    macro_rules! cover {
        ($e:expr, $($tt:tt)*) => {{
            // Make rust-analyzer and warnings useful
            // #[cfg(rust_analyzer)]
            #[allow(dead_code, unused_imports)]
            let _: () = {
                // Bring our stub kani module in scope
                use $crate::kani;
                if false {
                    let _ = $e;
                }
            };
        }};
        ($($tt:tt)*) => {};
    }

    /// Any [`kani::Arbitrary`] value
    pub fn any<T>() -> T {
        todo!()
    }

    /// Assume to be true
    pub fn assume(_cond: bool) {}
}
