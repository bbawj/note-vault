---
title: "Allocators"
date: 2023-07-15
---
# Allocators
A kernel often also requires support for heap allocation. With the support of [Paging](Notes/Memory%20Organisation.md#Paging), we can define a virtual memory range and map it to physical frames. Now, all we need is an allocator.

The job of an allocator is to manage the available heap memory. It needs to return unused memory on `alloc` calls and keep track of memory freed by `dealloc` so that it can be reused again. Most importantly, it must never hand out memory that is already in use somewhere else because this would cause undefined behavior.
## Bump Allocator
The idea behind a bump allocator is to linearly allocate memory by increasing (_“bumping”_) a `next` variable, which points to the start of the unused memory. At the beginning, `next` is equal to the start address of the heap. On each allocation, `next` is increased by the allocation size so that it always points to the boundary between used and unused memory:
![](Pics/Pasted%20image%2020230715101712.png)
### Pros and Cons
- The big advantage of bump allocation is that it’s very fast. Compared to other allocator designs that need to actively look for a fitting memory block and perform various bookkeeping tasks on `alloc` and `dealloc`, a bump allocator [can be optimized](https://fitzgeraldnick.com/2019/11/01/always-bump-downwards.html) to just a few assembly instructions. This makes bump allocators useful for optimizing the allocation performance, for example when creating a [virtual DOM library](https://hacks.mozilla.org/2019/03/fast-bump-allocated-virtual-doms-with-rust-and-wasm/).
- The main limitation of a bump allocator is that it can only reuse deallocated memory after all allocations have been freed. This means that a single long-lived allocation suffices to prevent memory reuse.
  ```rust
fn many_boxes_long_lived() {
    let long_lived = Box::new(1); // new
    for i in 0..HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
    assert_eq!(*long_lived, 1); // new
}
```
### Tricks
- We could update `dealloc` to check whether the freed allocation was the last allocation returned by `alloc` by comparing its end address with the `next` pointer. In case they’re equal, we can safely reset `next` back to the start address of the freed allocation. This way, each loop iteration reuses the same memory block.
- We could add an `alloc_back` method that allocates memory from the _end_ of the heap using an additional `next_back` field. Then we could manually use this allocation method for all long-lived allocations, thereby separating short-lived and long-lived allocations on the heap. Note that this separation only works if it’s clear beforehand how long each allocation will live. Another drawback of this approach is that manually performing allocations is cumbersome and potentially unsafe.
### The fundamental issue
A bump allocator cant effectively reuse freed memory regions. 
![png](Pics/Pasted%20image%2020230715102700.png)
There are a total of 5 unused memory regions, but the next pointer only gives us access to the last one. We could store all the unused regions in a constant sized array but we would not be able to know what size or how large it could get. We can't use dynamic data structures, because then our heap allocator would depend on itself.
## Linked List Allocator
![png](Pics/Pasted%20image%2020230715110728.png)

