# `collection_cap`

[![Build](https://github.com/MaxMahem/collection_cap/actions/workflows/build.yml/badge.svg)](https://github.com/MaxMahem/collection_cap/actions/workflows/build.yml)
[![Docs](https://github.com/MaxMahem/collection_cap/actions/workflows/docs.yml/badge.svg)](https://MaxMahem.github.io/collection_cap/collection_cap/index.html)
[![dependency status](https://deps.rs/repo/github/MaxMahem/collection_cap/status.svg)](https://deps.rs/repo/github/MaxMahem/collection_cap)
[![codecov](https://codecov.io/github/MaxMahem/collection_cap/graph/badge.svg?token=N5JJLLQ04L)](https://codecov.io/github/MaxMahem/collection_cap)
![GitHub License](https://img.shields.io/github/license/MaxMahem/collection_cap)

A lightweight crate for defining and validating capacity constraints.

This crate is `no_std` compatible and contains no `unsafe` code.

## Core Traits

- **`StaticCap`**: A trait for types with a static, type-level capacity constraint. Declares a [`Cap`](StaticCap::Cap) type and a [`const CAP`](StaticCap::CAP) value that can check iterator compatibility.
- **`VariableCap`**: A trait for types that have a dynamic or runtime capacity constraint, and can be used to validate that an iterator is compatible with that constraint.

Implementations are provided for `Array` by default. See the [features](#features) section for more conditional enabled implementations.

## Static Capacity

Types that implement `StaticCap` (like arrays) can be checked using the `IterCapExt` extension trait:

```rust
use collection_cap::IterCapExt;

(0..10).ensure_compatible::<[i32; 10]>().expect("Should be compatible");
(0..9).ensure_compatible::<[i32; 10]>().expect_err("Should underflow");
(0..11).ensure_compatible::<[i32; 10]>().expect_err("Should overflow");
```

```rust
use arrayvec::ArrayVec;
use collection_cap::IterCapExt;

let mut vec = ArrayVec::<i32, 10>::new();
(0..10).ensure_compatible::<ArrayVec<i32, 10>>().expect("Should be compatible");
(0..11).ensure_compatible::<ArrayVec<i32, 10>>().expect_err("Should overflow");
```

## Variable Capacity

The error types `VarCapError`, `Overflows`, and `Underflows` validate and report if an `Iterator` is compatible with a capacity constraint based on its `size_hint` and a runtime capacity constraint, `VariableCap`. This is particularly useful for validating that an iterator is compatible with the remaining capacity of a collection, and `IterCapExt` provides a convenient way to query this.

```rust
use collection_cap::IterCapExt;
use arrayvec::ArrayVec;

let mut ten_element_vec: ArrayVec<i32, 10> = (0..5).collect();
assert_eq!(ten_element_vec.remaining_capacity(), 5);

(0..3).ensure_compatible_with(&ten_element_vec).expect("3 more elements should be compatible");
(0..6).ensure_compatible_with(&ten_element_vec).expect_err("6 more elements should not be compatible");
```

It is also possible to specify the constraint to check directly, using any type that implements `VariableCap`. A variety of `VariableCap` implementations are provided, including `MinCapVal`, `MaxCapVal`, `MinMaxCapVal`, and `ExactCapVal`. In addition, `RangeTo`, `RangeToInclusive`, `RangeFrom`, `Range`, `RangeInclusive`, and `RangeFull` also implement `VariableCap`.

```rust
use collection_cap::IterCapExt;

(0..5).ensure_compatible_with(..=5).expect("5 elements should be compatible");
(0..6).ensure_compatible_with(..=5).expect_err("6 elements should not be compatible");
```

### Capacity Compatibility

Note that for non-exact size iterators, these checks only guarantee that an iterator's `size_hint` is compatible with the given capacity. They do not guarantee that an iterator will actually fit the capacity during iteration, as the `size_hint` only reports the minimum and maximum number of elements an iterator *might* produce.

A 'universal' size hint (`(0, None)`), for example, is compatible with any capacity because it doesn't contradict any constraints.

```rust
use collection_cap::IterCapExt;

let infinite_iter = std::iter::repeat(0).filter(|_| true);
assert_eq!(infinite_iter.size_hint(), (0, None), "Should produce A 'universal' size hint"); 

infinite_iter.ensure_compatible::<[i32; 10]>()
    .expect("A 'universal' size hint is compatible with any capacity");
```

If these methods return an error, however, it guarantees that the iterator's
`size_hint` is incompatible with the capacity constraints.

## Capacity Markers

In some cases, it may be useful to define a capacity constraint without a specific collection type. For example, validating that an iterator is compatible with a certain number of elements. For this, the crate provides `MinCapMarker`, `MaxCapMarker`, `MinMaxCap`, and `ExactSize` for type-level constraints, and `MinCapVal`, `MaxCapVal`, `MinMaxCapVal`, and `ExactCapVal` for runtime constraints.

## Implementing for local types

Using the traits for local types is straightforward. If the type has a known static capacity when empty, implement `StaticCap` using one of the appropriate `Capacity` types and `const CAP` value. If the type's capacity is mutable at runtime, implement `VariableCap`.

## Installation

It's on crates.io: [collection_cap](https://crates.io/crates/collection_cap)

### Features

- `arrayvec`: Implements `StaticCap` and `VariableCap` for `ArrayVec`.
