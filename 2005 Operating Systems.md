# Operating Systems
#moc 
Essentially a piece of code which controls and coordinates the use of hardware among various programs for various users.
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

The OS is typically [[Interrupts]] driven.

## Functions of the OS
1. [[Direct Memory Access]]
2. [[Hardware Protection]]
3. Handle [[Processes]]
4. [[Process scheduling]]
5. [[Process Synchronization]]
6. [[Deadlocks]]
7. [[Real Time Operating Systems]]
8. [[Virtualization]]
9. [[Memory Organisation]]
10. [[Virtual Memory]]
11. [[File Systems]]
12. [[I/O Subsystem]]