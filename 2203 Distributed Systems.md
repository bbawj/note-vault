---
title: "2203 Distributed Systems"
date: 2023-01-21
---
# 2203 Distributed Systems
#moc 
- [[Distributed Abstractions]]
- [Failure Detectors](Notes/Failure%20Detectors.md)
- [[Broadcast Abstractions]]
- [[Distributed Shared Memory]]
- [[Consensus]]
- [[Time Abstractions]]
## What are distributed systems
A set of nodes, connected by a network, which appear to its users as a single coherent system.
![](https://i.imgur.com/9tNONlM.png)
## Core problems
### Agreement
#### Two generals problem
“Two generals need to coordinate an attack”  
- Must agree on time to attack  
- They’ll win only if they attack simultaneously  
- Communicate through messengers  
- Messengers may be killed on their way
![](https://i.imgur.com/y9Jt5pm.png)
Generals are unable to come to an agreement within a specified time bound using unreliable communication channels.
#### Consensus Problem
All nodes/processes propose a value  
Some nodes (non correct nodes) might crash & stop responding  
The algorithm must ensure a set of properties (specification):  
- All correct nodes eventually decide  
- Every node decides the same  
- Only decide on proposed values
![](https://i.imgur.com/czyoLXJ.png)
![](https://i.imgur.com/Ir3zMp7.png)
This problem models the core issue in distributed databases known as **atomic commits**, where we choose to commit if every node agrees to commit and abort if at least one node aborts. It is a consensus with 2 values {commit, abort}.
#### Broadcast Problem
Atomic Broadcast  
- A node broadcasts a message  
- If sender correct, all correct nodes deliver message
- All correct nodes deliver the same messages (consensus) 
- Messages delivered in the same order
> [!Note]
> Atomic broadcast can be used to solve consensus in the following way:
> 1. Decide on the first received proposal
> 2. Since all messages are in the same order, all nodes will decide the same
> 
> Consensus can be solved by Atomic broadcast
> 
> *Atomic broadcast is equivalent to Consensus*
## Modelling Distributed Systems
### Timing assumptions
- Processes: bounds on time to make a computation step
- Network: bounds on time to transmit a message
- Clocks: lower and upper bounds on clock drift rate
### Failure assumptions
- Processes: what kind of failure?
- Network: can network drop messages, temporarily disconnect?
### Asynchronous System Model
- No bound on time to deliver a message  
- No bound on time to compute  
- Clocks are not synchronized
### Synchronous system  
- Known bound on time to deliver a message (latency)  
- Known bound on time to compute  
- Known lower and upper bounds in physical clock drift rate  
Examples:  
- Embedded systems (shared clock)  
- Multicore computers
## Measuring Performance
### Message complexity
The number of messages required to terminate an operation of an abstraction
### Time complexity (Rounds)
One time unit in an Execution E is the longest message delay in E. We assume all communication steps takes one time unit. We also call this a round or step.

Time Complexity is Maximum time taken by any execution of the algorithm under the assumptions  
- A process can execute any finite number of actions (events) in zero time  
- The time between send(m)i,j and deliver(m)i,j is at most one time unit  
