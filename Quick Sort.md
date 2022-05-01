# Quick Sort
## General Idea
1. Select a pivot element
2. Partition the input into 2 halves (1 that is lower than the pivot, 1 that is higher than the pivot)
	- At each partition we end up with an array that is sorted relative to the pivot
3. Recursively do the same for each half until we reach subarrays of 1 element.

## Pseudocode
![](https://i.imgur.com/3ezJ7J7.png)

### Partition Method
The middle element is usually chosen as the pivot:
1. If an element is smaller than the pivot: swap the position of a big element $(++lastsmall)$ with this smaller element $(i)$. __(This step results in an [[005 Sorting Algorithms#^85ee66|unstable]] algorithm)__.
2. If an element is larger or equal to the pivot: increment the index $(i)$
3. Swap the pivot position $low$ with $lastsmall$, placing the pivot in the middle

![](https://i.imgur.com/F8TKAUu.png)
![](https://i.imgur.com/s4gxfuU.png)

## Complexity
![](https://i.imgur.com/3libTLv.png)
![](https://i.imgur.com/5VTznYZ.png)

![](https://i.imgur.com/10uneW7.png)

## Examples