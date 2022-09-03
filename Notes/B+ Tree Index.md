# B+ Tree Index
Idea: build a multi-layer index in the structure of a [[B-tree]]
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
<iframe width="560" height="315" src="https://www.youtube.com/embed/lTJy2MXI4lY?start=438" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>