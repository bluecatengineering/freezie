#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub,
    non_snake_case,
    non_upper_case_globals
)]
#![deny(rustdoc::broken_intra_doc_links)]
#![allow(clippy::cognitive_complexity)]
//! # freezie
//!
//! A wrapper type with no `DerefMut` impl, disallowing mutation
//! ```compile_fail
//! use domain_list::freeze::Freeze;
//! let mut v = Freeze::new(vec![1, 2, 3]);
//! v.push(1); // ERROR - cannot borrow data in dereference of ...
//! ```

use std::ops::Deref;

/// A wrapper type with no `DerefMut` impl, disallowing mutation
/// ```compile_fail
/// use domain_list::freeze::Freeze;
/// let mut v = Freeze::new(vec![1, 2, 3]);
/// v.push(1); // ERROR - cannot borrow data in dereference of ...
/// ```
#[derive(Copy, Debug, Clone, Default, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub struct Freeze<T>(T);

impl<T> Deref for Freeze<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Freeze<T> {
    /// construct a new "frozen" type
    pub fn new(inner: T) -> Self {
        Self(inner)
    }

    /// unfreeze the inner value
    pub fn defrost(self) -> T {
        self.0
    }
}

/// Defines how to make a type immutable
pub trait Frozen {
    /// `freeze` this type to make it immutable
    fn freeze(self) -> Freeze<Self>
    where
        Self: Sized,
    {
        Freeze::new(self)
    }
}

impl<T> Frozen for T {
    fn freeze(self) -> Freeze<T> {
        Freeze::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_from_iter() {
        let im = (0..10).collect::<Vec<_>>().freeze();
        for (&x, y) in im.iter().zip(0..10) {
            assert_eq!(x, y);
        }
    }
}
