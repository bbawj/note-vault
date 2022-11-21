---
title: "B+ Tree Index"
date: 2022-11-08
lastmod: 2022-11-21
---
# B+ Tree Index
Idea: build a multi-layer index in the structure of a [B-tree](Notes/B-tree.md)
> [!Properties]
> 1. Each tree node is stored within a *block*
> 2. Each node stores at most n+1 pointers and n keys
> 3. Each level is an index
> 	- sorted within each node
> 	- sorted across nodes at the same level

> [!Leaf Node Properties]
> Consider a leaf node storing k+1 pointers (k <= n) and k keys
> 1. First k pointers are to records and last one is to the next leaf node
> 2. Each key is equal to the key that its corresponding pointer is pointing to in the record

> [!Internal Node Properties]
> Consider an internal node storing k+1 pointers (k <= n) and k keys
> 1. The ith key is the lower bound of the range of the i+1 pointer. The 2nd pointer points to a subtree that has the first key as the first element:![](https://i.imgur.com/yyB4CcW.png)
> 
### Validity
![](https://i.imgur.com/vWLJeF6.png)

![](https://i.imgur.com/BqfRjog.png)
### Searching
1. If search key is greater than ith key, follow i+1 pointer, else follow ith pointer
![](https://i.imgur.com/3DufQar.png)

![](https://i.imgur.com/hSGZIQX.png)
### Insertion
1. Find which node to insert record
2. If node is not full, insert the record (maintain sorted order)
3. Else
	1. Split the node and distribute the keys
	2. If no parent, create root node
	3. Else, insert into parent recursively until no splits needed
![](https://i.imgur.com/1WdRWqt.png)

![](https://i.imgur.com/iBO9eDF.png)

![](https://i.imgur.com/EBPXqTI.png)
### Deletion
Case 1: key can be deleted while maintaining constraints
Case 2: key can be borrowed from sibling nodes, e.g. take 16 from left:
![](https://i.imgur.com/PFt0kHV.png)
Case 3: cannot borrow from siblings
1. Merge 2 nodes and delete 1 of them
2. Delete key from parent (one child is removed, may need to remove key from parent)
3. Recursively apply delete on this parent if it is not full enough
![](https://i.imgur.com/qYeNPlC.png)
![](https://i.imgur.com/Nv07qVW.png)
### Construction
One way to do so is through a series of insert operations
No sequential storage of leaf nodes but is more suitable for dynamic data.
#### Bulk Loading
Leaves will be stored sequentially, will work for static data (all data is known before hand)
1. Sort all data entries based on search key
2. Start by creating all leaf nodes by packing the keys
3. Insert internal nodes bottom up
<iframe width="560" height="315" src="https://www.youtube.com/embed/HJgXVxsO5YU?start=160" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>
## Practice Problems
![](https://i.imgur.com/WLEaTv1.png)
a.
1. 5
2. 5
3. 4
4. 4
b.
1. Min key + 1 = 3
2. Min key + 1 = 3
3. Original keys: n. After split at least  $\lfloor(n/2)\rfloor=2$
4. $\lfloor(n+1)/2\rfloor=2$
![](https://i.imgur.com/6QXYSgP.png)

![Drawing 2022-09-05 10.35.50.excalidraw](Excalidraw/Drawing%202022-09-05%2010.35.50.excalidraw.md)
![](https://i.imgur.com/Bu5oJA2.png)
Height of the B+tree will be $log_{150}1000000=2.75$
Need at least 3 levels in the btree
At most will take 3 I/O + 1 I/O to access data block
![](https://i.imgur.com/yhicZvc.png)
- Each internal node can index 151 children
- Last level indexes 150 records
![](https://i.imgur.com/06CoAso.png)
a.
i. Data level: 1,000,000 records -> 100,000 blocks
Leaf level will have 1,000,000 pointers = $1000000/70\approx14286$ blocks
2nd level will have $14286/70\approx205$ blocks
3rd level will have $205/70\approx3$ blocks
Root will have 1 block 
Total blocks: $100000+14286+205+3=114494$
Height of the B-tree will at least be: $log_{70}1000000=3.25\approx4$
Number of I/O = 4+1 = 5
b. Same as a. Since dense index, order of the data record does not matter
c. What if a) but sparse index?
Leaf level will have 100,000 pointers to each data record block: $100000/70=1429$ blocks
2nd level: $1429/70\approx21$
Root: 1
Total blocks: $100,000+1429+21+1=101451$
![](https://i.imgur.com/OSwkx7i.png)
![B+ Tree Index 2022-09-13 21.06.04.excalidraw](Pics/B+%20Tree%20Index%202022-09-13%2021.06.04.excalidraw.md)
