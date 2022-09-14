# B-tree
A self balancing tree data structure.

Consider a B-Tree of order n (here we use example of [[Notes/Conventional Indexes#B Tree Index|B+Tree]]):
- Each orange box is a key
- Each blue line is a pointer to subtree
![](https://i.imgur.com/1jSNPRN.png)

![](https://i.imgur.com/gYfysVX.png)
This also means that each internal node has  at least $\lfloor{n/2}\rfloor$ +1 children 
## Practice Problems
![](https://i.imgur.com/2am4T1y.png)
a.
1. Interior node min keys: $\lfloor(n/2)\rfloor=5$
	Min pointers: $min\ key + 1 = 5+1=6$
2. Leaf node min key: $\lfloor(n+1/2)\rfloor=5$
	Min pointers: $min\ key + 1 = 5+1=6$
b.
1. Interior node min key: 5
	Min pointers: 5+1 =6
2. Leaf node min key: 6
	Min pointers: $min\ key + 1 = 6+1=7$