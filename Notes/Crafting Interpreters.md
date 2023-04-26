---
title: "Crafting Interpreters"
date: 2023-04-24
lastmod: 2023-04-26
---
# Crafting Interpreters
- [[Notes/Programming Language Design]]
## Parts of a Language
The paths from source code to machine code:
![](https://i.imgur.com/BzSEUJj.png)
### Scanning
Also known as lexing or lexical analysis. Take a linear stream of characters and chunk them into *tokens*.
![](https://i.imgur.com/HTwN5Ae.png)

More of this in [[Scanning]].
### Parsing
Takes the tokens and forms *grammar* through construction of an [Abstract Syntax Tree](Notes/Representing%20Code.md#Abstract%20Syntax%20Tree).
### Static Analysis
"In an expression like a + b, we know we are adding a and b, but we don’t know what those names refer to. Are they local variables? Global? Where are they defined?"
- Binding: for each *identifier*, figure out where it is defined and wire them together. This is affected by scoping.
- Type checking: if the language is statically typed, figure out the types of those identifiers and report type errors where operations are not supported.

Results from the analysis needs to be stored for later use:
- AST: stored back as attributes on the AST which were not previously initialised during parsing
- Symbol table: a lookup table associating each key (identifier) to what it refers to
### Intermediate Representation
Compiling code can be viewed as involving two ends. The "front-end" is specific to the source code language which the program is written in. The "back-end" is the target architecture which the program will run. IRs allow different front-ends to produce a shared IR, and allow different back-ends to convert the IR to the target architecture.
### Optimisation
Swapping parts of the program for parts which have the same semantics but implemented more efficiently.
### Code generation
Converting the IR into machine code. There are two options:
1. Real machine code generation: native code which the OS can directly execute . This is fast but involves complex work due to high number of instructions. It also makes the code less portable as it will only work on the specific target architecture.
2. Virtual machine code i.e. bytecode generation: produce code for a generalised idealised virtual machine which has synthetic instructions mapping more closely to language semantics than to a specific computer architecture
#### Virtual Machine
Not to be confused with the [system virtual machine](Notes/Virtualization.md). This abstraction refers to process virtual machines, a program that emulates the hypothetical chip to support the virtual architecture (targeted by the virtual machine code) at runtime.
### Runtime
We usually need some services that our language provides while the program is running. E.g.
Automatic memory management: a garbage collector needs to be implemented to reclaim unused bits.

In compiled languages like Go, the code implementing the runtime is directly inserted into the resulting executable. If a language is run inside an interpreter like Python, the runtime lives there.
## Alternate Paths
### Tree-walk interpreters
The interpreter traverses the abstract syntax tree and evaluates each node as it goes. IR, code generation not required.
### Transpilers
Writing a complete backend for a language is a lot of work. Another method could be to write the front end of the language and in the backend, produce a string of valid source code for some other language that is about as high level and use the backend tools for that language to do the rest of the work e.g. Typescript to JavaScript.
## Interpreter vs Compiler
Compiling is an implementation technique that involves translating a source language to some other usually lower level form. Generating bytecode, transpiling are all examples of compiling.

An implementation "is an interpreter" if it takes source code and executes it immediately. Go is an interpreter: `go run`  compiles Go source code to machine code and runs it. Go is a compiler : `go build` compiles without running.
![](https://i.imgur.com/71QCQUY.png)
