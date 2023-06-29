---
title: "Garbage Collection"
date: 2023-06-26
---
# Garbage Collection
In a *managed language*, the language implementation manages memory allocation and freeing on the user’s behalf. When a user performs an operation that requires some dynamic memory, the [Virtual Machine](Notes/Virtual%20Machine.md) automatically allocates it. The programmer never worries about deallocating anything. It ensures any memory the program is using sticks around as long as needed using a **garbage collector**.
## Reachability
How does a VM tell what memory is not needed? It considers a piece of memory to still be in use if it could possibly be read in the future. A value is *reachable* if there is some way for a user program to reference it. 

Some values can be directly accessed:
```
var global = "string";
{
  var local = "another";
  print global + local;
}
```
These are available on the stack or as an entry in the global hashmap and are called **roots**. Any values referenced by roots must still be alive and hence also reachable.
1. Starting with the roots, traverse through object references to find the full set of reachable objects.
2. Free all objects _not_ in that set.
## Mark-Sweep Garbage Collection
![](https://i.imgur.com/sRZFQIX.png)
- Marking: start with the roots and traverse through all of the objects those roots refer to. This is a classic graph traversal of all of the reachable objects. Each time we visit an object, we mark it in some way. 
- Sweeping: Once the mark phase completes, every reachable object in the heap has been marked. That means any unmarked object is unreachable and ripe for reclamation. We go through all the unmarked objects and free each one.
### Tricolor Abstraction
Each object has a conceptual “color” that tracks what state the object is in, and what work is left to do. This allows the GC to pause and pick up where it left off when needed.
![](https://i.imgur.com/M9H2mvs.png)
-  **![A white circle.](http://craftinginterpreters.com/image/garbage-collection/white.png) White:** At the beginning of a garbage collection, every object is white. This color means we have not reached or processed the object at all.
-  **![A gray circle.](http://craftinginterpreters.com/image/garbage-collection/gray.png) Gray:** During marking, when we first reach an object, we darken it gray. This color means we know the object itself is reachable and should not be collected. But we have not yet traced _through_ it to see what _other_ objects it references. In graph algorithm terms, this is the _worklist_—the set of objects we know about but haven’t processed yet.
-  **![A black circle.](http://craftinginterpreters.com/image/garbage-collection/black.png) Black:** When we take a gray object and mark all of the objects it references, we then turn the gray object black. This color means the mark phase is done processing that object.
### Weak References
A reference that does not protect the referenced object from collection by the GC. For example, [strings saved in the intern table](Notes/String%20Interning.md) have a direct (root) reference by the VM, but must be treated as a weak reference if not the GC would never clean up its memory.
## When to collect
Every managed language pays a performance price compared to explicit, user-authored deallocation. The time spent actually freeing memory is the same, but the GC spends cycles figuring out which memory to free. That is time not spent running the user’s code and doing useful work. In our implementation, that’s the entirety of the mark phase. The goal of a sophisticated garbage collector is to minimize that overhead.
![](https://i.imgur.com/etpe29Z.png)
- **Throughput** is the total fraction of time spent running user code versus doing garbage collection work
- **Latency** is the longest _continuous_ chunk of time where the user’s program is completely paused while garbage collection happens. It’s a measure of how “chunky” the collector is. 
### Self adjusting heap
The collector frequency automatically adjusts based on the live size of the heap. We track the total number of bytes of managed memory that the VM has allocated. When it goes above some threshold, we trigger a GC. After that, we note how many bytes of memory remain—how many were not freed. Then we adjust the threshold to some value larger than that.

The result is that as the amount of live memory increases, we collect less frequently in order to avoid sacrificing throughput by re-traversing the growing pile of live objects. As the amount of live memory goes down, we collect more frequently so that we don’t lose too much latency by waiting too long.
## Generational GC
A collector loses throughput if it spends a long time re-visiting objects that are still alive. But it can increase latency if it avoids collecting and accumulates a large pile of garbage to wade through. If only there were some way to tell which objects were likely to be long-lived and which weren’t. Then the GC could avoid revisiting the long-lived ones as often and clean up the ephemeral ones more frequently.

The key observation was that most objects are very short-lived but once they survive beyond a certain age, they tend to stick around quite a long time. The longer an object _has_ lived, the longer it likely will _continue_ to live.
- Every time a new object is allocated, it goes into a special, relatively small region of the heap called the “nursery”. Since objects tend to die young, the garbage collector is invoked frequently over the objects just in this region.
- Those that survive are now considered one generation older, and the GC tracks this for each object. If an object survives a certain number of generations—often just a single collection—it gets _tenured_. At this point, it is copied out of the nursery into a much larger heap region for long-lived objects. The garbage collector runs over that region too, but much less frequently since odds are good that most of those objects will still be alive.