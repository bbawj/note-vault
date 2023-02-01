---
title: "Distributed Abstractions"
date: 2023-01-31
---
# Distributed Abstractions
The basic building blocks of any distributed system is a set of distributed algorithms. which are implemented as a middleware between network (OS) and the application.  
![](https://i.imgur.com/78JtdFf.png)
## Event based Component Model
The distributed computing model consists of a set of processes and a network. Events can be used as messages between components of the same process, which trigger event handlers.
Types of events
- Requests: incoming to component
- Indications: outgoing from component
![](https://i.imgur.com/uhDwRCU.png)
## Specifications
### Interface
This specifies the API, importantly, the requests and events of the service
![](https://i.imgur.com/dzfSvde.png)
### Correctness Properties
![](https://i.imgur.com/oVSfzzE.png)
#### Safety
Properties that state that nothing bad ever happens. It can only be:
- satisfied in infinite time (you cannot be sure you are safe)
- violated in finite time (when bad happens)
![](https://i.imgur.com/Ym9AsqW.png)
The **prefix** of a trace T is the first k (for k ≥ 0) events of T  
- cut off the tail of T  
- finite beginning of T  
An **extension** of a prefix P is any trace that has P as a prefix
>[!Formal definition]
> A property P is a safety property if given any execution E such that P(trace(E)) = false, there exists a prefix of E, s.t. every extension of that prefix gives an execution F s.t. P(trace(F))=false

![500](https://i.imgur.com/9w2eSWC.png)
#### Liveliness
Properties that state that something good eventually happens. It can only be:
- satisfied in finite time (when good happens)
- violated in infinite time (there is always hope)
>[!Formal definition]
>A property P is a liveness property if given any prefix F of an execution E, there exists an extension of trace(F) for which P is true

![500](https://i.imgur.com/gZEIagM.png)
## Model/Assumptions
### Failure assumptions
Processes that do not fail in an execution are **correct**.
#### Crash stop failure
Process stops taking steps like sending and receiving messages.
#### Omission failure
Process omits sending or receiving messages
- Send omission: not sending messages according to algorithm
- Receive omission: not receiving messages that have been sent to the process
#### Crash recovery
Process might crash and
- never recover, or
- recovers an infinite number of times
 
May recover after crashing with a special recovery event automatically generated
- Crash recovery == omission: *By crashing, recovering and reading the last state from storage, crash recovery is equivalent to omitting the send/receive during the crash duration.*
Access to stable storage to read and write to permanent device which survives the crash
- Omission $\subset$ Crash recovery: volatile memory extends omission with amnesia by not being able to restore all the state.
#### Byzantine
Process behaves arbitrarily such as sending messages not in its algorithm, or behave maliciously attacking the system.
![500](https://i.imgur.com/YPA4gvB.png)
Model B is a special case of model A if a process that works correctly under A, also works correctly under B.
### Channel Failure Modes
#### Fair loss links
Channels delivers any message sent with non-zero probability (no network partitions)
![500](https://i.imgur.com/m28zEgQ.png)
![500](https://i.imgur.com/g3BpSDt.png)
#### Stubborn links
Channels delivers any message sent infinitely many times
![500](https://i.imgur.com/RbpQui1.png)
![500](https://i.imgur.com/JeNQufn.png)
We can implement stubborn links using fair loss links
- sender stores every message it sends in *sent*
- periodically resends all messages in *sent*
![](https://i.imgur.com/Eb9gQyT.png)
#### Perfect Links
Channels that deliver any message sent exactly once.
![500](https://i.imgur.com/ixku41P.png)
![500](https://i.imgur.com/o41bsVf.png)
![500](https://i.imgur.com/5lsk0Da.png)
### Timing assumptions
Processes: bounds on time to make a computation step  
Network: Bounds on time to transmit a message between a sender and a receiver  
Clocks: Lower and upper bounds on clock rate-drift and  clock skew w.r.t. real time

Asynchronous systems: no timing assumption on process and channels
Partially synchronous systems: eventually every execution will exhibit synchrony
Synchronous systems: build on solid timed operations and clocks
#### Causality
![](https://i.imgur.com/zyGQcSe.png)
![](https://i.imgur.com/LHEtnUb.png)
#### Logical Clocks
A logical clock is an algorithm that assigns a timestamp to each event occurring in a  
distributed system.
$$if  \ a\rightarrow b, t(a)<t(b)$$
![](https://i.imgur.com/0rlOoTm.png)
#### Lamport clocks:
![500](https://i.imgur.com/Nr21gJZ.png)
![500](https://i.imgur.com/GPVztK0.png)
We need to distinguish the total order of events for same timestamps across different processes.
![500](https://i.imgur.com/td2qdsA.png)
#### Vector clocks
We want to tell the causal relation using the timestamps.
$$
\begin{align}
v(a) < v(b), then\ a\rightarrow_\beta b \\
if\ a\rightarrow_\beta b,v(a)<v(b)
\end{align}
$$
![](https://i.imgur.com/UK0RvdY.png)
![](https://i.imgur.com/8gYdatT.png)
![](https://i.imgur.com/hL7Y497.png)
![](https://i.imgur.com/FWREtoM.png)
Limitations
- Vectors need to be defined of size n
- cannot provide total event ordering
#### Similarity
If two executions F and E have the same collection of events, and their causal order is preserved, F and E are said to be similar executions, written `F~E`
![](https://i.imgur.com/NpvZWmp.png)
## [[Failure Detectors]]