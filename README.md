[![Build Status](https://github.com/droundy/tinyset/actions/workflows/rust.yml/badge.svg)](https://github.com/droundy/tinyset/actions)
[![Build status](https://ci.appveyor.com/api/projects/status/h0rn4amvlwce10pl?svg=true)](https://ci.appveyor.com/project/droundy/tinyset)
[![Crates.io version](https://img.shields.io/crates/v/tinyset.svg)](https://crates.io/crates/tinyset)

[Read the documentation.](https://docs.rs/tinyset)

# tinyset

`tinyset` contains a few collections that are optimized to scale
in size well for small numbers of elements, while still scaling
well in time (and size) for numbers of elements.  We now have
just a few types that you might care for.

1. [`Set64`] is a set for types that are 64 bits in size or less
and are `Copy`, intended for essentially integer types.  This is
our most efficient type, since it can store small sets with just
the size of one pointer, with no heap storage.

2. [`SetU64`] just holds `u64` items, and is the internal storage
of [`Set64`].

3. [`SetU32`] just holds `u32` items, and uses a bit less memory
than [`SetU64`].

4. [`SetUsize`] holds `usize` items, and uses either [SetU64] or
[SetU32] internally.

All of these set types will do no heap allocation for small sets of
small elements.  On 64-bit systems, each set will store up to seven
elements with no heap allocation, if the elements are small.  The
more elements there are, the smaller they need to be.

These sets all differ from the standard sets in that they iterate
over items rather than references to items, because they do not
store values directly in a way that can be referenced.  All of the
type-specific sets further differ in that `remove` and `contains`
accept values rather than references.

This crate has a single optional dependency (enabled by default), which
is the `rand` crate, used for randomization to avoid DOS collision attacks.
You can speed up your compile by disabling this feature with
```
tinyset = { version = "0.4", default-features = false }
```
which will result in using a very simple pseudorandom number generator
seeded by the system time.

# Benchmarks

To run the benchmark suite, run

    cargo bench

This will give you loads of timings and storage requirements for a
wide variety of set types.
