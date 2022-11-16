# Uniform Cost Search
Take the full path cost to the node as g(n)

Terminate when a goal node is found.

Optimality: True for graph with nonnegative weights

Similar implementation to [Dijkstra's Algorithm](Notes/Dijkstra's%20Algorithm.md) but without computing shortest path for all nodes.
### Graph Traversal
![](https://i.imgur.com/syk9okN.png)

_Assuming ties are handled in alphabetical order_

Expansion Order:
A > B > D > E > F > G
Final Path:
A > B > D > E > F > G