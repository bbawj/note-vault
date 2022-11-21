---
title: "Insertion Sort"
date: 2022-11-08
lastmod: 2022-11-21
---
# Insertion Sort
## General Idea
Incremental approach
1. Iterate through the array and maintain a sorted array _in-place_, at the start of the array.
2. For every element, loop through the sorted array backwards. Compare and swap the elements until it is in the correct spot.
3. Repeat until the entire array has been iterated through.

## Pseudocode
![](https://i.imgur.com/goWoPyc.png)

_Equal elements are not swapped in position_. This means that Insertion Sort is [](005%20Sorting%20Algorithms.md#^85ee66%20%7Cstable).
## Complexity
> [!NOTE] Best Case
> - Occurs when the array is already sorted
> 	- 1,2,3,4,5
> - 1 Key comparison is still required at each iteration
> - Total of $n-1$ comparisons

Observe that the time complexity is $O(n+I)$ where $I$ is the number of inversions. This also means that the time complexity to sort an array that is almost sorted i.e. _small number of inversions_, is linear. 

> [!NOTE] Worst Case
> - Occurs when the array is reversely sorted, $\theta(n^2)$ inversions
> 	- 5,4,3,2,1
> - Each iteration requires iterating through the entire sorted array
> - Total: $$1+2+...+(n-1)=\sum_{i=1}^{n-1}i=\frac{(n-1)n}{2} $$

![](https://i.imgur.com/ziRUDDA.png)

## Examples
![](https://i.imgur.com/RvTrICu.png)

## Overall Evaluation
![](https://i.imgur.com/DSvfcbw.png)
