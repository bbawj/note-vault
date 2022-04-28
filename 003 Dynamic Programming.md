# 003 Dynamic Programming
#moc 
_Dynamic Programming is a tool to solve problems which satisfy the [[Principle of Optimality]]._
## Well Known Problems
- [[Fibonacci Sequence]]
- [[Longest Common Subsequence]]
- [[Chain Matrix Multiplication]]
- [[Knapsack Problem]]
- [[Making Change]]
## Strategies
Both strategies will achieve the same time complexity but bottom up is usually more CPU time efficient due to the simplicity of the code
### Top Down Approach
1. Formulate the problem in terms of recursive smaller subproblems.
2. Use a dictionary to store the solutions to subproblems
3. Turn the formulation into a recursive function
	1. Before any recursive call, check the store to see if a solution has been previously computed
	2. Store the solution before returning

Example with Fib DP:
![](https://i.imgur.com/zKdEyaU.png)

### Bottom Up Approach
1. Formulate the problem in terms of recursive smaller subproblems.
2. Draw the subproblem graph to find dependencies
3. Use a dictionary to store the solutions to subproblems
4. Turn the formulation into a recursive function
	1. Compute the solutions to subproblems first
	2. Use the solutions to compute the solution for P and store it

![](https://i.imgur.com/I2520pv.png)

Example with Fib DP:
![](https://i.imgur.com/0OgEhHu.png)
