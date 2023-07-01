---
title: "2005 Operating Systems"
tags: [moc]
date: 2022-11-07
lastmod: 2023-07-01
---
# Operating Systems
#moc 
Essentially a piece of code which controls and coordinates the use of hardware among various programs for various users.
## The Boot Process
When you turn on a computer, it begins executing *firmware code* that is stored in motherboard [ROM](https://en.wikipedia.org/wiki/Read-only_memory). This code performs a [power-on self-test](https://en.wikipedia.org/wiki/Power-on_self-test), detects available RAM, and pre-initializes the CPU and hardware. Afterwards, it looks for a bootable disk and starts booting the operating system kernel.
### Boot Process Firmware
On x86, there are two firmware standards: the “Basic Input/Output System“ (**[BIOS](https://en.wikipedia.org/wiki/BIOS)**) and the newer “Unified Extensible Firmware Interface” (**[UEFI](https://en.wikipedia.org/wiki/Unified_Extensible_Firmware_Interface)**). The BIOS standard is old and outdated, but simple and well-supported on any x86 machine since the 1980s. UEFI, in contrast, is more modern and has much more features, but is more complex to set up
#### Bootloaders
When you turn on a computer, it loads the BIOS from some special flash memory located on the motherboard. The BIOS runs self-test and initialization routines of the hardware, then it looks for bootable disks. If it finds one, control is transferred to its *bootloader*, which is a 512-byte portion of executable code stored at the disk’s beginning. Most bootloaders are larger than 512 bytes, so bootloaders are commonly split into a small first stage, which fits into 512 bytes, and a second stage, which is subsequently loaded by the first stage.

The bootloader has to determine the location of the kernel image on the disk and load it into memory. It also needs to switch the CPU from the 16-bit real mode first to the 32-bit protected mode, and then to the 64-bit long mode, where 64-bit registers and the complete main memory are available. Its third job is to query certain information (such as a memory map) from the BIOS and pass it to the OS kernel.
## Types of OS
1. Batch Systems: batch similar jobs which automatically transfers control from one job to another
	- Only 1 job in memory at any time
	- When job waits for IO, the CPU is idle
2. Multiprogram / Time-sharing Systems: several jobs are kept in main memory at the same time
	- Goal: Improve CPU utilization by running more than one program concurrently even in a single-core CPU
	- Different from multiprocessing: increase computing power with parallel architectures
	- __Requires OS to be able to handle memory management, CPU and I/O scheduling for efficiency
3. Embedded Systems: physical systems where operations are controlled by computing
	- Examples:
	- Real time systems: have jobs that must complete without well-defined fixed time constraints (e.g. car airbag deployment)
	- Handheld systems

## Functions of the OS
1. [IO Subsystem](Notes/IO%20Subsystem.md)
2. [Direct Memory Access](Notes/Direct%20Memory%20Access.md)
3. [Interrupts](Notes/Interrupts.md)
4. [Hardware Protection](Notes/Hardware%20Protection.md)
5. Handle [Processes](Notes/Processes.md)
6. [Process scheduling](Notes/Process%20scheduling.md)
7. [Process Synchronization](Notes/Process%20Synchronization.md)
8. [Deadlocks](Notes/Deadlocks.md)
9. [Real Time Operating Systems](Notes/Real%20Time%20Operating%20Systems.md)
10. [Virtualization](Notes/Virtualization.md)
11. [Memory Organisation](Notes/Memory%20Organisation.md)
12. [Virtual Memory](Notes/Virtual%20Memory.md)
13. [File Systems](Notes/File%20Systems.md)
## References
### Operating Systems Concepts
Exercise solutions:
- https://codex.cs.yale.edu/avi/os-book/OS10/practice-exercises/index-solu.html
Instructor's Manual:
- http://web.uettaxila.edu.pk/CMS/AUT2011/seOSbs/tutorial/Sol.%20-%20Silberschatz.Galvin%20-%20Operating.System.Concepts.7th.pdf
