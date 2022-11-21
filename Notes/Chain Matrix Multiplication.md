---
title: "Chain Matrix Multiplication"
date: 2022-11-08
lastmod: 2022-11-21
---
# Chain Matrix Multiplication
![](https://i.imgur.com/LZShYsW.png)

![](https://i.imgur.com/WfUbMCv.png)

## Problem Formulation
Satisfaction of the Principle of Optimality:
Let $A_i$ represent the $i^{th}$ matrix with dimensions $(d_{i-1}\times d_i)$. 
The optimal way to multiply matrices $A_i \ to\ A_j$  can be broken down into the optimal way to multiply the matrices $A_i \ to\ A_k + A_{k+1} \ to\ A_j$ (for some k) + the cost to multiply the final 2 matrices = $d_i \times d_k \times d_j$   

Define $OptCost(i,j)$ to be the optimal cost of multiplying matrices with dimensions $d_i, d_{i+1},...d_j$

Base case:
$$OptCost(i,j) = 0 \qquad j-i=1$$
Recursive equation can be formed as follows: loop through possible k to find min
$$OptCost(i,j) = \min_{i+1\le k\le j-1}(OptCost(i,k)+OptCost(k,j)+d_i\times d_k\times d_j) $$
## Strategy
![](https://i.imgur.com/6baCPCO.png)

Store the solutions to subproblems in _cost 2d array_.
$Cost[i][j]$ represents the optimal cost of multiplying matrices $A_{i+1} \to A_j$
![](https://i.imgur.com/L8ICbot.png)

Store the optimal values of k (index to split the matrix multiplication) in _last 2d array_.![](https://i.imgur.com/0rm8iwb.png)
## Pseudocode
![](https://i.imgur.com/VJPaKmo.png)

``` java
// Matrix A[i] has dimension dims[i-1] x dims[i] for i = 1..n
MatrixChainOrder(int dims[])
{
    // length[dims] = n + 1
    n = dims.length - 1;
    // m[i,j] = Minimum number of scalar multiplications (i.e., cost)
    // needed to compute the matrix A[i]A[i+1]...A[j] = A[i..j]
    // The cost is zero when multiplying one matrix
    for (i = 1; i <= n; i++)
        m[i, i] = 0;

    for (len = 2; len <= n; len++) { // Subsequence lengths
        for (i = 1; i <= n - len + 1; i++) {
            j = i + len - 1;
            m[i, j] = MAXINT;
            for (k = i; k <= j - 1; k++) {
                cost = m[i, k] + m[k+1, j] + dims[i-1]*dims[k]*dims[j];
                if (cost < m[i, j]) {
                    m[i, j] = cost;
                    s[i, j] = k; // Index of the subsequence split that achieved minimal cost
                }
            }
        }
    }
}
```
## Exercises
Suppose the dimensions of the matrices A, B, C, and D are 20x2, 2x15, 15x40, and 40x4, respectively, and we want to know how best to compute AxBxCxD. Show the arrays cost, last, and multOrder computed by Algorithms matrixOrder() in the lecture notes.

|     | 0   | 1   | 2   | 3                                                                                            | 4                                                                                                                                                                                                                 |
| --- | --- | --- | --- | -------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 0   | -   | 0   | 600 | min{$(0,1)+(1,3)+d_0*d_1*d_3$ <br>                         $(0,2)+(2,3)+d_0*d_2*d_3$} = 2800 | min{$(0,1)+(1,4)+d_0*d_1*d_4$ <br>                                                        $(0,2)+(2,4)+d_0*d_2*d_4$ <br>                                                        $(0,3)+(3,4)+d_0*d_3*d_4$} = 1680 |
| 1   |     |     | 0   | 1200                                                                                         | min{$(1,2)+(2,4)+d_1*d_2*d_4$ <br>                                                                                                                         $(1,3)+(3,4)+d_1*d_3*d_4$} = 1520                      |
| 2   |     |     |     | 0                                                                                            | 2400                                                                                                                                                                                                              |
| 3   |     |     |     |                                                                                              | 0                                                                                                                                                                                                                 |
| 4   |     |     |     |                                                                                              |                                                                                                                                                                                                                   |
Last Table

|     | 0   | 1   | 2   | 3   | 4   |
| --- | --- | --- | --- | --- | --- |
| 0   | -   | -   | 1   | 1   | 1   |
| 1   |     |     |     | 2   | 3   |
| 2   |     |     |     |     | 3   |
| 3   |     |     |     |     |     |
| 4   |     |     |     |     |     |
Final order: $A_1*((A_2*A_3)*A_4)$

![](https://i.imgur.com/x5CyjBB.png)

![](https://i.imgur.com/AEoBKW9.png)

## Greedy heuristic
![](https://i.imgur.com/VOcanZm.png)
