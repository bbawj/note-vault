---
title:"Index Based Algorithms"
---
# Index Based Algorithms
Having an index on 1 or more attributes of a relation makes some algorithms more feasible.
>[! ] $V(R,a)$
>This represents the number of distinct values of an attribute $a$ in a relation _R_
>
## Index Based Selection
For a given selection operation $\sigma_{a=v}(R)$ (_meaning select all tuples in R where attribute a = v_) we can use an index on attribute a to gain cost savings.
### Clustered Index
Since each tuple with the same attribute value are packed in as little blocks as possible, a clustered index will have savings averaging: 
$$IO =B(R)/V(R,a)$$
The actual value can be higher due to:
1. All tuples with $a=v$ might be spread across more than 1 block
### Non-clustered Index
We can assume that each tuple will be on a different block, a non-clustered index will have savings averaging: 
$$IO =T(R)/V(R,a)$$
![](https://i.imgur.com/O2Kz3y2.png)
## Index Based Joining
Assume an operation to join S and R over an attribute $C$. 
If only S has an index on C, algorithm:
1. Iterate over all the blocks of R
2. For each tuple $t$ in each block $b$ with an attribute value $t.C$ use the index to find the matching attribute value in S
3. Join and output
### Cost
Step 1: incur a cost of B(R) as we need to read all blocks of R
Step 2: each tuple of R requires a read of the index
- Clustered index: $T(R)\times B(S) / V(S,C)$
- Non-Clustered index: $T(R)\times T(S) / V(S,C)$
Total cost: $B(R) + \text{index cost}$
![](https://i.imgur.com/aNMbZzE.png)
### Sorted Index Join - Zig Zag Join
With a sorted index on both relations, we can just perform the final step of [](Notes/Two%20Pass%20Algorithms.md#Sort%20Based%20Algorithms%7Csort-based%20joining). 
Index allows us to ignore retrieving data blocks where there are no matching keys.
#### Cost
From example 15.12, the number of disk I/O's if R and S both have sorted indexes, the total cost would simply be that cost to read all blocks of R and S: 1500. Consider that a large fraction of R or S cannot match tuples of the other relation, then the total cost will be considerably less than 1500.
## Practice Problems
![Pasted image 20221010171512](Pics/Pasted%20image%2020221010171512.png)
Cost of access the data blocks:
There are $\frac{k}{10}$ distinct values that we must access.
There are a total of $B(R)/k=1000/k$ blocks having distinct values
Total block access = $\frac{1000}{k}\times\frac{k}{10}=1000$
