---
title: "Memory Safety"
date: 2023-05-11
---
Safe memory access is when each memory location that is used must have been
- allocated (statically, on stack, or on heap),
- initialized (write before read).
- dynamically allocated memory must be freed exactly once.
- No memory exhaustion.
# Memory Corruption
Exploited to get access to protected data, or overwrite important data that governs control flow; may hijack process:
1. Access to an unallocated memory region, or a region outside given buffer.
2. May read uninitialized memory, or write to memory used by other buffer.
## Buffer Overflow
![](Pics/Pasted%20image%2020230911011103.png)
A variant of buffer overflow is Stack Smashing, or stack buffer overflow. Following the above C code we can  overwrite the return address on the frame with address to malicious program:
![200x300](Pics/Pasted%20image%2020230920231925.png)  ![200x300](Pics/Pasted%20image%2020230920232012.png)
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

Needs the absolute address of malicious code which can be infeasible. By inserting NOP instructions before the malicious code, it can improve the guess chance by allowing the program to advance until the address of the malicious program.
### Compiler Support
#### Stack Guard
Key insight: difficult to modify the return address without overwriting stack memory in front of the return address. Generate a canary next to the return address and check it whenever a function returns:
![](Pics/Pasted%20image%2020230920233534.png)
Random Canary
- Choose random string at program startup
- Insert canary string into every stack frame.
- Verify canary before returning from function. If canary value is changed, then exit program (potential Denial-of-Service attack)
 - To corrupt, attacker must learn current random string
Terminator canary.
- Canary = {\0, newline, linefeed, EOF}
- String functions will not copy beyond terminator
- Attacker cannot use string functions to corrupt stack.
![](Pics/Pasted%20image%2020230921003100.png)
#### Point Guard
Protect the pointers from being overwritten with more performance overhead:
- Encrypt all pointers while in memory
- Generate a random key when program is executed
- Each pointer is XORed with this key when stored into memory
- Attacker cannot predict the target program’s key. Even if the pointer is overwritten, after XORing with the key it will point to a “random” memory address. This can prevent the execution of malicious functions, but can crash the program
#### Stack Shield
A GNU C compiler extension that protects the return address by separating the return address from data.
- Copy the return address to a non-overflowable area
- Restore the return address when returning from a function, altered return addresses on the stack will have no effect.
### OS Support
#### Address Space Layout Randomization
OS randomly arranges address space of key data areas for each program such as the base, stack, heap and library pointers, which makes guessing malicious program address harder.
![](Pics/Pasted%20image%2020230921003943.png)
#### Non-executable Memory
Executable memory regions allow attackers to inject a binary payload
For example, a simple way to obtaining shell access:
![](Pics/Pasted%20image%2020230911011702.png)
Mark all writable memory locations as non-executable. Some examples are ExecShield in Linux.
Code reuse attacks make use of already loaded functions, for example overwriting the return address to libc `execve` with `/bin/sh` as an argument.
![](Pics/Pasted%20image%2020230921004246.png)
#### Shadow Stack
Keep a copy of the stack in memory. Compare the actual and shadow stack to see if the return address has been changed.
#### Hardware Support
ARM Memory Tagging Extension (MTE):
- Every memory pointer and user memory region has a 4 bit-tag. These tags must match when reading from the pointer. Else, a hardware exception is raised.

Hardware can support an attribute in the Page Table Entry to control if the page is executable.
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
	- Mutation based: explore as many states as possible by changing inputs randomly, possibly guided by heuristics from an initial corpus of data. These are simple to set up but have low coverage.
	- Generative: use specification of input format (RFC etc.) into a generative procedure to make use of domain-specific knowledge but requires more effort
	- Coverage guided: use fuzzing to create new test cases. Test and measure code coverage and use the feedback to craft new inputs for uncovered code. This is good for finding new program states.