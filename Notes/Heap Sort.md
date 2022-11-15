# Heap Sort
<iframe width="560" height="315" src="https://www.youtube.com/embed/2DmK_H7IdTo" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>

## General Idea
Data structure based sorting algorithm using [Heaps](Notes/Heaps.md).

Makes use of the Partial Order Tree property:
A tree is a maximising partial order tree if and only if each node has a key value greater than or equal to each of its child nodes.

1. Create a maximizing heap
2. Remove the root node, giving us the largest value. Fill an array from the back with this max value
3. Fix the heap structure
4. Repeat until we obtain a sorted array
## Pseudocode
![](https://i.imgur.com/admXX0H.png)

Remove the max element and retain the heap structure using [](Notes/Heaps.md#Fix%20Heap%20maximising)
![](https://i.imgur.com/CUBKrov.png)

## Complexity
Summary of complexities for Heapsort and corresponding heap structure methods:
![](https://i.imgur.com/594IBYA.png)

Heapsort has a best, worst and average case time complexity of $O(nlogn)$
## Examples
![](https://i.imgur.com/Z4vO1rs.png)
