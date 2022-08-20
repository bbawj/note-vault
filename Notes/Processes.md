# Processes
A program in execution, also known as a job.
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

## Inter-Process Communication (IPC)
![](https://i.imgur.com/sayPPYP.png)
### Message passing
No use of shared variables. 
Works through 2 system calls:
1. send(message) 
2. receive(message)

Direct: processes must name each other explicitly 
Indirect: messages are sent and received through a mailbox or port 
![](https://i.imgur.com/OZdrt4M.png)
