# B-tree
A self balancing tree data structure.

Consider a B-Tree of order n (here we use example of [[Indexes#B Tree Index|B+Tree]]):
- Each orange box is a key
- Each blue line is a pointer to subtree
![](https://i.imgur.com/1jSNPRN.png)

![](https://i.imgur.com/gYfysVX.png)
This also means that each internal node has  at least $\lfloor{n/2}\rfloor$ +1 children 