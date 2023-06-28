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
