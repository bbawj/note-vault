---
title: "Consensus"
date: 2023-02-16
lastmod: 2023-02-16
---
# Consensus
Processes propose values and they all have to agree on one of these values.
Single Value Consensus
- Validity: decided values are those which are proposed
- Agreement: no 2 *correct* processes decide differently
- Termination: every correct process eventually decides
- Integrity: a process decides at most once
Single Value Uniform Consensus
- **Uniform** Agreement: no *2 processes* decide different values
## Paxos Algorithm
An [Eventual Leader Election](Notes/Failure%20Detectors.md#Eventual%20Leader%20Election) (weakest leader elector we can use) can be used to elect 1 single proposer.
- Proposers: attempt to impose their proposal to acceptors
- Acceptors: may accept values issued by proposers (cannot talk to each other)
- Learners: decide depending on what is accepted
Contention problem: several processes might initially be proposers
### Abortable Consensus
Algorithm aborts if there is contention of multiple proposers. 
#### Version 1 (Centralised)
Proposer sends value to a central acceptor. Acceptor decides on the first value which it gets. 
Problem 1: if this acceptor fails, we will never know of the decision
#### Version 2 (Decentralised)
Proposers talk to a set of acceptors, use a majority [quorum](Notes/Distributed%20Abstractions.md#Quorums) to choose a value and enable fault tolerance.
Problem 2: acceptor accepts the first proposal, if messages arrive out of order, possible to have no majority ![](Pics/Pasted%20image%2020230216145932.png)
#### Version 3 (Enable restarts)
If no majority value, we need to restart until there is one.
Since proposers can propose again, we need a way to differentiate between them.
- Use a ballot number: sequence number in the form $i, n+i, 2n+i$ for a process $i$ and $n$ processes
Problem 3: restarts lead to different majority accepted values across time, learners cannot make a single decision 
![](Pics/Pasted%20image%2020230216150901.png)
#### Version 4
Proposers query acceptors so that if a value is accepted, every higher proposal issued has the same value previously accepted
1. Proposer $prepare(n)$:
	- Gets a promise from acceptors not to accept a proposal less ballot number n
	- Acceptor also responds with the value corresponding to the highest ballot number proposal
2. Proposer $accept(n,v)$:
	- Pick the value from the maximum proposal number returned. If none of the processes return a value, proposer can pick freely.
 3. Acceptor $accept(n,v)$ if not accepted any $prepare(m)$ such that $m>n$; else $reject$
4. Proposer $decide(v)$ if majority acks; else $abort$ 
![](Pics/Pasted%20image%2020230216162304.png)

![](Pics/Pasted%20image%2020230216162327.png)
#### Optimisations
- Reject `prepare(n)` if accepted `prepare(m); m > n`
- Reject `accept(n,v)` if answered `accept(m,u); m > n`
- Reject `prepare(n)` if answered `accept(m,u); m > n` 
- Ignore messages if majority obtained 
## Multi Paxos
The motivation: replicated state machines need to agree on a sequence of commands to execute.
Initial states
- $ProCmds = \emptyset$: stores the list of commands proposed
- Log = <>: a log of decided commands
A process which wants to execute a command C triggers $rb-broadcast<C, Pid>$. On delivery, the command pair is added to `ProCmds` unless it is already in Log.
![](Pics/Pasted%20image%2020230216175839.png)
Problem: the same command across multiple processes might be decided in different slots in time.
## Sequence Consensus
Rather than agreeing on a single command and storing that in a Log, we can directly try to agree on the sequence of commands.
- Validity: if process p decides on a value, the value is a sequence of commands
- Uniform Agreement: if process p decides u and another decides v, then *one is a prefix of the other*
- Integrity: process can later decide another value, but the *previous value is a strict prefix of the newly decided value*
- Termination
After adopting a value with highest proposal number, the proposer is allowed to extend the sequence with the new command. 
![](Pics/Pasted%20image%2020230216180512.png)