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
