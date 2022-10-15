# Query Optimisation
## Algebraic Laws for Improving Query Plans
### Laws Involving Join
![](https://i.imgur.com/oDq81HA.png)
#### Converting Selection and Product to Joins
![](https://i.imgur.com/sGg1CNA.png)
### Laws Involving Selection
![](https://i.imgur.com/I95RFp3.png)
If R is not a set (it is a bag which can contain duplicates) then the union operation will not eliminate duplicates correctly.
#### Pushing selection
![](https://i.imgur.com/3pmPzmA.png)
![](https://i.imgur.com/q584Z29.png)
### Projection Laws
Pushing projection is less useful than pushing selection. This is because projection keeps the number of tuples the same and only reduces the length of the tuples.
> [! Basic idea: can introduce a projection anywhere in an expression tree, as long as  it eliminates only attributes that are neither used by an operator above  nor are in the result of the entire expression. ]

![](https://i.imgur.com/kMU6jVT.png)
## Logical Query Plan
Use of operator trees to represent the execution plan.
![](https://i.imgur.com/pg8oQVv.png)
### Optimising the Operator Tree
We can use the algebraic laws to devise a more optimal logical query plan.
![](https://i.imgur.com/2VYfLtr.png)
![](https://i.imgur.com/KlplmiL.png)
![](https://i.imgur.com/jjASNOH.png)
![](https://i.imgur.com/t4dBrQc.png)
## Physical Query Plan
A physical query plan are the actions for which to execute the logical query plan.
It includes:
- Order and grouping of operations
- [[Notes/Query Processing|Algorithms for each operator]]
- [[Notes/Query Execution|Argument passing from operator to operator: pipelining vs materialisation]]
We need a way to make choices for each of these components.
### Cost estimation
The cost of a plan is the sum of the cost of each operator in the tree. However, to know the cost of an operator often requires the input sizes to be known. This is often not available for intermediary operators which are executed after other operators such as `SELECT` are done.
Assumptions:
1. Uniform distribution of values in domain
2. Independent distribution of values in different columns
3. Independence of predicates for select and join
![](https://i.imgur.com/69FO4mE.png)
#### Estimating Selection
![](https://i.imgur.com/kLBr1UB.png)
3 is used as an intuition for how a range operation would usually return less than half of all the tuples.
![](https://i.imgur.com/vQPnDLO.png)
For negation, we can estimate all tuples in the relation to satisfy the condition. Alternatively, notice that the number of distinct tuples that satisfy the relation is equal to $V(R,A)-1$
![](https://i.imgur.com/kqmLVen.png)
Notice that the equation relates to converting an OR relation into an equivalent AND notation. $A|B=!(!A \cap !B)$  
![](https://i.imgur.com/oVNvTd7.png)
#### Estimating Joins
Assumptions:
![](https://i.imgur.com/c5Vnm56.png)
- Containment of value: satisfied when Y is a key in S and the corresponding foreign key in R. Approximately true due to probability since if Y appears in S, it is likely to appear in R as well since S is large.
- Preservation: violated when there are tuples in R which join with no tuples in S.
![](https://i.imgur.com/y5mkPC7.png)
![](https://i.imgur.com/EAwXdkf.png)
![](https://i.imgur.com/oPDMzqE.png)
#### Using Statistics
The assumption of uniform distribution is not accurate since real data is not uniformly distributed.
We can maintain a histogram for each relation to help us improve the estimation:
![](https://i.imgur.com/PcMR1ga.png)
Statistics from each bucket can be used to determine the number of tuples in a range:
![](https://i.imgur.com/HPXoFGg.png)
Example:
![](https://i.imgur.com/Tou8AKE.png)
Example for join:
![](https://i.imgur.com/1iQ4kdj.png)
Sampling can be used to increase performance:
![](https://i.imgur.com/lE0yNfb.png)
## Practice Problems
![](https://i.imgur.com/dSCheOj.png)
$$
\begin{aligned}
T(R\Join S)&=T(R)\times T(S)/max(V(R,a),V(S,a))
\\T(R_1\Join R_2\Join R_3) &=T(T(R_1\Join R_2)\Join R_3)
\\&=(1000\times1500/1100)\times750/100
\\&=10227.27
\end{aligned}
$$
![](https://i.imgur.com/n4C5J07.png)
$$
\begin{align}
\text{Size of Y}=20\times128=2560B
\\B(Y) =5
\\ \text{Size of X}=60\times64=3840B
\\B(X) =7.5
\\\text{Block access for Sort merge join with 2PMMS}=5\times(5+8)
\\=65
\\\text{Block access for selection}=0.5\times(5+8)=7
\\\text{Total Disk Access}=7+65=72
\end{align}
$$
![](https://i.imgur.com/J4jwXGW.png)
a.
1. Select movie with year > 1990 and rating = 10
2. Join Movie and Studio
b.
$$\begin{align}
P(Year>1990)=1/3,P(Rating=10)=1/10
\\\text{Size after Select}=0.1\times
\end{align}
$$


