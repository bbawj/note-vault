---
title: "Greedy Best First Search"
date: 2022-11-08
lastmod: 2022-11-21
---
# Greedy Search
Expands the node that appears to be closest to the goal based on the evaluation function h(n) i.e. expands the lowest h(n) values first.

Completeness: No
Optimality: No
Time Complexity: $O(b^m)$
Space Complexity: $O(b^m)$

### Graph Traversal
![](https://i.imgur.com/syk9okN.png)

_Assuming ties are handled in alphabetical order_

Expansion Order:
A > B > C > G 
Final Path:
A > B > C > G
