---
title: "Memory Safety"
date: 2023-05-11
---
# Memory Safety
Safe memory access is when each memory location that is used must have been
- allocated (statically, on stack, or on heap),
- initialized (write before read).
- dynamically allocated memory must be freed exactly once.
- No memory exhaustion.
## Memory Corruption
Exploited to get access to protected data, or overwrite important data that governs control flow; may hijack process:
1. Access to an unallocated memory region, or a region outside given buffer.
2. May read uninitialized memory, or write to memory used by other buffer.
## Memory Leaks
A serious issue for long running programs.

Allocated memory is not freed but also never used again:
1. Lost for good: memory is no longer reachable, perhaps due to losing the only pointer to that memory. May be garbage collected.
2. Potentially lost: memory is still reachable (the program still holds a pointer), and hence may not be garbage collected. 
## Memory Checking Tools
Popular tool: [valgrind](http://valgrind.org/).
- Mature memory checker, used in many projects.
- Finds any problems related to heap-allocated memory and some stack allocated cases.
### Input generation
These tools use different inputs to attempt to find one which can cause a memory error.
- Unit testing: able to model specific input sequences but difficult to cover many inpits
- Random testing: automatically generate many inputs but can still result in shallow coverage if the specific input format is not followed (by generating alot of noise that is not relevant to the program).
- Fuzz testing: generates many *slightly invalid* inputs based on format of valid examples. 