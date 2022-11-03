# Interrupts
An interrupt is a request for the processor to interrupt currently executing code so that the event can be processed in a timely manner. 

> [!Traps]
A trap is a CPU generated interrupt caused by a software error or a request:
> - __unhandled exceptions in a program used to transfer control back to the [[2005 Operating Systems|OS]] __ 
> - user programs requesting execution of system calls which needs the OS
## Interrupt Service Routine
### Interrupt Handling
1. [[Context Switch]] 
2. Determines the type of interrupt that has occurred using the segments of code
3. Executes the appropriate ISR based on the interrupt vector table
![](https://i.imgur.com/Dd7mo5y.png)
The ISR is a very short routine so as to not suspend the main program for too long. This usually means no usage of loops.
- Allows for efficient use of CPU as it does not need to monitor I/O device status
- More hardware is required between I/O and processor to interface with each other