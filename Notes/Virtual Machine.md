---
title: "Virtual Machine"
date: 2023-05-26
---
# Bytecode Interpreter
## Motivation
A tree-walk [Abstract Syntax Tree](Notes/Representing%20Code.md#Abstract%20Syntax%20Tree) interpreter is slow. The parser converts tokens into individual AST objects, which use up a lot of memory and do not take advantage of [spatial locality](Notes/Virtual%20Memory.md#Working%20Set%20Model).
![](https://i.imgur.com/g9oCfBB.png)
## What is bytecode?
Structurally, bytecode resembles machine code. It’s a dense, linear sequence of binary instructions. That keeps overhead low and plays nice with the cache. However, it’s a much simpler, higher-level instruction set than any real chip out there. (In many bytecode formats, each instruction is only a single byte long, hence “bytecode”.)
## Virtual Machine
Bytecode is an idealized fantasy instruction set that makes your life as the compiler writer easier.

The problem with a fantasy architecture, of course, is that it doesn’t exist. We solve that by writing an _emulator_—a simulated chip written in software that interprets the bytecode one instruction at a time. A _virtual machine (VM)_, if you will.

The VM is the runtime of the bytecode. It will execute bytecode instructions to do what we want.
### Stack
A VM which uses a stack to execute instructions. When an instruction “produces” a value, it pushes it onto the stack. When it needs to consume one or more values, it gets them by popping them off the stack.
```c
define BINARY_OP(op) \
    do { \
      double b = pop(); \
      double a = pop(); \
      push(a op b); \
    } while(false)
```