# Interrupts
An interrupt is a request for the processor to interrupt currently executing code so that the event can be processed in a timely manner. 

A trap is a CPU generated interrupt caused by a software error or a request: __unhandled exceptions in a program used to transfer control back to the [[2005 Operating Systems]]

## Interrupt Service Routine
### Interrupt Handling
1. OS preserves the state of the CPU by storing registers and the Program Counter
2. Determines the type of interrupt that has occurred using the segments of code
3. Executes the appropriate ISR based on the interrupt vector table