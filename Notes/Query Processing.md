# Query Processing
Given a query, we need to devise an algorithm to obtain the desired result

## Example: `Select B, D From R, S Where R.A = “c” AND S.E = 2 AND R.C=S.C`
Some possible solutions:
1. Cartesian product -> Select tuple -> Project![](https://i.imgur.com/VuY8jzL.png)
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
### One Pass Algorithms
#### Select
![](https://i.imgur.com/WgOKFsA.png)
- I/O Cost: B(R)
- Space: M >= 1
#### Duplicate Elimination
![](https://i.imgur.com/BwMqbS2.png)
- I/O Cost: B(R)
- Space: $M-1 \ge B(distinct(R))$
#### Natural Join
![](https://i.imgur.com/excURMh.png)
- I/O Cost: B(R) + B(S)
- Space: $M-1 \ge min(B(R),B(S))$
### Nested Loop Join
One argument is read only once while another is read repeatedly
#### Simple Nested Loop Join
Sacrifice some I/O time in order to save on memory:
![](https://i.imgur.com/Rt4LHH6.png)
- I/O Cost: $B(R) + B(S)\times B(R)$
- Space: $M \ge2$
#### Block based nested loop join
We can improve the I/O cost by utilizing all the buffers available:
![](https://i.imgur.com/UZwOtmQ.png)
- I/O Cost: $B(R) + B(S)\times (B(R)/(M-1))$
- Space: $M \ge2$
If M is large -> devolves into the one pass algorithm
If M is 2 -> devolves into the simples nested loop join algorithm
## Practice Problems
![](https://i.imgur.com/cMgeDuA.png)
a. One pass algorithm means that at least one of the relations must be fully loaded into the input buffer. M must be at least 100 + 1 = 101
I/O cost: 100 + 150 = 250
b. Isnt this the same as in a?
c. 
![](https://i.imgur.com/OxR8CyD.png)
We can load 999 blocks of R and 1 block of S at a time.
$Cost = (20000/999)\times5000+20000\approx120101$
![](https://i.imgur.com/9bIMrwI.png)
Assume we divide the buffer equally among the 2 relations
We load B(R) a total of $B(R)\div\frac{M}{2}$  times
For each of these set of B(R), we will need to load B(S) for $B(S)\div\frac{M}{2}$ times
We load B(S) total of $\frac{B(R)}{M/2}\times \frac{B(S)}{M/2}$
Each time we load blocks of B(S) we incur M/2 I/O cost
We load B(R) total of B(R) times
$Cost =\frac{2B(R)B(S)}{M}$. The end.