---
title: "Multitasking"
date: 2023-07-21
lastmod: 2023-07-21
---
# Multitasking
Multitasking is the ability to execute multiple tasks concurrently. Only a single task can be executed on a CPU core at a time. To create the illusion that the tasks run in parallel, the operating system rapidly switches between active tasks so that each one can make a bit of progress.
## Pre-emptive Multitasking
The operating system controls when to switch tasks. This is made possible by [interrupts](Notes/Interrupts.md), which allows the OS to regain control of the CPU on each interrupt.
![](Pics/Pasted%20image%2020230721233556.png)
Since tasks are interrupted at arbitrary points in time, they might be in the middle of some calculations. In order to be able to resume them later, the operating system must backup the whole state of the task, including its [call stack](https://en.wikipedia.org/wiki/Call_stack) and the values of all CPU registers via a [context switch](Notes/Context%20Switch.md).
### Advantages
- make it possible to run untrusted userspace programs
### Disadvantages
- each task requires its own stack. Compared to a shared stack, this results in higher memory usage per task and often limits the number of tasks in the system
- the operating system always has to save the complete CPU register state on each task switch, even if the task only used a small subset of the registers.
## Cooperative Multitasking
Instead of forcibly pausing running tasks at arbitrary points in time, cooperative multitasking lets each task run until it voluntarily gives up control of the CPU. This allows tasks to pause themselves at convenient points in time, for example, when they need to wait for an I/O operation anyway.

Cooperative multitasking is often used at the [language level](Notes/Asynchronous%20Programming.md), like in the form of [coroutines](https://en.wikipedia.org/wiki/Coroutine)or [async/await](https://rust-lang.github.io/async-book/01_getting_started/04_async_await_primer.html). The idea is that either the programmer or the compiler inserts [_yield_](https://en.wikipedia.org/wiki/Yield_(multithreading)) operations into the program, which give up control of the CPU and allow other tasks to run. For example, a yield could be inserted after each iteration of a complex loop.
### Advantages
- Since tasks define their pause points themselves, they don’t need the operating system to save their state. Instead, they can save exactly the state they need for continuation before they pause themselves, which often results in better performance.
### Disadvantages
- an uncooperative task can potentially run for an unlimited amount of time. Thus, a malicious or buggy task can prevent other tasks from running and slow down or even block the whole system