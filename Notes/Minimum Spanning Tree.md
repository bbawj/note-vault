---
title: "Minimum Spanning Tree"
date: 2022-11-08
lastmod: 2022-11-21
---
# Minimum Spanning Tree
A minimum-weight _spanning tree_ in a weighted graph

Spanning tree: connected, acyclic subgraph containing all the vertices of a graph

> [!NOTE] Minimum Spanning Tree Property
> Let T be a spanning tree of G, where $G=(V,E,W)$ is a connected weighted graph
> 
> For every edge (u,v) of G that is not in T
> - if (u,v) is added to T, it creates a cycle such that (u,v) is a maximum-weight edge on that cycle
> - then T has the MST Property

#### Spanning Trees with the MST property have the same total weight
![](https://i.imgur.com/QzcqYvy.png)
![](https://i.imgur.com/ypxVcgC.png)
![](https://i.imgur.com/TksXmkx.png)


#### A tree is a MST if and only if it has the MST property
Tree is an MST if it has the MST property:
![](https://i.imgur.com/LeYvaAI.png)
Tree with MST property is an MST:
![](https://i.imgur.com/0Qm1bh1.png)


#### Uniqueness property: if a graph has unique edge weights, there can only be 1 MST
<iframe width="560" height="315" src="https://www.youtube.com/embed/Ftkv1Ijp5Jw?start=100" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>

#### Cut Property
For any cut C of the graph, if the weight of an edge e in the cut-set of C is strictly smaller than the weights of all other edges of the cut-set of C, then this edge belongs to all MSTs of the graph.

Proof: Assume that there is an MST T that does not contain e. Adding e to T will produce a cycle, that crosses the cut once at e and crosses back at another edge e' . Deleting e' we get a spanning tree $T∖{e'}∪{e}$ of strictly smaller weight than T. This contradicts the assumption that T was a MST.

By a similar argument, if more than one edge is of minimum weight across a cut, then each such edge is contained in some minimum spanning tree.

Misinterpretation of the cut property:
![](https://i.imgur.com/YMmnopg.png)

Answer is No. For any node u in $M_2$  there can be some other path P that connects u to v in $M_2$ which passes through the cut multiple times (enters $M_1$). This path can have edge weights smaller than the path $P'$ that connects u to v through $M_2$
![](https://i.imgur.com/u35Zgg2.png)
