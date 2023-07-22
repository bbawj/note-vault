---
title: "Interrupts"
date: 2022-11-08
lastmod: 2022-11-21
---
# Interrupts
An interrupt is a request for the processor to interrupt currently executing code so that the event can be processed in a timely manner. It usually refers to **hardware interrupts** triggered by, for example, USB controllers, which generate them on the basis of some event. Interrupts can also include [Exceptions](Notes/Exceptions.md), which are triggered by the CPU itself.
## Interrupt Controller
Connecting all hardware devices directly to the CPU is not possible. Instead, a separate _interrupt controller_ aggregates the interrupts from all devices and then notifies the CPU:
```
                                    ____________             _____
               Timer ------------> |            |           |     |
               Keyboard ---------> | Interrupt  |---------> | CPU |
               Other Hardware ---> | Controller |           |_____|
               Etc. -------------> |____________|

```
### 8259 Programmable Interrupt Controller (PIC)
The 8259 has eight interrupt lines and several lines for communicating with the CPU. The typical systems back then were equipped with two instances of the 8259 PIC, one primary and one secondary PIC, connected to one of the interrupt lines of the primary:
```
                     ____________                          ____________
Real Time Clock --> |            |   Timer -------------> |            |
ACPI -------------> |            |   Keyboard-----------> |            |      _____
Available --------> | Secondary  |----------------------> | Primary    |     |     |
Available --------> | Interrupt  |   Serial Port 2 -----> | Interrupt  |---> | CPU |
Mouse ------------> | Controller |   Serial Port 1 -----> | Controller |     |_____|
Co-Processor -----> |            |   Parallel Port 2/3 -> |            |
Primary ATA ------> |            |   Floppy disk -------> |            |
Secondary ATA ----> |____________|   Parallel Port 1----> |____________|

```

This graphic shows the typical assignment of interrupt lines. We see that most of the 15 lines have a fixed mapping, e.g., line 4 of the secondary PIC is assigned to the mouse.
## Interrupt Service Routine
### Interrupt Handling
1. [Context Switch](Notes/Context%20Switch.md) 
2. Determines the type of interrupt that has occurred using the segments of code
3. Executes the appropriate ISR based on the interrupt vector table
![](https://i.imgur.com/Dd7mo5y.png)
The ISR is a very short routine so as to not suspend the main program for too long. This usually means no usage of loops.
- Allows for efficient use of CPU as it does not need to monitor I/O device status
- More hardware is required between I/O and processor to interface with each other
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