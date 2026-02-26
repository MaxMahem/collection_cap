# collection_cap

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

Implementations are provided for `Array` by default. See the [features](#features) section for more conditional enabled implementations.

## Capacity Markers

In some cases, it may be useful to define a capacity constraint without a specific collection type. For example, validating that an iterator can produce a certain number of elements. For this, the crate provides `MinCapMarker`, `MaxCapMarker`, `MinMaxCap`, and `ExactSize`.

## Capacity Errors

### `CapError`, `CapOverflow`, `CapUnderflow`

These error types validate and report if it is possible for an `Iterator` to fufill a capacity constraint based on its `size_hint`. This is particularly useful for pre-validating operations before they attempt to fill a fixed-capacity collection. They use dynamic `usize` bounds at runtime.

#### Example: `ArrayVec` and `RemainingCap`

For types that implement `RemainingCap`, like `ArrayVec`, `CapOverflow` can be used to validate if an iterator will fit into the remaining capacity of the collection:

```rust
use collection_cap::err::CapOverflow;
use arrayvec::ArrayVec;

let mut ten_element_vec: ArrayVec<i32, 10> = (0..5).collect();
assert_eq!(ten_element_vec.remaining_capacity(), 5);

CapOverflow::ensure_can_fit_in(&(0..3), &ten_element_vec)
    .expect("3 more elements should fit");

let err = CapOverflow::ensure_can_fit_in(&(0..6), &ten_element_vec)
    .expect_err("6 more elements should not fit");
assert_eq!(err.min_size(), 6);
assert_eq!(err.max_cap(), 5);
```

### `TargetCapError<C>`, `TargetOverflow<C>`, `TargetUnderflow<C>`

These are strongly-typed errors that validate bounds using the static `C::MIN_CAP` and `C::MAX_CAP` constraints of a specific collection `C`. They implement `From` for easy conversion into their corresponding dynamic error types.

#### Example: `Array` capacity validation

An array has a `MinCap` and `MaxCap` of `SIZE`:

```rust
use collection_cap::err::TargetCapError;

TargetCapError::<[i32; 10]>::ensure_can_fit(&(0..10)).expect("Must fit");
TargetCapError::<[i32; 10]>::ensure_can_fit(&(0..9)).expect_err("Should underflow");
TargetCapError::<[i32; 10]>::ensure_can_fit(&(0..11)).expect_err("Should overflow");
```

### Capacity Compatability

Note: these capacity validations only gurantee that an iterator theoretically *can* fit in the given capacity. They do not guarantee that an iterator will actually fit in the given capacity. They only guarantee that an iterator's `size_hint` does not declare that it will *not* fit in the given capacity.

#### Example: Non-fitting iterator

```rust
use collection_cap::err::TargetCapError;

let infinite_iter = std::iter::repeat(0).filter(|_| true);
assert_eq!(infinite_iter.size_hint(), (0, None), 
    "An infinite filtered iterator can produce between 0 and infinit elements"); 

TargetCapError::<[i32; 10]>::ensure_can_fit(&infinite_iter)
    .expect("Since the iterator can produce 10 elements, it is compatible");
```

## Installation

It's on crates.io: [collection_cap](https://crates.io/crates/collection_cap)

### Features

- `arrayvec`: Implements `MaxCap` for `ArrayVec`.
