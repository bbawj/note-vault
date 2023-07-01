---
title: "Interrupts"
date: 2022-11-08
lastmod: 2022-11-21
---
# Interrupts
An interrupt is a request for the processor to interrupt currently executing code so that the event can be processed in a timely manner. It usually refers to **hardware interrupts** triggered by, for example, USB controllers, which generate them on the basis of some event. Interrupts can also include [Exceptions](Notes/Exceptions.md), which are triggered by the CPU itself.
## Interrupt Service Routine
### Interrupt Handling
1. [Context Switch](Notes/Context%20Switch.md) 
2. Determines the type of interrupt that has occurred using the segments of code
3. Executes the appropriate ISR based on the interrupt vector table
![](https://i.imgur.com/Dd7mo5y.png)
The ISR is a very short routine so as to not suspend the main program for too long. This usually means no usage of loops.
- Allows for efficient use of CPU as it does not need to monitor I/O device status
- More hardware is required between I/O and processor to interface with each other
