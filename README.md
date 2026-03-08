# `collection_cap`

[![Build](https://github.com/MaxMahem/collection_cap/actions/workflows/build.yml/badge.svg)](https://github.com/MaxMahem/collection_cap/actions/workflows/build.yml)
[![Docs](https://github.com/MaxMahem/collection_cap/actions/workflows/docs.yml/badge.svg)](https://MaxMahem.github.io/collection_cap/collection_cap/index.html)
[![dependency status](https://deps.rs/repo/github/MaxMahem/collection_cap/status.svg)](https://deps.rs/repo/github/MaxMahem/collection_cap)
[![codecov](https://codecov.io/github/MaxMahem/collection_cap/graph/badge.svg?token=N5JJLLQ04L)](https://codecov.io/github/MaxMahem/collection_cap)
![GitHub License](https://img.shields.io/github/license/MaxMahem/collection_cap)

A lightweight crate for defining and validating capacity constraints.

This crate is `no_std` intersecting and contains no `unsafe` code.

## Core Traits

- [**`Capacity`**](https://MaxMahem.github.io/collection_cap/collection_cap/trait.Capacity.html): Validates iterator intersection against a capacity constraint.
- [**`ConstCap`**](https://MaxMahem.github.io/collection_cap/collection_cap/trait.ConstCap.html): Declares a compile-time capacity constraint.
- [**`VariableCap`**](https://MaxMahem.github.io/collection_cap/collection_cap/trait.VariableCap.html): Declares a runtime capacity constraint.

Implementations are provided for `Array` by default. See the [features](#features) section for more conditional enabled implementations.

### [`ConstCap`](https://MaxMahem.github.io/collection_cap/collection_cap/trait.ConstCap.html)

Types that have a compile-time capacity constraint (like arrays) can implement [`ConstCap`](https://MaxMahem.github.io/collection_cap/collection_cap/trait.ConstCap.html). And select a const [`Capacity`](https://MaxMahem.github.io/collection_cap/collection_cap/trait.Capacity.html) implementation to use. This is most useful for pre-validating collection type operations.

```rust
use collection_cap::ConstCap;
use collection_cap::cap::ConstMaxCap;

struct MyConstCollection;

impl ConstCap for MyConstCollection {
    type Cap = ConstMaxCap<10>;
    const CAP: Self::Cap = ConstMaxCap::<10>;
}
```

### [`VariableCap`](https://MaxMahem.github.io/collection_cap/collection_cap/trait.VariableCap.html)

Types that have a capacity constraint that can change or is determined at runtime (like `ArrayVec`) can implement [`VariableCap`](https://MaxMahem.github.io/collection_cap/collection_cap/trait.VariableCap.html). And return a [`Capacity`](https://MaxMahem.github.io/collection_cap/collection_cap/trait.Capacity.html) implementation that reflects the current capacity constraint. This is most useful for pre-validating extension operations.

```rust
use collection_cap::VariableCap;
use collection_cap::cap::MaxCapVal;

struct MyDynamicCollection { remaining: usize }

impl VariableCap for MyDynamicCollection {
    type Cap = MaxCapVal;
    fn capacity(&self) -> Self::Cap { MaxCapVal(self.remaining) }
}
```

### [`IterCapExt`](https://MaxMahem.github.io/collection_cap/collection_cap/trait.IterCapExt.html)

While the [`Capacity`](https://MaxMahem.github.io/collection_cap/collection_cap/trait.Capacity.html) trait can be used directly, it is most useful when combined with the [`IterCapExt`](https://MaxMahem.github.io/collection_cap/collection_cap/trait.IterCapExt.html) extension trait. This trait provides a number of methods for checking iterator intersection and overlap.

```rust
use collection_cap::IterCapExt;
use arrayvec::ArrayVec;

(0..10).ensure_intersects::<[i32; 10]>().expect("Exact match");
(0..11).ensure_intersects::<ArrayVec<i32, 10>>().expect_err("Overflow");

let mut vec: ArrayVec<i32, 10> = (0..5).collect();
(0..3).ensure_intersects_with(&vec).expect("Overlaps remaining");
(0..6).ensure_intersects_with(&vec).expect_err("Too many");
```

### Capacity types

The following [`Capacity`](https://MaxMahem.github.io/collection_cap/collection_cap/trait.Capacity.html) implementations are provided.

| Represents | Variable | Const | Range equivalent |
| :--- | :--- | :--- | :--- |
| Minimum | [`MinCapVal`](https://MaxMahem.github.io/collection_cap/collection_cap/cap/struct.MinCapVal.html) | [`ConstMinCap`](https://MaxMahem.github.io/collection_cap/collection_cap/cap/struct.ConstMinCap.html) | `min..` |
| Maximum | [`MaxCapVal`](https://MaxMahem.github.io/collection_cap/collection_cap/cap/struct.MaxCapVal.html) | [`ConstMaxCap`](https://MaxMahem.github.io/collection_cap/collection_cap/cap/struct.ConstMaxCap.html) | `..=max` |
| Min & Max | [`MinMaxCapVal`](https://MaxMahem.github.io/collection_cap/collection_cap/cap/struct.MinMaxCapVal.html) | [`ConstMinMaxCap`](https://MaxMahem.github.io/collection_cap/collection_cap/cap/struct.ConstMinMaxCap.html) | `min..=max` |
| Exact | [`ExactCapVal`](https://MaxMahem.github.io/collection_cap/collection_cap/cap/struct.ExactCapVal.html) | [`ConstExactCap`](https://MaxMahem.github.io/collection_cap/collection_cap/cap/struct.ConstExactCap.html) | `size..=size` |
| Unbounded | [`UnboundedCap`](https://MaxMahem.github.io/collection_cap/collection_cap/cap/struct.UnboundedCap.html) | [`UnboundedCap`](https://MaxMahem.github.io/collection_cap/collection_cap/cap/struct.UnboundedCap.html) | `..` |

These types can be used either directly, or as a return type parameter for [`ConstCap`](https://MaxMahem.github.io/collection_cap/collection_cap/trait.ConstCap.html) or [`VariableCap`](https://MaxMahem.github.io/collection_cap/collection_cap/trait.VariableCap.html). [`VariableCap`](https://MaxMahem.github.io/collection_cap/collection_cap/trait.Capacity.html) is also implemented for std range types, as indicated.

```rust
use collection_cap::IterCapExt;
use collection_cap::cap::ConstExactCap;

(0..5).ensure_intersects_with(..=5).expect("should be intersecting");
(0..6).ensure_intersects_with(..=5).expect_err("should not be intersecting");

(0..5).ensure_intersects::<ConstExactCap<5>>().expect("should be intersecting");
(0..6).ensure_intersects::<ConstExactCap<5>>().expect_err("should not be intersecting");
```

## Capacity 'Intersection'

Note that for non-[`ExactSizeIterator`](https://doc.rust-lang.org/std/iter/trait.ExactSizeIterator.html), these checks only guarantee that an iterator's [`size_hint`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.size_hint) intersects with the given capacity. They do not guarantee that an iterator will actually overlap the capacity during iteration, as the [`size_hint`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.size_hint) only reports the minimum and maximum number of elements an iterator *might* produce. An error is only returned if the iterator's [`size_hint`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.size_hint) indicates that it *cannot* overlap the capacity constraint.

For a stronger guarantee, [`IterCapExt::ensure_overlaps`](https://MaxMahem.github.io/collection_cap/collection_cap/trait.IterCapExt.html#tymethod.ensure_overlaps) and [`IterCapExt::ensure_overlaps_into`](https://MaxMahem.github.io/collection_cap/collection_cap/trait.IterCapExt.html#tymethod.ensure_overlaps_into) can be used. These methods check if the *entire* range reported by an iterator's [`size_hint`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.size_hint) overlaps within the capacity constraints. If any possible count of elements could violate the constraint, these methods will return an error.

Put another way, `ensure_intersects` has the possibility of a false positive, while `ensure_overlaps` has the possibility of a false negative.

```rust
use collection_cap::IterCapExt;

let max_5_elements = ..=4;

let produces_10 = (0..10).filter(|_| true);
assert_eq!(produces_10.size_hint(), (0, Some(10)), "Can produce 0 to 10 elements");
produces_10.ensure_intersects_with(max_5_elements)
    .expect("Intersection only requires that it MIGHT overlap");

let produces_3 = (0..10).filter(|x| *x < 3);
assert_eq!(produces_3.size_hint(), (0, Some(10)), "Can produce 0 to 10 elements");
produces_3.ensure_overlaps_into(max_5_elements)
    .expect_err("Overlap requires that it MUST overlap");
```

See the [`Capacity#note-on-overlap`](https://MaxMahem.github.io/collection_cap/collection_cap/trait.Capacity.html#note-on-overlap) documentation for more details.

## Installation

It's on crates.io: [collection_cap](https://crates.io/crates/collection_cap)

### Features

- `arrayvec`: Implements `ConstCap` and `VariableCap` for `ArrayVec`.
- `alloc`: Adds the `SpareCapacityExt` extension trait for `Vec`, `String`, and `VecDeque` to allow querying their remaining capacity as a [`MaxCapVal`](https://MaxMahem.github.io/collection_cap/collection_cap/cap/struct.MaxCapVal.html).
