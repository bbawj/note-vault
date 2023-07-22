---
title: "Context Switch"
date: 2022-11-08
lastmod: 2022-11-21
---
# Context Switch
OS preserves the state of the CPU by backing up the whole state of the task, including the call stack, storing registers and the program counter.

__Context switch time is overhead__: note that there is time spent where both processes are idle
![](https://i.imgur.com/7cFAtio.png)
As the call stack can be vary large, the OS typically sets up a separate call stack for each task instead of having to back up the entire call stack content on each task switch. Such a task with its own call stack is a [thread](Notes/Threads.md).
