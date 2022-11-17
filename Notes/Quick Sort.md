---
title: "Quick Sort"
---
# Quick Sort
## General Idea
1. Select a pivot element
2. Partition the input into 2 halves (1 that is lower than the pivot, 1 that is higher than the pivot)
	- At each partition we end up with an array that is sorted relative to the pivot (pivot is in the final position)
3. Recursively do the same for each half until we reach subarrays of 1 element.

## Pseudocode
![](https://i.imgur.com/3ezJ7J7.png)

### Partition Method
The middle element is usually chosen as the pivot:
1. Swap mid with low, placing the pivot at the start of the array
- ![](https://i.imgur.com/s9rWqhm.png)
2. If an element is smaller than the pivot: swap the position of a big element $(++lastsmall)$ with this smaller element $(i)$. __(This step results in an [](005%20Sorting%20Algorithms.md#^85ee66%7Cunstable) algorithm)__.
3. If an element is larger or equal to the pivot: increment the index $(i)$
4. Once last index is reached: Swap the pivot position $low$ with $lastsmall$, placing the pivot in the middle

![](https://i.imgur.com/F8TKAUu.png)
![](https://i.imgur.com/s4gxfuU.png)

> [!NOTE] Key equality
> Note that when a key $x$ is equal to the pivot, we do not perform any swaps or increment the lastsmall counter; __we simply treat it as it was bigger.__
> 
> This means that at the end of the partition, $x$ will be placed on the right of pivot. <u>Leaving the right subarray non-empty.</u>
> 
> If we want the left subarray to be empty, the pivot must be $\le$ all keys.
> If we want the right subarray to be empty, the pivot must be $>$ all keys
## Complexity
![](https://i.imgur.com/3libTLv.png)
![](https://i.imgur.com/5VTznYZ.png)

![](https://i.imgur.com/10uneW7.png)
This results in $O(nlogn)$ complexity

![](https://i.imgur.com/2PzRqdf.png)
![](https://i.imgur.com/FhofqQe.png)


## Examples
Forming a worst case array:
![](https://i.imgur.com/TBa1xzm.png)

## Overall Evaluation
![](https://i.imgur.com/d4EdqEd.png)
