# Threads
A thread (or lightweight process) consists of its own thread id, program counter, registers and stack space

__Multithreading__: A process can have multiple threads: this allows efficient sharing of memory for the program without having to create additional processes which has high overhead.
![](https://i.imgur.com/2wnSSnO.png)
## Implementation models
Want to support an arbitrary number of threads but the OS can only support a limited number due to physical constraints

Logical (user) threads: Created in user space and allows users to create as many threads as they want
Kernel threads (physical): Created in kernel space and slower to create and manage than user threads;   Resources are eventually allocated in kernel threads

Ways to map logical to physical:
- Many to one: can result in blockage of thread when one is in use
- One to one: creating user threads = creating kernel threads; not very efficient
- Many to many: not easy to decide an efficient mapping

## Practice Problems
*Explain the difference between a single-threaded and a multi-threaded process.*
- Threads in a process share code, data and heap regions of memory, whereas stack space is unique to each thread. Also, each thread has its own Thread Control Block (TCB), similar to a PCB.
- In a single-threaded process, there is only one thread of execution, and hence it is identical to a process. 
- In a multi-threaded process, the individual threads can execute concurrently, thus increasing system throughput; when one thread of a process is blocked (“waiting” state), another thread can continue its execution (“running” state).