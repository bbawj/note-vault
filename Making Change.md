# Making Change
![](https://i.imgur.com/9zfeU5i.png)

## Problem Formulation
> [!NOTE] Note that
> The minimum number of coins to form value x, is 1 + the minimum of the solutions to value $x-d_1, x-d_2....x-d_k$

We can form the following recurrence equations:
Base case (when n is 0):
$$change(n)=0$$
Finding the minimum of subproblems:
$$change(n)=\min_{i=1...m,d_i\le n}(1+change(n-d_i))$$
## Strategy
![](https://i.imgur.com/6HMagN9.png)

## Pseudocode
![](https://i.imgur.com/zzyfrLX.png)
