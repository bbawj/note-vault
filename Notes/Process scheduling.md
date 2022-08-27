# Process Scheduling
A process execution alternates between CPU executions and I/O operations
__CPU Burst__: duration of one CPU execution cycle
__I/O Burst__: duration of one I/O operation (wait time)
![](https://i.imgur.com/dztmS0B.png)
## Types
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
This algorithm is optimal o achieve minimum average waiting time. _However, this algorithm is often not used in practice as it is difficult to know the burst length of a process._
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
- For a CPU capacity, we need to maximize a certain property: similar to [[Knapsack Problem]]

### Global Scheduling
Maintain 1 or more ready queues for the entire system without mapping any queue to any CPU core
__Symmetric scheduling__: one scheduling strategy/algorithm across all cores
![](https://i.imgur.com/iIndE5i.png)

![](https://i.imgur.com/CLAO4wv.png)

![](https://i.imgur.com/a7HDMCi.png)
## Practice Problems
Under Round-Robin scheduling, if quantum size is q, average CPU burst length is B, average number of CPU bursts per process is N, and average number of processes in the ready queue is R, then the average response time for a process is?
$$\frac{0+q+2q+3q+...+(R-1)q}{R} = \frac{\frac{R}{2}(R-1)q}{R}=(R-1)q$$