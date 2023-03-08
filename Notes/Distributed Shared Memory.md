---
title: "Distributed Shared Memory"
date: 2023-02-08
---
# Distributed Shared Memory
![](https://i.imgur.com/ITWox3G.png)
## Registers
A register represents each memory location. The operations are:
- write(r,v): update the value of register $x_r$ to v
- read(r): return the current value of register $x_r$
## Trace
A trace is a sequence of events
- $r-inv_i(r)$, $r-res_i(v)$: read invocation by process pi on $x_r$ register and the corresponding response with value $v$
- $w-inv_i(r)$, $w-res_i(v)$: write invocation by process pi on $x_r$ register and the corresponding response with value $v$
### Properties
Well-formed
- First event of every process is an invocation  
- Each process alternates between invocations and responses  
Sequential  
- 𝑥-inv by i immediately followed by a corresponding 𝑥-res at i  
- 𝑥-res by i immediately follows a corresponding 𝑥-inv by i, i.e. no concurrency, read x by p1, write y by p5, ...  
Legal  
- T is sequential  
- Each read to Xr returns last value written to register Xr
Complete
- Every operation is complete  
- Otherwise T is partial  
- An operation O of a trace T is  
	- complete if both invocation & response occurred in T  
	- pending if O invoked, but no response  
Precedence
- op1 precedes op2 in a trace T if (denoted <T)  
- Response of op1 precedes invocation of op2 in T  
- op1 and op2 are concurrent if neither precedes the other
## Regular Register Algorithms
A regular register is one that meets the following criteria:
Termination  
- Each read/write operation issued by a correct process eventually completes.  
Validity  
- Read returns last value written if  
- Not concurrent with another write, and  
- Not concurrent with a failed write  
- Otherwise may return last or concurrent “value”
![](https://i.imgur.com/9OyS87V.png)
### Fail-Stop Read-one Write-All
Uses [perfect failure detector P](Notes/Failure%20Detectors.md#Perfect%20failure%20detector):
write(v)
1. Update local value to v
2. [Fail Stop Broadcast](Notes/Broadcast%20Abstractions.md#Fail%20Stop) v to all
3. Wait for ACK from all correct processes
4. Return
read
1. Return local value
![500](https://i.imgur.com/ms7UOou.png)
[Eventually perfect failure detector](Notes/Failure%20Detectors.md#Eventually%20perfect%20failure%20detector) will not work here as it might falsely suspect some processes as having crashed. During this time, since a write on another process only waits for ACKs from all correct processes, it could return early. A read on the falsely suspected process will incorrectly return the old value.
### Fail silent Majority voting
Make use of timestamp-value pairs, tvp = (ts, v), where the timestamp can be used to determine which value is more recent. Each process stores the value of register r with max timestamp of each register r.

Majority idea is based of [quorums](Notes/Distributed%20Abstractions.md#Quorums).
- Read and write operation reads from quorums, this means at least 1 process knows the most recent value ![500](https://i.imgur.com/LrUlCMD.png)

![](https://i.imgur.com/xhFW67T.png)
![](https://i.imgur.com/jdpFdgF.png)

![](https://i.imgur.com/7chktQ2.png)
## Sequential Consistency
Allows executions whose results appear as if the operations of each processes were executed in some sequential order according to "local time" (we can reorder operations across processes but not locally):
![](https://i.imgur.com/ZcrgHbn.png)

![](https://i.imgur.com/UiMocib.png)
![](https://i.imgur.com/EJCOw3g.png)
### Liveness requirements
- Wait-free: no deadlocks, no livelocks, no starvation
- Lock-free: no deadlock, no livelocks, maybe starvation
- Obstruction-free: no deadlock, maybe livelocks and starvation
## Register Linearizability/Atomicity
Allows executions whose results appear as if the operations of each processes were executed in some sequential order according to "global time" (cannot reorder):
![](https://i.imgur.com/Q6ny756.png)

![](https://i.imgur.com/WNFn3kS.png)
### Read/Write Majority Problem
![](https://i.imgur.com/XaReYst.png)
### Read-Impose Write Majority
![](https://i.imgur.com/loWTmww.png)
![](https://i.imgur.com/YQDpASj.png)
![](https://i.imgur.com/tfvYJS7.png)
### Extending to N readers N writers (Read-impose Write-consult-majority)
Problem: 
![](https://i.imgur.com/cjeoDqd.png)
Before writing, read from majority to get the latest timestamp (query phase before update phase). 
![](https://i.imgur.com/PySj7Kq.png)
## Eventual Consistency
![](https://i.imgur.com/9Tdj6IH.png)
State updates can be issued at any replica/correct process. All updates are disseminated via BEB, RB,...  
- Each correct process that receives all updates should deterministically converge to the same state.  
- Eventually every correct process should receive all updates...  
- Problem: When can a process know it has received all updates??
### Strong Eventual Consistency
If state operations are **commutative** and processes exchange information, eventually they converge to an identical view.
![](https://i.imgur.com/2d68BHj.png)
![](https://i.imgur.com/vyWBePp.png)
## Conflict Free Replicated Data Types (CRDTs)
Data structures which implement strong eventual consistency.
![](https://i.imgur.com/rdcu52T.png)
![](https://i.imgur.com/jZ8OQqc.png)
The join operation allows there to be a commutative operation relationship between sets. However, operations need to have a strict monotonically increasing effect on the set.
### State Based CRDT (CvRDT)
![](https://i.imgur.com/3zI2hzY.png)
#### Grow-Only Counter
![](https://i.imgur.com/5u0Ry92.png)

![](https://i.imgur.com/B8tFa70.png)
#### Up-Down Counter
![](https://i.imgur.com/phCGsID.png)

![](https://i.imgur.com/KydUmYt.png)
#### Or-Set
![](https://i.imgur.com/idz0zR2.png)
![](https://i.imgur.com/6UZUPuV.png)
### Operation Based CRDTs (CmRDTs)
CmRDTs impose stricter assumptions. Causally dependent updates are replaced with [Causal Broadcast](Notes/Broadcast%20Abstractions.md#Causal%20Broadcast) and the join function is replaced with any commutative update function.
- Less states and IO required (only the operations are broadcasted)
- More restrictions to programming model leading to less flexibility
![](https://i.imgur.com/A8bZIfc.png)
#### Or-Set
![](https://i.imgur.com/Egu3eSa.png)
