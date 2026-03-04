# `collection_cap`

[![Build](https://github.com/MaxMahem/collection_cap/actions/workflows/build.yml/badge.svg)](https://github.com/MaxMahem/collection_cap/actions/workflows/build.yml)
[![Docs](https://github.com/MaxMahem/collection_cap/actions/workflows/docs.yml/badge.svg)](https://MaxMahem.github.io/collection_cap/collection_cap/index.html)
[![dependency status](https://deps.rs/repo/github/MaxMahem/collection_cap/status.svg)](https://deps.rs/repo/github/MaxMahem/collection_cap)
[![codecov](https://codecov.io/github/MaxMahem/collection_cap/graph/badge.svg?token=N5JJLLQ04L)](https://codecov.io/github/MaxMahem/collection_cap)
![GitHub License](https://img.shields.io/github/license/MaxMahem/collection_cap)

A lightweight crate for defining and validating capacity constraints.

This crate is `no_std` compatible and contains no `unsafe` code.

## Core Traits

- [**`Capacity`**](https://MaxMahem.github.io/collection_cap/collection_cap/trait.Capacity.html): Validates iterator compatibility against a capacity constraint.
- [**`StaticCap`**](https://MaxMahem.github.io/collection_cap/collection_cap/trait.StaticCap.html): Declares a compile-time capacity constraint.
- [**`VariableCap`**](https://MaxMahem.github.io/collection_cap/collection_cap/trait.VariableCap.html): Declares a runtime capacity constraint.

Implementations are provided for `Array` by default. See the [features](#features) section for more conditional enabled implementations.

### [`StaticCap`](https://MaxMahem.github.io/collection_cap/collection_cap/trait.StaticCap.html)

Types that have a compile-time capacity constraint (like arrays) can implement [`StaticCap`](https://MaxMahem.github.io/collection_cap/collection_cap/trait.StaticCap.html). And select a static [`Capacity`](https://MaxMahem.github.io/collection_cap/collection_cap/trait.Capacity.html) implementation to use. This is most useful for pre-validating collection type operations.

```rust
use collection_cap::StaticCap;
use collection_cap::cap::StaticMaxCap;

struct MyStaticCollection;

impl StaticCap for MyStaticCollection {
    type Cap = StaticMaxCap<10>;
    const CAP: Self::Cap = StaticMaxCap::<10>;
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

While the [`Capacity`](https://MaxMahem.github.io/collection_cap/collection_cap/trait.Capacity.html) trait can be used directly, it is most useful when combined with the [`IterCapExt`](https://MaxMahem.github.io/collection_cap/collection_cap/trait.IterCapExt.html) extension trait. This trait provides a number of methods for checking iterator compatibility and fit.

```rust
use collection_cap::IterCapExt;
use arrayvec::ArrayVec;

(0..10).ensure_compatible::<[i32; 10]>().expect("Exact match");
(0..11).ensure_compatible::<ArrayVec<i32, 10>>().expect_err("Overflow");

let mut vec: ArrayVec<i32, 10> = (0..5).collect();
(0..3).ensure_compatible_with(&vec).expect("Fits remaining");
(0..6).ensure_compatible_with(&vec).expect_err("Too many");
```

### Capacity types

The following [`Capacity`](https://MaxMahem.github.io/collection_cap/collection_cap/trait.Capacity.html) implementations are provided.

| Represents | Variable | Static | Range equivalent |
| :--- | :--- | :--- | :--- |
| Minimum | [`MinCapVal`](https://MaxMahem.github.io/collection_cap/collection_cap/cap/struct.MinCapVal.html) | [`StaticMinCap`](https://MaxMahem.github.io/collection_cap/collection_cap/cap/struct.StaticMinCap.html) | `min..` |
| Maximum | [`MaxCapVal`](https://MaxMahem.github.io/collection_cap/collection_cap/cap/struct.MaxCapVal.html) | [`StaticMaxCap`](https://MaxMahem.github.io/collection_cap/collection_cap/cap/struct.StaticMaxCap.html) | `..=max` |
| Min & Max | [`MinMaxCapVal`](https://MaxMahem.github.io/collection_cap/collection_cap/cap/struct.MinMaxCapVal.html) | [`StaticMinMaxCap`](https://MaxMahem.github.io/collection_cap/collection_cap/cap/struct.StaticMinMaxCap.html) | `min..=max` |
| Exact | [`ExactCapVal`](https://MaxMahem.github.io/collection_cap/collection_cap/cap/struct.ExactCapVal.html) | [`StaticExactCap`](https://MaxMahem.github.io/collection_cap/collection_cap/cap/struct.StaticExactCap.html) | `size..=size` |
| Unbounded | [`UnboundedCap`](https://MaxMahem.github.io/collection_cap/collection_cap/cap/struct.UnboundedCap.html) | [`UnboundedCap`](https://MaxMahem.github.io/collection_cap/collection_cap/cap/struct.UnboundedCap.html) | `..` |

These types can be used either directly, or as a type parameter for [`StaticCap`](https://MaxMahem.github.io/collection_cap/collection_cap/trait.StaticCap.html) or [`VariableCap`](https://MaxMahem.github.io/collection_cap/collection_cap/trait.VariableCap.html). [`Capacity`](https://MaxMahem.github.io/collection_cap/collection_cap/trait.Capacity.html) is also implemented for all std range types, as indicated.

```rust
use collection_cap::IterCapExt;
use collection_cap::cap::StaticExactCap;

(0..5).ensure_compatible_with(..=5).expect("should be compatible");
(0..6).ensure_compatible_with(..=5).expect_err("should not be compatible");

(0..5).ensure_compatible::<StaticExactCap<5>>().expect("should be compatible");
(0..6).ensure_compatible::<StaticExactCap<5>>().expect_err("should not be compatible");
```

## Capacity 'Compatibility'

Note that for non-[`ExactSizeIterator`](https://doc.rust-lang.org/std/iter/trait.ExactSizeIterator.html), these checks only guarantee that an iterator's [`size_hint`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.size_hint) is compatible with the given capacity. They do not guarantee that an iterator will actually fit the capacity during iteration, as the [`size_hint`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.size_hint) only reports the minimum and maximum number of elements an iterator *might* produce. An error is only returned if the iterator's [`size_hint`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.size_hint) indicates that it *cannot* fit the capacity constraint.

For a stronger guarantee, [`IterCapExt::ensure_fit`](https://MaxMahem.github.io/collection_cap/collection_cap/trait.IterCapExt.html#tymethod.ensure_fit) and [`IterCapExt::ensure_fits_into`](https://MaxMahem.github.io/collection_cap/collection_cap/trait.IterCapExt.html#tymethod.ensure_fits_into) can be used. These methods check if the *entire* range reported by an iterator's [`size_hint`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.size_hint) fits within the capacity constraints. If any possible count of elements could violate the constraint, these methods will return an error.

Put another way, `ensure_compatible` has the possibility of a false positive, while `ensure_fit` has the possibility of a false negative.

```rust
use collection_cap::IterCapExt;

let max_5_elements = ..5;

let produces_10 = (0..10).filter(|_| true);
assert_eq!(produces_10.size_hint(), (0, Some(10)), "Can produce 0 to 10 elements");
produces_10.ensure_compatible_with(max_5_elements)
    .expect("Compatibility only requires that it MIGHT fit");

let produces_3 = (0..10).filter(|x| *x < 3);
assert_eq!(produces_3.size_hint(), (0, Some(10)), "Can produce 0 to 10 elements");
produces_3.ensure_fits_into(max_5_elements)
    .expect_err("Fit requires that it MUST fit");
```

See the [`Capacity#note-on-fit`](https://MaxMahem.github.io/collection_cap/collection_cap/trait.Capacity.html#note-on-fit) documentation for more details.

## Installation

It's on crates.io: [collection_cap](https://crates.io/crates/collection_cap)

### Features

- `arrayvec`: Implements `StaticCap` and `VariableCap` for `ArrayVec`.
- `alloc`: Adds the `SpareCapacityExt` extension trait for `Vec`, `String`, and `VecDeque` to easily get their remaining capacity as a `MaxCapVal`.
