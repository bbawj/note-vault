---
title: "Failure Detectors"
date: 2023-01-31
---
# Failure Detectors
![](https://i.imgur.com/pIX6LVN.png)
## Properties
### Completeness
No false negatives: all failed processes are suspected
#### Strong completeness
![](https://i.imgur.com/pNaTp6N.png)
#### Weak completeness
![](https://i.imgur.com/2wh0Xxc.png)
### Accuracy
No false positives: correct processes are not suspected
#### Strong accuracy
No correct process is ever suspected
#### Weak accuracy
There exists a correct process *P* which is never suspected by any process
## Perfect failure detector
![](https://i.imgur.com/3kL3gD1.png)
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
![500](https://i.imgur.com/uwSCoBm.png)

![](https://i.imgur.com/GzKPYmu.png)
### Strong completeness equivalent to weak completeness
![](https://i.imgur.com/4qlsXBC.png)

![](https://i.imgur.com/7cBJV1R.png)
- If strong accuracy, no one is ever inaccurate, reduction never spreads inaccurate Susp
- If weak accuracy, everyone is accurate about at least one process p, no one spreads inaccurate information about p
![](https://i.imgur.com/UaULaGC.png)
