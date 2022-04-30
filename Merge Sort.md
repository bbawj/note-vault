# Merge Sort
## General Idea
Divide and Conquer
1. Divide the problem recursively into smaller (n/2) sub problems
2. When only 1 element remains, this subproblem is considered sorted
3. Combine sorted subproblems together (backtracking) using a _merge function_.

### Merge function
![](https://i.imgur.com/9277bZh.png)

![](https://i.imgur.com/FjYLTRc.png)

![](https://i.imgur.com/uLjOcjg.png)


## Pseudocode
![](https://i.imgur.com/ugtAtdh.png)

## Complexity
Complexity at each recursive call is due to the application of the merge function

> [!NOTE] Worst Case
> - At least 1 element is moved to the new merged list after each comparison
> - The final comparison will result in 2 elements moving to the merged list
> - Hence, key comparisons needed to merge n elements is at most n-1

> [!NOTE] Best Case
> - When both subarrays are already sorted relative to each other, each comparison will move one element from the smaller array into the merged list until it is empty
> - The bigger array will be appended to the end without further comparisons
> - If both arrays are equal size (n/2), there will be at best $\frac{n}{2}$ key comparisons


![](https://i.imgur.com/NQwgyjU.png)	
## Examples