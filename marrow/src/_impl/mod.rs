//! Internal do not use
#[macro_export]
#[doc(hidden)]
macro_rules! _with_arrow {
    ($($tt:tt)*) => {
        // arrow-version: replace:         #[cfg(feature = "arrow-53")]
        #[cfg(feature = "arrow-53")]
        {
            use $crate::_impl::arrow;
            $($tt)*
        }
    };
}

// arrow-version: replace: #[cfg(feature = "arrow-{version}")]
#[cfg(feature = "arrow-53")]
#[doc(hidden)]
pub mod arrow;
