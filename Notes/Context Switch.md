---
title: "Context Switch"
date: 2022-11-08
lastmod: 2022-11-21
---
# Context Switch
OS preserves the state of the CPU by storing registers and the Program Counter
__Context switch time is overhead__: note that there is time spent where both processes are idle
![](https://i.imgur.com/7cFAtio.png)
## Interrupt Stack Table (IST) and Task State Segment (TSS)
The 64-bit TSS has the following format:

|Field|Type|
|---|---|
|(reserved)|`u32`|
|Privilege Stack Table|`[u64; 3]`|
|(reserved)|`u64`|
|Interrupt Stack Table|`[u64; 7]`|
|(reserved)|`u64`|
|(reserved)|`u16`|
|I/O Map Base Address|`u16`|

The _Privilege Stack Table_ is used by the CPU when the privilege level changes. For example, if an exception occurs while the CPU is in user mode (privilege level 3), the CPU normally switch to kernel mode (privilege level 0) before invoking the exception handler.