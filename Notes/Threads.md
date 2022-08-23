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
Explain the difference between a single-threaded and a multi-threaded process.
Multi-threaded process splits up its resources into individuals components that have its own registers and stack. This allows 1 process to be able to carry out multiple sub-tasks and share memory efficiently.