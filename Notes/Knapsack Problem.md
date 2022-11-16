---
title: "Knapsack Problem"
---
# Knapsack Problem
Given n items where the $i^{th}$ item has a weight $w_i$ and value $v_i$

Find the largest total value of items that fits in a knapsack of capacity $C$.
## Problem Formulation
Let $P(C, j)$ be the max profit by selecting a subset of j objects with knapsack capacity C.

Base case: Capacity or number of items is 0
$$P(C,0)=P(0,j)=0 $$
Otherwise, the solution to a knapsack of capacity C can be found using the solutions to the subproblems of capacity $C-w_i$ for items $i=1 \to n$. For each item, we can choose either include or not include it in the knapsack:
$$P(C,j)=max(P(C,j-i), P(C-w_j,j-1)+v_j) $$
## Strategy
![](https://i.imgur.com/p9wp7ok.png)

Profit table:
![](https://i.imgur.com/HCYL5ee.png)

## Pseudocode
![](https://i.imgur.com/4ll6HfB.png)

## Greedy Heuristics
Maximizing by the profit per weight:
![](https://i.imgur.com/v1spIPp.png)

![](https://i.imgur.com/JIPUvt6.png)



## Exercises
![](https://i.imgur.com/mCLwpf1.png)

|     | 0   | 1   | 2   | 3   | 4   |
| --- | --- | --- | --- | --- | --- |
| 0   | 0   | 0   | 0   | 0   | 0   |
| 1   | 0   | 0   | 0   | 0   | 0   |
| 2   | 0   | 0   | 0   | 0   | 0   |
| 3   | 0   | 0   | 0   | 0   | 50  |
| 4   | 0   | 0   | 40  | 40  | 50  |
| 5   | 0   | 10  | 40  | 40  | 50  |
| 6   | 0   | 10  | 40  | 40  | 50  |
| 7   | 0   | 10  | 40  | 40  | 90  |
| 8   | 0   | 10  | 40  | 40  | 90  |
| 9   | 0   | 10  | 50  | 50  | 90  |
| 10  | 0   | 10  | 50  | 70  | 90  | 
