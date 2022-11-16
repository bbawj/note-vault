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

The OS is typically [Interrupts](Notes/Interrupts.md) driven.

## Functions of the OS
1. [Direct Memory Access](Notes/Direct%20Memory%20Access.md)
2. [Hardware Protection](Notes/Hardware%20Protection.md)
3. Handle [Processes](Notes/Processes.md)
4. [Process scheduling](Notes/Process%20scheduling.md)
5. [Process Synchronization](Notes/Process%20Synchronization.md)
6. [Deadlocks](Notes/Deadlocks.md)
7. [Real Time Operating Systems](Notes/Real%20Time%20Operating%20Systems.md)
8. [Virtualization](Notes/Virtualization.md)
9. [Memory Organisation](Notes/Memory%20Organisation.md)
10. [Virtual Memory](Notes/Virtual%20Memory.md)
11. [File Systems](Notes/File%20Systems.md)
12. [IO Subsystem](Notes/IO%20Subsystem.md)
## References
### Operating Systems Concepts
Exercise solutions:
- https://codex.cs.yale.edu/avi/os-book/OS10/practice-exercises/index-solu.html
Instructor's Manual:
- http://web.uettaxila.edu.pk/CMS/AUT2011/seOSbs/tutorial/Sol.%20-%20Silberschatz.Galvin%20-%20Operating.System.Concepts.7th.pdf