# Architecture & Design

This document captures the rationale behind important architectural and API design choices in `collection_cap`.

## The `Capacity` Trait Architecture

The capacity system is split into internal mechanics and user-facing API surface. This mirrors the standard library's `Iterator` and `IntoIterator` traits.

### `Capacity`: The "Iterator"

The `Capacity` trait is the core of the capacity system and is responsible for carrying out the actual capacity checks (`check_compatibility` and `check_fit`). It is sealed and internal, and all implementations are local to this crate.

There are three varieties of implementations:

1. **Const**: Represents a capacity that is known at compile time.
2. **Variable**: Represents a capacity that is known at runtime.
3. **`UnboundCap`**: A unit type that represents a capacity with no bounds.

Since the 'variable' implementations are all `const` constructable, theoretically they could be used to cover the 'const' case. However, the 'const' implementations allow some constraints to be validated at compile time (i.e. a `ConstMinMaxCap` can be validated to always have a Max >= Min).

In addition, collection type operations usually operate on const capacities, so having a const capacity type allows these types of errors to have a different error type than extension operations, which usually have variable capacities. The `ArrayVec` implementation demonstrates this divide, it has implementations of both `ConstCap` and `VariableCap`, and uses const capacity types for its Const implementations.

See also the error type design section.

### `ConstCap` and `VariableCap`: The "IntoIterator"

The primary extension points of `collection_cap` are `ConstCap` and `VariableCap`. These traits represent types that "have" a capacity, and provide a mechanism to provide that capacity.

Because of this, the `ConstCap` and `VariableCap` traits should be kept very simple for users to implement. They only need to provide a way to get the const or current capacity. Avoid designs that add extra constraints or logic to these traits that the user has to implement, those should be handled by the `Capacity` trait itself.

## `Capacity` Trait

In theory the design space of capacity could be further locked down. For example, variable capacities generallly represent the ability of an instance of a collection's (limited) capability for extension. However, if an instance exists, it is unlikely that it would only accept extension of at least a minimum size. Indeed, probably only the `MaxCapVal` implementations make sense for them.

However, since the `ConstCap` and `VariableCap` implementations let them define the type of capacity they return, a type can still constrain the type of capacities that are reasonable for them, without having to define seperate traits for each type of capacity.

### `CapError` and `OverlapError` Associated Types

These are implement as associated types, to allow the `check_compatability` and `check_fit` methods to return error types that exactly capture their failure modes. For example, a `MinCapVal` can only fail by underflowing, and a `MaxCapVal` can only fail by overflowing. While a `MinMaxCapVal` can fail by either underflowing or overflowing. The altenrative would be one global error type, but that would allow representation of some failure types that are not possible for some Capacity types, which is not ideal.

### `Min` and `Max` Associated Types, and `min_cap` and `max_cap` Methods

These types allow the capacity to be decomposed into their constituent min and max bound components. The utility here is marginal, but it does not cost the user anything to have them. It does allow some logic to be expressed more generally. Possibly it might be useful as a constraint later as well. For example, you could constrain to only take `ConstCap` or `VariableCap` that have a `Cap` value that has a `Min` value of `UnboundCap`, i.e. they don't have a minimum capacity constraint.

## `OverlapError` and `CapError` Error Types

The primary design element here is making the error types generic over the capacity type. This allows the same error types to be used for both const and variable capacities, but stil be distinguishable on a type level.

It would be possible to further constrain these error types, for example, by requiring that a `MinUnderflow`'s CAP have a `UnboundCap` for its `Max` value. Preventing it from being constructed with a `Cap` that has a Max bound. However, since all implementations of `Capacity` are local to this crate, and the primary way of creating any of these error types is via the return type of the `check_compatibility` and `check_fit` methods, which come from these `Capacity` implementations, it is not necessary to add these constraints, and instead simply not ever create any illogical implementations.

Still this design decision might be worth revisting.

## Ideas to explore

* Would it be worth adding a constraint the forced implementations of `ConstCap` to return a const capacity type? Should `VariableCap` then be forced to return a variable capacity type? The former could be done by requiring the `Cap` associated type to be a `ConstCap` implementation, since all of our const capacity types also implement that trait. This enforces our prefered design decision but might be a little confusing to users.

Doing similar for `VariableCap` would also be possible, but seems less useful. The const cap types currently implement `VariableCap` (for convience), which would be prevented. But this could be removed (maybe it isn't a great idea to begin with?). It seems unlikely that a user would want to use a const capacity type as a runtime capacity. So maybe doing this would prevent a silly mistake?

* If/when the `Try` trait stabilizes we can look into using a `Failable` return type instead of `Result` since the unit type return value is not useful.
