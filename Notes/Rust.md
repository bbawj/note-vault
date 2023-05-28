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
