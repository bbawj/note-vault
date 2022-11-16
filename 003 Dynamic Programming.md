# 003 Dynamic Programming
#moc 
_Dynamic Programming is a tool to solve problems which satisfy the [Principle of Optimality](Notes/Principle%20of%20Optimality.md)._
## Well Known Problems
- [Fibonacci Sequence](Notes/Fibonacci%20Sequence.md)
- [Longest Common Subsequence](Notes/Longest%20Common%20Subsequence.md)
	- [Longest Increasing Subsequence](Notes/Longest%20Increasing%20Subsequence.md)
	- [Alignment Problem](Notes/Alignment%20Problem.md)
- [Chain Matrix Multiplication](Notes/Chain%20Matrix%20Multiplication.md)
- [Knapsack Problem](Notes/Knapsack%20Problem.md)
	- [Making Change](Notes/Making%20Change.md)
- [Travelling Salesman Problem](Notes/Travelling%20Salesman%20Problem.md)
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

## Exercises
#### Binomial Coefficients 
![](https://i.imgur.com/FiqSnSZ.png)
b. ![](https://i.imgur.com/b6X5HDI.png)
c. A top down approach:![](https://i.imgur.com/J1XxTUV.png)
d. A bottom up approach ![](https://i.imgur.com/4zzRPuZ.png)

## References
- https://www2.seas.gwu.edu/~ayoussef/cs6212/dynamicprog.html