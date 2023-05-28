---
title: "Rust"
date: 2023-05-28
lastmod: 2023-05-28
---
# Rust
## Lifetimes
Quite simply, a lifetime is the name of a region (~block/scope) of code somewhere in a program. That's it. When a reference is tagged with a lifetime, we're saying that it has to be valid for that *entire* region. Different things place requirements on how long a reference must and can be valid for. The entire lifetime system is in turn just a constraint-solving system that tries to minimize the region of every reference. If it successfully finds a set of lifetimes that satisfies all the constraints, your program compiles! Otherwise you get an error back saying that something didn't live long enough.
### Lifetime elision
There are certain cases that are so common that Rust will automatically pick the lifetimes for you. This is lifetime elision.
```rust
// Only one reference in input, so the output must be derived from that input
fn foo(&A) -> &B; // sugar for:
fn foo<'a>(&'a A) -> &'a B;

// Many inputs, assume they're all independent
fn foo(&A, &B, &C); // sugar for:
fn foo<'a, 'b, 'c>(&'a A, &'b B, &'c C);

// Methods, assume all output lifetimes are derived from `self`
fn foo(&self, &B, &C) -> &D; // sugar for:
fn foo<'a, 'b, 'c>(&'a self, &'b B, &'c C) -> &'a D;
```
## Reference counting with `Rc`
Let's say we want a set of 3 linked lists to look something like this:
```
list1 -> A ---+
              |
              v
list2 ------> B -> C -> D
              ^
              |
list3 -> X ---+

```
This just can't work with Boxes, because ownership of `B` is _shared_. Who should free it? If I drop list2, does it free B? With boxes we certainly would expect so!

Functional languages — and indeed almost every other language — get away with this by using _garbage collection_. With the magic of garbage collection, B will be freed only after everyone stops looking at it. Hooray! Instead, all Rust has today is _reference counting_. Reference counting can be thought of as a very simple GC.

Rc is just like Box, but we can duplicate it, and its memory will _only_ be freed when _all_ the Rc's derived from it are dropped. Unfortunately, this flexibility comes at a serious cost: we can only take a shared reference to its internals. This means we can't ever really get data out of one of our lists, nor can we mutate them.
## Mutability
_Inherited mutability_ (AKA external mutability): the mutability of a value is inherited from the mutability of its container. That is, you can't just randomly mutate some field of a non-mutable value because you feel like it.

Interior mutability types violate this: they let you mutate through a shared reference. There are two major classes of interior mutability: cells, which only work in a single-threaded context; and locks, which work in a multi-threaded context.
### Interior Mutability with `Rc<RefCell<T>>`
```rust
fn borrow(&self) -> Ref<'_, T>;
fn borrow_mut(&self) -> RefMut<'_, T>;
```
The rules for `borrow` and `borrow_mut` are exactly those of `&` and `&mut`: you can call `borrow` as many times as you want, but `borrow_mut` requires exclusivity.

Rather than enforcing this statically, RefCell enforces them at runtime. If you break the rules, RefCell will just panic and crash the program. Why does it return these Ref and RefMut things? Well, they basically behave like `Rc`s but for borrowing. They also keep the RefCell borrowed until they go out of scope.