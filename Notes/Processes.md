# Processes
A program in execution, also known as a job. One process can have multiple [[Threads |threads]].
## Process in memory
![](https://i.imgur.com/DnW6ijk.png)

Information related to each process is then stored in a Process Control Block (PCB)
![](https://i.imgur.com/K8z95Jd.png)
## Process states
1. New: The process is being created 
2. Running: Instructions are being executed 
3. Waiting: The process is waiting for I/O or event 
4. Ready: The process is ready to run, but is waiting to be assigned to the CPU 
5. Terminated: The process has completed
![](https://i.imgur.com/p6tLvdK.png)
> [!Notes]
> - Timer interrupt is used in multiprogramming systems to switch between ready processes
> - Running -> waiting is due to some interrupt source from the currently running process
> - Process always goes through the ready state before running (no direct from waiting to running)
## Process scheduling
All processes are stored in queue structures
Job queue: set of all processes with the same state in the system
- ready queue: processes in the ready state
- device queue: processes waiting for specific I/O device

A scheduler will be in charge of handling these queues
![](https://i.imgur.com/fjJLGsE.png)
## Process Operations
### Creation
![](https://i.imgur.com/Di9N9yE.png)
This creation process can have 2 types:
- Parent and child execute concurrently
- Parent waits for all children to terminate before continuing execution. This is done through system calls `wait()`
### Termination
1. Exit: Process asks the OS to delete it
2. Abort: Parent terminates children processes
## Inter-Process Communication (IPC)
![](https://i.imgur.com/sayPPYP.png)
### Message passing
No use of shared variables. 
Works through 2 system calls:
1. `send(message) 
2. `receive(message)

Direct: processes must name each other explicitly 
Indirect: messages are sent and received through a mailbox or port 
![](https://i.imgur.com/OZdrt4M.png)
## Practice Problems
![](https://i.imgur.com/MROpdFs.png)
a. False. That is the ready state. The waiting state is for processed waiting for some I/O or event not CPU
b. True
c. False. It is used by the parent to wait for children to finish.
d. Not sure actually. If message passing uses a mailbox approach it should use the same amount of memory

What are two main differences between the data and stack regions of a process memory?
1. Data region is used to store global variables while stack region is used to store the currently executing local functions and parameters.
2. Data region is fixed, while the stack can grow and shrink as the program executes.
![](https://i.imgur.com/ObBUEZ0.png)
P0: Ready -> Running -> Ready
P1: Running -> Waiting -> Ready -> Running

[[Context Switch]]
A: Save state of P1 into PCB1
B: Load state of P0 from PCB0
C: Save state of P0 into PCB0
D: Reload state of P1 from PCB1