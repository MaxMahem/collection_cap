# `collection_cap`

[![Build](https://github.com/MaxMahem/collection_cap/actions/workflows/build.yml/badge.svg)](https://github.com/MaxMahem/collection_cap/actions/workflows/build.yml)
[![Docs](https://github.com/MaxMahem/collection_cap/actions/workflows/docs.yml/badge.svg)](https://MaxMahem.github.io/collection_cap/collection_cap/index.html)
[![dependency status](https://deps.rs/repo/github/MaxMahem/collection_cap/status.svg)](https://deps.rs/repo/github/MaxMahem/collection_cap)
[![codecov](https://codecov.io/github/MaxMahem/collection_cap/graph/badge.svg?token=N5JJLLQ04L)](https://codecov.io/github/MaxMahem/collection_cap)
![GitHub License](https://img.shields.io/github/license/MaxMahem/collection_cap)

A lightweight crate for defining and validating capacity constraints.

This crate is `no_std` compatible and contains no `unsafe` code.

## Core Traits

- **`MinCap`**: Defines a minimum capacity constraint.
- **`MaxCap`**: Defines a maximum capacity constraint.
- **`RemainingCap`**: Allows a collection to report its remaining capacity.
- **`CapConstraint`**: A trait for types that have a capacity constraint, and can be used to validate that an iterator can fit the capacity constraint.

Implementations are provided for `Array` by default. See the [features](#features) section for more conditional enabled implementations.

## Capacity Errors

### `CapError<C>`, `CapOverflow<C>`, `CapUnderflow<C>`

These validate that an iterator's bounds can satisfy the static `C::MIN_CAP` and `C::MAX_CAP` constraints of a specific collection `C`.

All implementations from this crate also implement `CapConstraint` returning one of these error types, allowing them to be used via the iterator extension trait `IterCapExt`.

```rust
use collection_cap::IterCapExt;

(0..10).ensure_can_fit::<[i32; 10]>().expect("Should fit");
(0..9).ensure_can_fit::<[i32; 10]>().expect_err("Should underflow");
(0..11).ensure_can_fit::<[i32; 10]>().expect_err("Should overflow");
```

```rust
use arrayvec::ArrayVec;
use collection_cap::IterCapExt;

let mut vec = ArrayVec::<i32, 10>::new();
(0..10).ensure_can_fit::<ArrayVec<i32, 10>>().expect("Should fit");
(0..11).ensure_can_fit::<ArrayVec<i32, 10>>().expect_err("Should overflow");
```

### `FitError`, `Overflows`, `Underflows`

These error types validate and report if it is possible for an `Iterator` to fufill a capacity constraint based on its `size_hint` based on a runtime capacity constraint. This is particularly useful for validating that an iterator can fit into the remaining capacity of a collection, and `IterCapExt` provides a convenient way to query this.

```rust
use collection_cap::IterCapExt;
use arrayvec::ArrayVec;

let mut ten_element_vec: ArrayVec<i32, 10> = (0..5).collect();
assert_eq!(ten_element_vec.remaining_capacity(), 5);

(0..3).ensure_fits_in(&ten_element_vec).expect("3 more elements should fit");
(0..6).ensure_fits_in(&ten_element_vec).expect_err("6 more elements should not fit");
```

### Capacity Compatibility

Note: that for non-exact size iterators, these error types can only guarantee that an iterator theoretically *can* fit in the given capacity. They do not guarantee that an iterator will actually fit in the given capacity, as a size hint only reports the minimum and maximum number of elements an iterator can produce. Failure however still guarantees that an iterator can not fit in the given capacity.

```rust
use collection_cap::IterCapExt;

let infinite_iter = std::iter::repeat(0).filter(|_| true);
assert_eq!(infinite_iter.size_hint(), (0, None), 
    "A filtered repeat iterator can produce between 0 and infinite elements"); 

infinite_iter.ensure_can_fit::<[i32; 10]>()
    .expect("Since the iterator can produce 10 elements, it is compatible");
```

## Capacity Markers

In some cases, it may be useful to define a capacity constraint without a specific collection type. For example, validating that an iterator can produce a certain number of elements. For this, the crate provides `MinCapMarker`, `MaxCapMarker`, `MinMaxCap`, and `ExactSize`.

## Installation

It's on crates.io: [collection_cap](https://crates.io/crates/collection_cap)

### Features

- `arrayvec`: Implements `MaxCap` and `RemainingCap` for `ArrayVec`.
