---
title: "Virtual Machine"
date: 2023-05-26
---
# Virtual Machine
The virtual machine is one part of our interpreter’s internal architecture. You hand it a chunk of code—literally a Chunk—and it runs it.
## Stack
A VM which uses a stack to execute instructions. When an instruction “produces” a value, it pushes it onto the stack. When it needs to consume one or more values, it gets them by popping them off the stack.
```c
define BINARY_OP(op) \
    do { \
      double b = pop(); \
      double a = pop(); \
      push(a op b); \
    } while(false)
```