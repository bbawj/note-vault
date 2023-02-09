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
Uses perfect [perfect failure detector P](Notes/Failure%20Detectors.md#Perfect%20failure%20detector):
write(v)
1. Update local value to v
2. [Fail Stop Broadcast](Notes/Broadcast%20Abstractions.md#Fail%20Stop) v to all
3. Wait for ACK from all correct processes
4. Return
read
1. Return local value
![500](https://i.imgur.com/ms7UOou.png)
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
Problem: ![](https://i.imgur.com/cjeoDqd.png)
Before writing, read from majority to get the latest timestamp (query phase before update phase). 
![](https://i.imgur.com/PySj7Kq.png)


