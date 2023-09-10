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
### Buffer Overflow
![](Pics/Pasted%20image%2020230911011103.png)
### Stack Smashing
Overwriting the return address on the frame with address to malicious program.
```c
#include <stdio.h>  
#include <string.h>  
void overflow(const char* input) {  
	char buf[256];  
	printf("Virtual address of 'buf' = Ox%p\n", buf);  
	strcpy(buf,input);  
}  
void attack() {  
	printf("'attack’ is called without explicitly invocation.\n");  
	printf("Buffer Overflow attack succeeded!\n");  
}  
int main(int argc, char* argv[]) {  
	printf("Virtual address of 'overflow' = Ox%p\n",overflow);  
	printf("Virtual address of 'attack' = Ox%p\n", attack);  
	char input[] = "..."; /* useless content as offset*/  
	char add[] = "\xf9\x51\x55\x55\x55\x55"; /* attack address*/  
	strcat(input, add);  
	overflow(input);  
	return 0;  
}
```
- Needs the absolute address of malicious code which can be infeasible. By inserting NOP instructions before the malicious code, it can improve the guess chance by allowing the program to advance until the address of the malicious program.

A simple way to obtaining shell access
![](Pics/Pasted%20image%2020230911011702.png)
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