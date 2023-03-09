---
title: "Failure Detectors"
date: 2023-01-31
lastmod: 2023-03-08
---
# Failure Detectors
![](https://i.imgur.com/pIX6LVN.png)
## Failure Models
- Fail-stop: can reliably detect failure (achievable in synchronous model)
- Fail-noisy: can eventually detect failure (achievable in partially synchronous model)
- Fail-silent: cannot detect between a crash or omission failure (asynchronous model)
## Properties
### Completeness
No false negatives: all failed processes are suspected
*Asynchrony: suspect every node to achieve completeness*
#### Strong completeness
![](https://i.imgur.com/pNaTp6N.png)
#### Weak completeness
![](https://i.imgur.com/2wh0Xxc.png)
### Accuracy
No false positives: correct processes are not suspected
*Asynchrony: suspect 0 nodes to achieve accuracy*
#### Strong accuracy
No correct process is ever suspected
#### Weak accuracy
There exists a correct process *P* which is never suspected by any process
#### Eventual Strong accuracy
After some time, strong accuracy achieved, prior to this, any behaviour possible.
- This does not satisfy weak accuracy, as before achieving strong accuracy, any behaviour allowed
#### Eventual Weak accuracy
After some time, weak accuracy achieved, prior to this, any behaviour possible. 
## Perfect failure detector
![](https://i.imgur.com/3kL3gD1.png)
Only implementable in the [Synchronous system](2203%20Distributed%20Systems.md#Synchronous%20system), else there will be some incorrectly suspected processes while figuring out the actual delay.
![](https://i.imgur.com/dssXVZU.png)
## Eventually perfect failure detector
![](https://i.imgur.com/3kKKWFY.png)
![](https://i.imgur.com/3DzXuIe.png)
How to achieve strong accuracy?
Each time p is inaccurately suspected by a correct q  
- Timeout T is increased at q  
- Eventually system becomes synchronous, and T becomes larger than the unknown bound δ (T>γ+δ)  
- q will receive HB on time, and never suspect p again
## Leader Election
We want all processes to detect a single and same correct process. To do so, we need to define the set of failed processes (using a FD)
![](https://i.imgur.com/uD2uuF0.png)
### Why local accuracy?
![](https://i.imgur.com/5p2EFci.png)
![](https://i.imgur.com/lIBmUQl.png)
### Implementation
![500](https://i.imgur.com/azDuheQ.png)
## Eventual Leader Election
![](https://i.imgur.com/vi4fjhY.png)
![500](https://i.imgur.com/A2Fj9bn.png)
## Reductions
![400](https://i.imgur.com/vRgHdn4.png)

![](https://i.imgur.com/GzKPYmu.png)
### Strong completeness equivalent to weak completeness
![](https://i.imgur.com/4qlsXBC.png)

![](https://i.imgur.com/7cBJV1R.png)
- If strong accuracy, no one is ever inaccurate, reduction never spreads inaccurate Susp
- If weak accuracy, everyone is accurate about at least one process p, no one spreads inaccurate information about p
### Eventual leader election $\Omega\equiv \diamond S$ 
Implement S using $\Omega$:
- Strong completeness $\equiv$ weak completeness: if we suspect everyone (except the leader which we know is correct), every process suspects all incorrect processes
- Eventual weak accuracy: if we only trust the leader, there exists 1 correct process not suspected
### Summary
![](https://i.imgur.com/UaULaGC.png)
