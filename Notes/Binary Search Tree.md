---
title: "Binary Search Tree"
date: 2022-11-08
lastmod: 2022-11-21
---
# Binary Search Trees
_BST property: Let x be a node in a binary search tree. If y is a node in the left subtree of x, then y.key $\le$ x.key. If y is a node in the right subtree of x, then y.key $\ge$ x.key._

![Pasted image 20220712154949](Pics/Pasted%20image%2020220712154949.png)

Inorder tree traversal of the BST will produce a sorted list.

## Operations
### Searching
We are able to search for a specific key in O(h) time where h is the height of the tree. The BST property allows us to perform binary search.

### Min and Max
The smallest node is rooted at the left most part of the the tree and is symmetric for the largest node.
```
while x.left != null
	x = x.left
return x
```
### Successors
We are able to find the successor of a node without any key comparisons:
1. If there is a right subtree to the node x, the successor is the leftmost element in the right subtree
2. Else, the successor is the parent of the first element which is a left child when traversing upwards through the tree.

```
if x.right != null
	return Tree-Min(x.right)
y = x.p
while y != null and x == y.right
	x = y
	y = y.p
return y
```
### Insert
The procedure maintains the trailing pointer y as the parent of x. After initialization, the while loop in lines 2-6 causes these two pointers to move down the tree, going left or right depending on the key comparison, until x becomes NIL.  We need the trailing pointer y, because by the time we find the NIL where z belongs, the search has proceeded one step beyond the node that needs to be changed. Lines 7–10 set the pointers that cause z to be inserted.
```
x = root
while x
	y = x
	if z.key < x.key
		x = x.left
	else x = x.right
z.p = y
if y == null
	root = y
else if z.key < y.key
	y.left = z
else y.right = z
```

