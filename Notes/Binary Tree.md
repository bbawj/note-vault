# Binary Trees
A tree structure in which each node has at most 2 children.

## Properties of binary trees
Full binary trees (all nodes have 0 or 2 children)
- The number of nodes n in a full binary tree is at least $2h+1$ and at most $2^{h+1}-1$, where h is the height of the tree. A tree consisting of only a root node has a height of 0.
- The number of leaf nodes l in a perfect binary tree, is $\frac{(n+1)}{2}$ because the number of non-leaf (a.k.a. internal) nodes $\sum _{k=0}^{\log _{2}(l)-1}2^{k}=2^{\log _{2}(l)}-1=l-1$.
- This means that a full binary tree with l leaves has $2l-1$ nodes.

Complete binary trees (like those use in [[Heaps]])
- A complete binary tree  has $\lfloor n/2 \rfloor$ internal nodes
