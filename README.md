# `collection_cap`

[![Build](https://github.com/MaxMahem/collection_cap/actions/workflows/build.yml/badge.svg)](https://github.com/MaxMahem/collection_cap/actions/workflows/build.yml)
[![Docs](https://github.com/MaxMahem/collection_cap/actions/workflows/docs.yml/badge.svg)](https://MaxMahem.github.io/collection_cap/collection_cap/index.html)
[![dependency status](https://deps.rs/repo/github/MaxMahem/collection_cap/status.svg)](https://deps.rs/repo/github/MaxMahem/collection_cap)
[![codecov](https://codecov.io/github/MaxMahem/collection_cap/graph/badge.svg?token=N5JJLLQ04L)](https://codecov.io/github/MaxMahem/collection_cap)
![GitHub License](https://img.shields.io/github/license/MaxMahem/collection_cap)

A lightweight crate for defining and validating capacity constraints.

This crate is `no_std` compatible and contains no `unsafe` code.

## Core Traits

- **`Capacity`**: Validates iterator compatibility against a capacity constraint.
- **`StaticCap`**: Declares a compile-time capacity constraint.
- **`VariableCap`**: Declares a runtime capacity constraint.

Implementations are provided for `Array` by default. See the [features](#features) section for more conditional enabled implementations.

## Static Capacity

Types that have a compile-time capacity constraint (like arrays) can implement `StaticCap`. The constraint can then be checked using the `IterCapExt` extension trait. If a fully consumed iterator violates the capacity constraint, an error is returned. The specific error type is defined by the `Capacity` implementation.

This is most useful for pre-validating collection type operations.

```rust
use collection_cap::IterCapExt;

// an array can accept no more or less than its size
(0..10).ensure_compatible::<[i32; 10]>().expect("Should be compatible");
(0..9).ensure_compatible::<[i32; 10]>().expect_err("Should underflow");
(0..11).ensure_compatible::<[i32; 10]>().expect_err("Should overflow");
```

```rust
use arrayvec::ArrayVec;
use collection_cap::IterCapExt;

let mut vec = ArrayVec::<i32, 10>::new();

// an arrayvec can accept up to its capacity
(0..10).ensure_compatible::<ArrayVec<i32, 10>>().expect("Should be compatible");
(0..11).ensure_compatible::<ArrayVec<i32, 10>>().expect_err("Should overflow");
```

## Variable Capacity

Types that have a capacity constraint that can change or is determined at runtime (like `ArrayVec`) can implement `VariableCap`. The constraint can then be checked using the `IterCapExt` extension trait. If a fully consumed iterator violates the capacity constraint, an error is returned. The specific error type is defined by the `Capacity` implementation.

```rust
use collection_cap::IterCapExt;
use arrayvec::ArrayVec;

let mut ten_element_vec: ArrayVec<i32, 10> = (0..5).collect();
assert_eq!(ten_element_vec.remaining_capacity(), 5);

(0..3).ensure_compatible_with(&ten_element_vec).expect("3 more elements should be compatible");
(0..6).ensure_compatible_with(&ten_element_vec).expect_err("6 more elements should not be compatible");
```

To specify a capacity constraint directly, you can use any type that implements `Capacity`. Including purpose-built types like `MinCapVal`, `MaxCapVal`, `MinMaxCapVal`, and `ExactCapVal`. Or any std range type.

```rust
use collection_cap::IterCapExt;

(0..5).ensure_compatible_with(..=5).expect("5 elements should be compatible");
(0..6).ensure_compatible_with(..=5).expect_err("6 elements should not be compatible");
```

### Capacity Compatibility

Note that for non-exact size iterators, these checks only guarantee that an iterator's `size_hint` is compatible with the given capacity. They do not guarantee that an iterator will actually fit the capacity during iteration, as the `size_hint` only reports the minimum and maximum number of elements an iterator *might* produce. An error is only returned if the iterator's `size_hint` indicates that it *cannot* fit the capacity constraint.

A 'universal' size hint (`(0, None)`), for example, indicates that the iterator can produce any number of elements, and so is compatible with any capacity constraint.

```rust
use collection_cap::IterCapExt;

let infinite_iter = std::iter::repeat(0).filter(|_| true);
assert_eq!(infinite_iter.size_hint(), (0, None), "Should produce A 'universal' size hint"); 

infinite_iter.ensure_compatible::<[i32; 10]>()
    .expect("A 'universal' size hint is compatible with any capacity");
```

If these methods return an error, however, it guarantees that the iterator's `size_hint` is incompatible with the capacity constraints.

## Implementing for local types

Using the traits for local types is straightforward. If the type has a known static capacity when empty, implement `StaticCap` using one of the appropriate `Capacity` types and `const CAP` value. If the type's capacity is mutable at runtime, implement `VariableCap` with a `capacity()` method that returns the appropriate `Capacity` type.

## Installation

It's on crates.io: [collection_cap](https://crates.io/crates/collection_cap)

### Features

- `arrayvec`: Implements `StaticCap` and `VariableCap` for `ArrayVec`.
