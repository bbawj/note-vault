# Query Processing
Given a query, we need to devise an algorithm to obtain the desired result
## Example
`Select B, D From R, S Where R.A = “c” AND S.E = 2 AND R.C=S.C`
Some possible solutions:
1. Cartesian product -> Select tuple -> Project ![](https://i.imgur.com/VuY8jzL.png)
2. Select tuple -> Natural join ->Project ![](https://i.imgur.com/pIvJyQj.png)
3. Index to select R tuples -> Use S.C index to select found R.C values -> Remove S.E != 2 -> Join matching![](https://i.imgur.com/otMOPCy.png)
## Operator Analysis
![](https://i.imgur.com/mzWlK2B.png)
Notations:
- R: a relation/table
- T(R): number of tuples in R
- B(R): number of blocks needed to hold tuples of R
- V(R, a): number of distinct values of attribute a in R
Assumptions for analysis:
1. Main memory is organized as input buffers (for holding data for computations) and output buffers (for storing results) each being the size of 1 block. Let M represent the number of input buffers available.
2. Relation R is clustered if its tuples are packed into as few blocks as possible. *If T(R) = 1,000 and each block can hold 1000 tuples, B(R) = 100*
	- We assume R is clustered. If not clustered, it may take T(R) disk I/Os rather than B(R) to read all the tuples
3. Ignore the final I/O cost for writing result in output buffer back to disk. Irrelevant to our calculations.
## Notable Algorithms:
- [One Pass Algorithms](Notes/One%20Pass%20Algorithms.md)
- [Two Pass Algorithms](Notes/Two%20Pass%20Algorithms.md)
- [Index Based Algorithms](Notes/Index%20Based%20Algorithms.md)
## Comparisons
![](https://i.imgur.com/YlB9izb.png)
