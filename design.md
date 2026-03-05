# Architecture & Design

This document captures the rationale behind important architectural and API design choices in `collection_cap`.

## The `Capacity` Trait Architecture

The capacity system is split into internal mechanics and user-facing API surface. This mirrors the standard library's `Iterator` and `IntoIterator` traits.

### `Capacity`: The "Iterator"

```rust
pub trait Capacity: Sealed + RangeBounds<usize> + Clone + Copy + Debug
```

- **Sealed & Internal**: The `Capacity` trait is sealed. We control all of its implementations natively. This trait is directly responsible for carrying out the mathematical capability checks (`check_compatibility` and `check_fit`).
- **The Core Engine**: Just as `Iterator` dictates how to step through data, `Capacity` drives how bounds are mathematically enforced. Guarding implementations behind a sealed trait ensures that error logic and optimization semantics are uniform and sound.

### `StaticCap` and `VariableCap`: The "IntoIterator"

Users of `collection_cap` interact predominantly with `StaticCap` and `VariableCap`.

- **User-Facing Extension Points**: These traits represent types that "have" a capacity, providing a mechanism to extract the internal, sealed `Capacity` instances safely.
- **Small Surface Area**: By keeping `StaticCap` and `VariableCap` very simple (often just returning or pointing to a `Capacity` type constraint), users can easily implement these traits on their own custom wrappers, types, or configuration payloads safely. This separates the complex logic of constraint validation from the simple property of "having" a constraint.

## Role of `Capacity` Elements

When reading the `Capacity` trait definition, you will see a few strictly guarded types that must be satisfied. Their existence serves very precise semantic roles.

### `CapError` and `FitError` Associated Types

These associated types allow the `check_compatability` and `check_fit` methods to return error types that exactly capture their failure modes. For example, a `MinCapVal` can only fail by underflowing, and a `MaxCapVal` can only fail by overflowing. While a `MinMaxCapVal` can fail by either underflowing or overflowing.

### `Min` and `Max` Associated Types

Each capacity implementer must declare associated `Min` and `Max` types (which themselves must also implement `Capacity`).

- **Why they exist**: If a capacity represents a constraint like `0..=10`, verifying compatibility fundamentally evaluates two separate conditions: "Is it >= 0?" and "Is it <= 10?".
- **Breaking into Atoms**: Defining `Min` and `Max` as their own constituent capacity constructs allows composite constraints (like `MinMaxCapVal` or `StaticMinMaxCap`) to be mathematically factored into single bounds natively within the type system.
- **Precise Error Mapping**: When a constraint fails, we don't return a generic "Out of bounds" error. By decomposing limits using `Min` and `Max`, the engine natively attaches the exact failed constraint representation (e.g. `MinUnderflow<Self::Min>`). This ensures debug traces isolate exactly which side of the boundary was violated and what the individual boundary state looked like.
