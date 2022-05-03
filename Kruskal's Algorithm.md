# Kruskal's Algorithm
<iframe width="560" height="315" src="https://www.youtube.com/embed/71UQH7Pr9kU" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>

## General Idea
A greedy algorithm to generate a [[Minimum Spanning Tree]] using the [[Union Find]] data structure.

1. Sort edges in increasing order of weight (a priority queue using a [[Heaps]])
2. Add the edge to the minimum spanning tree unless it creates a cycle
	- Cycle check uses union find
## Pseudocode
![](https://i.imgur.com/k8KosSA.png)

## Complexity
Time complexity is $O(ElogE)$ mainly due to the contribution of the [[Heaps]] implementation of priority queue to obtain the least cost edge.
## Examples
![](https://i.imgur.com/6GAifnv.png)

Manually:
![](https://i.imgur.com/5kfRLym.png)
Update the id directly to the root (D) when connecting with other equivalence classes when using [[Union Find#Weighted Quick Union]]:
![](https://i.imgur.com/1pVDfMY.png)
