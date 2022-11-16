# Prim's Algorithm
<iframe width="560" height="315" src="https://www.youtube.com/embed/cplfcGZmX7I" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>
## General Idea
It is a greedy algorithm to generate a [Minimum Spanning Tree](Notes/Minimum%20Spanning%20Tree.md).
1. Select the minimum weight edge from tree vertex to fringe vertex
2. Update the fringe vertex distances with the minimum weight edge and the parent node for this min weight edge
3. Repeat until all vertices are in the tree
## Pseudocode
![](https://i.imgur.com/GI9xa3E.png)

![](https://i.imgur.com/NRqvTB2.png)

## Complexity

## Proof
Combine with proof of [](Notes/Minimum%20Spanning%20Tree.md#A%20tree%20is%20a%20MST%20if%20and%20only%20if%20it%20has%20the%20MST%20property%7CTheorem%201)
![](https://i.imgur.com/4sfUAW9.png)

## Examples
![](https://i.imgur.com/i02q3xQ.png)

![](https://i.imgur.com/C4Rltcn.png)
