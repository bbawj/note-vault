# Deadlocks
A set of blocked processes each holding a resource and waiting to acquire a resource held by another process in the set
## Modelling Deadlocks
![](https://i.imgur.com/VsAUubR.png)

![](https://i.imgur.com/XRHpl5v.png)

![](https://i.imgur.com/DT0bS6m.png)
### Cyclic Properties of Deadlocks
> [!Having a cycle in the graph is only a necessary condition but *not a sufficient condition* for a deadlock.]
![](https://i.imgur.com/FIkraZH.png)

If each resource only has 1 instance, a cycle implies a deadlock.
## Deadlock Conditions
A deadlock **may** occur if these conditions hold at the same time:
1. Mutual exclusion: Only one process at a time can use a resource instance
2. Hold and wait: A process holding at least one resource is waiting to acquire additional resources held by other processes
3. No preemption: A resource can be released only voluntarily by the process holding it, after that process has completed its task
4. Circular wait: There exists a set {P0, P1, …, Pn} of waiting processes such that P0 is waiting for a resource that is held by P1, P1 is waiting for P2, …, Pn–1 is waiting for Pn, and Pn is waiting for P0
## Deadlock Prevention
If one of the above conditions are not satisfied, a deadlock will not occur.
Example using [[Notes/Process Synchronization#Dining Philosophers|Dining Philosophers Problem]]:
![](https://i.imgur.com/6KG4dAv.png)
![](https://i.imgur.com/H79K3yy.png)
Each process must request for the lower numbered resource first before able to request for the higher numbered resource. This breaks the circular wait as process requests are increasing in their order (no cycle):
![](https://i.imgur.com/guwTUp1.png)
## Deadlock Avoidance
Rather than prevent deadlocks as they are about to occur, we can avoid entering into a state where deadlocks are possible. This state is called the *unsafe state*. 
![](https://i.imgur.com/0jvzJaO.png)

> [! Safe state]
>  if there exists a safe completion sequence of all processes without deadlock

A process completion sequence is safe, if for each $P_i$ , the resources that it requests can be satisfied by currently available resources + resources held by all the $Pj , j< i$
![](https://i.imgur.com/BM2vXln.png)

Algorithm:
1. When a process request for resource, determine if allocation leaves the system in a safe state
2. If safe: grant the request
3. Else: wait
### Banker's Algorithm
Checking whether the satisfaction of a request will lead to an unsafe state

Necessary assumptions:
1. Each process must declare the maximum instances of each resource type that it needs
2. When a process gets all its resources it must return them in a finite amount of time
We need to keep track of some information:
Let m be the number of resource types and n be number of processes
- Available: `Available = [m]int` the number of instances of each resource currently available to be allocated
- Max: `Max[n][m]` is number of resource a process can request. *Note*: the process completes once it reaches its max
- Allocation: current allocated resources
- Need: number of instances required to complete
![](https://i.imgur.com/tdiWbm8.png)
Each process can also make a request for resources:
![](https://i.imgur.com/mhB7FRp.png)

![](https://i.imgur.com/0LPoKIW.png)
## Deadlock Detection
Allow the system to enter deadlock state, invoke detection and recovery algorithms.
## Practice Problems
![](https://i.imgur.com/I4IuswX.png)
a. False. If there are only 4 people, the circular wait condition is broken
b. True. A single process will not be in a deadlock as there are no other processes which it is sharing resources with
c. False. Not all cycles indicate a deadlock.
![](https://i.imgur.com/oLMMgoT.png)

![[Pics/Deadlocks 2022-09-18 10.53.26.excalidraw]]
![](https://i.imgur.com/2FgBtCS.png)
a. Available -> 1. P4 allocation = 5. No process can be satisfied with available 1. Unsafe state
b. Safe state. Completion order: P3, P4, P2, P1
![](https://i.imgur.com/z5mdNjh.png)
x = 0
| Process | Allocation | Need  | Available | Completed    |
| ------- | ---------- | ----- | --------- | ------------ |
| P0      | 2 1 1      | 0 1 0 | 0 1 0     | P0 Completed |
| P1      | 1 1 0      | 2 1 2 | 3 3 2     | P1 Completed |
| P2      | 1 1 1      | 2 0 1 | 2 2 1     | P2 Completed |
| P3      | 1 1 1      | 4 1 0 | 4 4 2     | P3 Completed             |
