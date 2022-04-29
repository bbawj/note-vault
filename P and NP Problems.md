# P and NP Problems
_Hard Problems_: the best-known algorithm for the problem is expensive in terms of running time
## Classification of Problems
![](https://i.imgur.com/WYRjqTz.png)
### Decision vs Optimization
![](https://i.imgur.com/AJKdFgY.png)

![](https://i.imgur.com/pndAEOL.png)

### P Problems
The class of __decision problems__ that are bounded by [[Polynomial Time Complexity]].

![](https://i.imgur.com/O6SIZkD.png)

### NP Problems
The class of decision problems for which there is a __polynomially bounded nondeterministic algorithm.__ It can be _verified_ in polynomial time.

Nondeterministic algorithm: ![](https://i.imgur.com/jqsNr81.png)

![](https://i.imgur.com/Sc32KDc.png)

> [!NOTE] Why is the Knapsack problem NP?
> Verification:
> There are $2^n$ subsets of n objects: to check all subsets we would need $O(2^n)$ time.
> However, given a guess of a subset: to check this subset we would need $O(n)$ time.
> Solution:
> DP Solution is [[Pseudo-Polynomial Time Complexity]]
## NP Completeness
![](https://i.imgur.com/ikXAvIN.png)

- NP complete problems are equal to each other in difficulty
- Hardest problems in NP
- If a polynomial time solution can be found for an NP complete problem: P = NP


## Too hard...use greedy heuristics
- [[Knapsack Problem#Greedy Heuristics]]
- [[Travelling Salesman Problem#Greedy Heuristics]]