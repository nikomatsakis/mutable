# mutable

This repository contains a thought experiment around how to make
working with shared, mutable data more ergonomic. My goal is to enable
a "Java-like" model -- though with a few extensions.

This crate uses unsafe code for efficiency, but the most important
thing is actually the "API surface" that it exports: in particular, it
tries to encourage a style of working with shared data where you are
never getting references to the contents, but instead copying things
out. This avoids the memory safety hazards associated with sharing.
(In fact, one could do a safe implementation that avoids the unsafe
code, at the cost of some dynamic checks; this may be a good
trade-off.)

## Java-like

In Java, values break down into two categories:

- small values, like `int` or `float`, which are copied by value
- references to objects, which are garbage-collected pointers

Moreover, one cannot get a reference to an individual field of an
object: you can only read the field (copying out its current value) or
write the field (overwriting its current value).

This is roughly the model that the `mutable` crate espouses: if you
want to create a struct with mutable fields, you simply declare those
fields as having type `Mut<T>` instead of `T`. Here, the type `T`
should be either a `Copy` type (`u32`, `usize`, etc) or a reference
counted pointer (`Rc<T>` or `Arc<T>`).

So, for example, I might translate a Java setup like:

```java
class Foo {
  Context cx;
}

class Context {
  int counter;
}
```

to the following:

```rust
struct Foo {
  cx: Mut<Rc<Context>>
}

struct Context {
  counter: Mut<usize>
}
```

Now, if I want to access the current value of the counter from a `foo: Foo`
value, I would do:

```rust
foo.cx.get().counter.get()
```

It is obviously kind of annoying that I must intersperse `get()`
operations at each point. I could of course declare `cx: Rc<Context>`
(or even `cx: Context`) to avoid this, but in that case the field
would not be mutable (the equivalent of `final Context cx` in
Java). Only you can decide if that fits your use case.

## Collections: Vectors and hashmaps

Of course, one annoyance you will find with this model is that we
often want to have collections, such as vectors and hashmaps. The
existing Rust collections don't like being put into a `Rc` -- they need to be able to get
`&mut` access for operations like `push`, and an `Rc` can't provide that.

For this reason we offer a `MutVec<T>` type. If you wanted to get the
equivalent of a Java `ArrayList<T>`, you would use a `Rc<MutVec<T>>`.
Like a Java `ArrayList`, a `MutVec` can be mutated through any alias.
Also like a Java `ArrayList`, it is only meant to be used with
"Java-like" types (scalar values or reference counted pointers). Since
a `MutVec` can be mutated through any alias, it never offers
references to its contents -- so it cannot implement the `Index`
trait. You can get data from it by doing `vec.at(3)`, for example, to
fetch the value with index `3`. Similarly, iterating over a
`MutVec<T>` yields up values of type `T` (as opposed to `&T`, for a
standard vector).

I intend to implement a `MutMap` type but didn't get around to it. =)
One obstacle though is that we would need to have a stronger notion of
purity (see the next section). Notably, we would need to ensure that
the `Hash` and `Eq` traits are 'purely' implemented.

## "Pure cloning"

One key concept in the library is the `PureClone` marker trait.  In
particular, the main thing that the `Mut` type offers above `Cell` is
that if you have a `Mut<Rc<T>>` value, you can still use `get()`, even
though `T: Copy` does not hold. The intuition here is that cloning an
`Rc<T>` is only going to increment the ref count and is never going to
mutate the cell.

This intuition is captured in the `PureClone` trait -- it is an unsafe
trait, and it declares that invoking `Clone::clone` on a value `v: T`
will never mutate any cells that (transitively) contain `v`. This is
basically always true in all realistic clone implementations, but is
not formally guaranteed.

## Stability caveats

This code is **not for production use** (at least not yet). For one
thing, it is something I dashed off in an airport. It also depends on
the nightly feature that enables overlapping marker traits for
ergonomics. **Finally, and most importantly, it makes some assumptions
about standard library types for its soundness guarantees -- these are
assumptions that *I* think are reasonable but which are not formally
agreed to.**

For example, we assume that `vec.push(...)` will never mutate any
cells (that it is "pure" in the sense of pure cloning).

