---
title: "Process scheduling"
date: 2022-11-08
lastmod: 2022-11-21
---
# Process Scheduling
A process execution alternates between CPU executions and I/O operations
__CPU Burst__: duration of one CPU execution cycle
__I/O Burst__: duration of one I/O operation (wait time)
![](https://i.imgur.com/dztmS0B.png)
## Types
All processes are stored in queue structures
Job queue: set of all processes with the same state in the system
- ready queue: processes in the ready state
- device queue: processes waiting for specific I/O device

A scheduler will be in charge of handling these queues
![](https://i.imgur.com/fjJLGsE.png)

![](https://i.imgur.com/JWypMYa.png)
## Objectives
__System__:
1. Maximize CPU utilization
	- Increase the % of time which CPU cores are busy executing processes
	- $\frac{\text{Total execution time}}{\text{Total time}}$ 
2. Maximize throughput
	- Number of processes that complete their execution per unit time (number of exit transitions)
__Process__:
1. Minimize turnaround time
	- Amount of time to finish executing a particular process (e.g. time between admitted and exit transitions)
2. Minimize waiting time
	- Amount of time process is in the __ready__ state
	- __Turnaround time - CPU burst time
3. Minimize response time
	- Time between admission and first response (assume to be start of execution)
## Uni-Core Algorithms
### First Come First Serve (FCFS)
Non pre-emptive type: processes have to voluntarily release CPU once allocated
![](https://i.imgur.com/Zl2BurR.png)

__Convoy effect:__ Short processes suffer increased waiting times due to earlier arrived long processes
![](https://i.imgur.com/mNeQ2yS.png)
### Shortest Job First (SJF)
How to handle the convoy effect from FCFS? __Prioritize processes based on CPU burst lengths__

__Non pre-emptive__: a process cannot be stopped. Preemption only after a process is completed
__Pre-emptive__ (Shortest Remaining Time First): processes in the midst of execution can be rescheduled
![](https://i.imgur.com/aNa162L.png)
This algorithm is optimal to achieve minimum average waiting time. _However, this algorithm is often not used in practice as it is difficult to know the burst length of a process._
### Priority Based
CPU is allocated to the process with highest priority
1. Priority based on arrival order (FCFS)
2. Priority base on CPU burst length (SJF)

Starvation problem: lower priority processes may never execute. Need to use aging to slowly increase priority of processes that have been in the pipeline longer.
### Round Robin
Use a fixed time quantum (q) for scheduling. A process is allocated CPU for q time units and after that it is preempted and inserted at the end of the ready queue.

Large q: degenerates to FCFS
Small q: many context switches leading to greater overhead
![](https://i.imgur.com/EIaetjn.png)
_This is the algorithm that is used most commonly in practice_
![](https://i.imgur.com/wKh7X6w.png)
### Multi-Queue
![](https://i.imgur.com/REMdkaK.png)

![](https://i.imgur.com/dMrin7M.png)
## Multi-Core Algorithms
### Partitioned Scheduling
Each process are partitioned at process creation time among CPU cores
Each process is mapped to one core
__Asymmetric scheduling__: each CPU can have a separate scheduling strategy/algorithm

How to map cores to processes?
- Burst lengths are not easy to know
- For a CPU capacity, we need to maximize a certain property: similar to [Knapsack Problem](Notes/Knapsack%20Problem.md)
- Thus, partitioned scheduling suffers from unbalanced loading of cores
### Global Scheduling
Maintain 1 or more ready queues for the entire system without mapping any queue to any CPU core
__Symmetric scheduling__: one scheduling strategy/algorithm across all cores
![](https://i.imgur.com/iIndE5i.png)

![](https://i.imgur.com/CLAO4wv.png)

![](https://i.imgur.com/a7HDMCi.png)
## Practice Problems
Under Round-Robin scheduling, if quantum size is q, average CPU burst length is B, average number of CPU bursts per process is N, and average number of processes in the ready queue is R, then the average response time for a process is?
$$\frac{0+q+2q+3q+...+(R-1)q}{R} = \frac{\frac{R}{2}(R-1)q}{R}=(R-1)q$$
![](https://i.imgur.com/KuJF3Ze.png)
a. False. If the CPU cannot be removed from the process, it is non-preemptive
b. False. Only need to run scheduler when a process exits or changes to waiting
c. False. Response time is time to first start of execution. Turnaround time is time to finish the process. Waiting time is time in the ready state. Turnaround - waiting time is just the time in the waiting and running states combined.
d. False. Migration overheads occur in global scheduling when a process partially executes on one core and then migrates to another. In partitioned scheduling processes don’t migrate between cores. However, partitioned scheduling has the problem of unbalanced loading of the cores depending on the process-core mapping.
![](https://i.imgur.com/w1gOgdk.png)
abc.
![ 800](Excalidraw/Drawing%202022-08-28%2021.06.51.excalidraw.md)
d.
Uni-core: RR
Duo-core: RR, SRTF
![](https://i.imgur.com/SyhdEDn.png)
Efficiency is total process time over total process time + total overhead: $\frac{T}{T+kS}$
where k is the total number of context switches
*we should also include the 1st context switch needed to start process*
a. If $Q->\infty$, there will be 0 context switches 
$Efficiency=\frac{T}{T+S} = 1$
b. If Q >T, average process will run without context switching
$Efficiency=\frac{T}{T+S}= 1$
c. S < Q < T. Average process will switch T/Q times.
$Efficiency=\frac{T}{S\times\lceil\frac{T}{Q}\rceil}$
c. Q = S. Average process will switch T/S times.
$Efficiency=\frac{T}{T+\frac{T}{S}S} = \frac{T}{2T}=0.5$
d. Q -> 0, number of switches tend to infinity
$Efficiency=0$
