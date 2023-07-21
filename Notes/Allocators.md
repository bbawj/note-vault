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
A linked list can be used to keep track of the freed areas of memory, hence the name, *free list*.
![png](Pics/Pasted%20image%2020230715110728.png)
Freed blocks should be merged together, else it will result in increasing and increasing fragmentation:
![](Pics/Pasted%20image%2020230716171506.png)
### Pros and Cons
- Able to reuse freed memory
- Poorer performance: list length depends on the number of unused memory blocks, the performance can vary extremely for different programs. A program that only creates a couple of allocations will experience relatively fast allocation performance. For a program that fragments the heap with many allocations, however, the allocation performance will be very bad because the linked list will be very long and mostly contain very small blocks.
## Fixed Size Block Allocator
Instead of allocating exactly as much memory as requested, we define a small number of block sizes and round up each allocation to the next block size. For example, with block sizes of 16, 64, and 512 bytes, an allocation of 4 bytes would return a 16-byte block, an allocation of 48 bytes a 64-byte block.
![](Pics/Pasted%20image%2020230716224307.png)
Like the linked list allocator, we keep track of the unused memory by creating a linked list in the unused memory. However, instead of using a single list with different block sizes, we create a separate list for each size class.

The property that each region in the list is the same size makes for some efficient allocations:
1. Round up the requested allocation size to the next block size. For example, when an allocation of 12 bytes is requested, we would choose the block size of 16 in the above example.
2. Retrieve the head pointer for the list, e.g., for block size 16, we need to use `head_16`.
3. Remove the first block from the list and return it.
Most notably, we can always return the first element of the list and no longer need to traverse the full list. Thus, allocations are much faster than with the linked list allocator. Deallocations also work the same way, by rounding up the freed size and adding the region to the head of the list, we avoid traversing the entire list.
### Fallback Allocator
A fallback allocator like a linked list allocator for allocation sizes which are rare, can reduce memory waste. Since only very few allocations of that size are expected, the linked list would stay small and the (de)allocations would still be reasonably fast.