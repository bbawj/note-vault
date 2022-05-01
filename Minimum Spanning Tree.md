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


#### If a graph has unique edge weights, there can only be 1 MST
<iframe width="560" height="315" src="https://www.youtube.com/embed/Ftkv1Ijp5Jw?start=100" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>
