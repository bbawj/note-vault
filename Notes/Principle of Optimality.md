---
title: "Principle of Optimality"
date: 2022-11-08
lastmod: 2022-11-21
---
# Principle of Optimality
Principle of Optimality: An optimal policy has the property that whatever the initial state and initial decision are, the remaining decisions must constitute an optimal policy with regard to the state resulting from the first decision.

This is the basis for [Markov Decision Process](Notes/Markov%20Decision%20Process.md).

## Application to Dynamic Programming
> [!NOTE] Paraphrasing...
> A problem is said to satisfy the principle of optimality if the subsolutions of an optimal solution of the problem are themselves optimal solutions for their subproblems.

__The [Shortest Path Problem](Notes/Shortest%20Path%20Problem.md) satisfies the principle:__

If $a,x1,x2,...,xn,b$ is a shortest path from node a to node b in a graph, then the portion of $xi \to xj$ on that path is a shortest path from $xi \to x$j. [](Notes/Shortest%20Path%20Problem.md#^c4528f%7CCan%20be%20proven%20by%20contradiction.) 

__The longest path problem does not satisfy:__

Take for example the undirected graph G of nodes a, b, c, d, and e, and edges $(a,b) (b,c) (c,d) (d,e) and (e,a)$. That is, G is a ring. The longest (noncyclic) path from a to d to a,b,c,d. The sub-path from b to c on that path is simply the edge b,c. But that is not the longest path from b to c. Rather, $b,a,e,d,c$ is the longest path. Thus, the subpath on a longest path is not necessarily a longest path.
