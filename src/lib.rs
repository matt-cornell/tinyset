// Copyright 2017-2018 David Roundy
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! `tinyset` contains a few collections that are optimized to scale
//! in size well for small numbers of elements, while still scaling
//! well in time (and size) for numbers of elements.  We now have
//! just a few types that you might care for.
//!
//! 1. [`Set64`] is a set for types that are 64 bits in size or less
//! and are `Copy`, intended for essentially integer types.  This is
//! our most efficient type, since it can store small sets with just
//! the size of one pointer, with no heap storage.
//!
//! 2. [`SetU64`] just holds `u64` items, and is the internal storage
//! of [`Set64`].
//!
//! 3. [`SetU32`] just holds `u32` items, and uses a bit less memory
//! than [`SetU64`].
//!
//! 4. [`SetUsize`] holds `usize` items, and uses either [SetU64] or
//! [SetU32] internally.
//!
//! All of these set types will do no heap allocation for small sets of
//! small elements.  `TinySet` will store up to 16 bytes of elements
//! before doing any heap allocation, while `Set` stores sets up to size 8
//! without allocation.  `Set64` will store up to 22 bytes of elements,
//! and if all your elements are small (e.g. `0..22 as u64` it will store
//! them in as few bytes as possible.
//!
//! These sets all differ from the standard sets in that they iterate
//! over items rather than references to items, because they do not
//! store values directly in a way that can be referenced.  All of the
//! type-specific sets further differ in that `remove` and `contains`
//! accept values rather than references.
//!
//! # Examples
//!
//! ```
//! use tinyset::Set64;
//! let mut s: Set64<usize> = Set64::new();
//! s.insert(1);
//! assert!(s.contains(&1));
//! ```

#![deny(missing_docs)]

#[doc(hidden)]
pub mod setu32b;

pub mod setusize;
pub use setusize::SetUsize;

pub mod setu32;
pub use setu32::SetU32;

pub mod setu64;
pub use setu64::SetU64;

pub mod set64;
pub use crate::set64::{Set64, Fits64};

mod copyset;
