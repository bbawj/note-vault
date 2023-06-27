---
title: "Garbage Collection"
date: 2023-06-26
---
# Garbage Collection
In a *managed language*, the language implementation manages memory allocation and freeing on the user’s behalf. When a user performs an operation that requires some dynamic memory, the [Virtual Machine](Notes/Virtual%20Machine.md) automatically allocates it. The programmer never worries about deallocating anything. It ensures any memory the program is using sticks around as long as needed using a **garbage collector**.
## Reachability
How does a VM tell what memory is not needed? It considers a piece of memory to still be in use if it could possibly be read in the future. A value is *reachable* if there is some way for a user program to reference it. 