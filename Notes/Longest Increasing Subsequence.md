# Longest Increasing Subsequence
![](https://i.imgur.com/ZQvHrc3.png)
### Example
![](https://i.imgur.com/POgmgUz.png)

## Problem Formulation
Satisfaction of the Principle of Optimality:
The solution to a subsequence ending at index j can be found using the solutions from subsequence ending at index 0 to j-1.

Base Case (longest subsequence ending at index 0):
$$LIS(0)=1$$
Case 1: the character at index j is smaller than the character at index k (for some k)
$$LIS(j)=1$$
Case 2: the character at index j is bigger than the character at index k (for some k)
$$LIS(j)=LIS(k)+1 $$
Solution can be found by taking the maximum:
$$LIS(j)=\max_{k=0\to j-1} \{LIS(K)+1\}$$
## Strategy
![](https://i.imgur.com/pbLc1Jj.png)

## Pseudocode
![](https://i.imgur.com/KrqNqt1.png)
