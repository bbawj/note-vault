---
title: "Union Find"
date: 2022-11-08
lastmod: 2022-11-21
---
# Union Find
A data structure to represent dynamic equivalence relations
![](https://i.imgur.com/fOnL8Vo.png)

## Operations
![](https://i.imgur.com/jhhDWZD.png)

These operations make it an efficient data structure to verify if a cycle exists in some __undirected__ graph.
- Union find is unable to detect cycles in directed graphs: union operation cannot distinguish between the subcomponent that makes the edge from a -> b and that which makes the edge b->a

![](https://i.imgur.com/Tv7KgtB.png)

## Implementations
### Quick Find
![](https://i.imgur.com/Katri54.png)

![](https://i.imgur.com/eRsdZvR.png)

#### Pseudocode
![](https://i.imgur.com/c5Kl8s7.png)

#### Complexity
![](https://i.imgur.com/iGofEbN.png)

### Quick Union
Unions are too expensive using quick find, as we have to change the ids of all elements in the combining equivalence class for each union operation.

Improvement: only store the parent of the node
![](https://i.imgur.com/5Mf1OyA.png)

![](https://i.imgur.com/sB3ZACP.png)

#### Pseudocode
![](https://i.imgur.com/NHSHVgc.png)

#### Complexity
![](https://i.imgur.com/kIEBDNq.png)

### Weighted Quick Union
Quick Union creates possible tall trees as the union operation does not necessarily occur between root nodes.

Improvement: avoid tall trees by linking smaller tree root to larger tree root
![](https://i.imgur.com/5BJu8tg.png)

![](https://i.imgur.com/311sffE.png)

#### Pseudocode
#### Complexity
![](https://i.imgur.com/ea12VYk.png)

Hence, besides initialization $O(n)$, all other operations take $O(logn)$
### WQU with Path Compression
Improvement: further reduce the effective path to reach the root node: for some node p which we are computing the root of, we can update id[] of nodes on the path from p to root.

![](https://i.imgur.com/AUlmrC9.png)
