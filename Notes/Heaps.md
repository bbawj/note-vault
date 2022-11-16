---
title:"Heaps"
---
# Heaps
A specialized tree based data structure.

Efficient data structure to implement a priority queue.
## Implementations
Binary Heap built with a [Binary Tree](Notes/Binary%20Tree.md):
![](https://i.imgur.com/TQ7v9f2.png)

![](https://i.imgur.com/YlmhOMz.png)

## Operations
### Fix Heap (maximising)
Method to obtain retain the heap structure after the root is removed. Assumes both left and right subtrees are already maximising heaps.

```java
fixHeap(H,k){
	if (H is a leaf)
		insert k in root of H;
	else {
		compare left and right child;
		largerSubHeap = the larger child of H;
		//key is the largest element
		if (k >= key of root(largerSubHeap))
			insert k in root of H;
		//key is not the largest, move the largest element up
		else{
			insert root(largerSubHeap) in root of H;
			//recursively find the spot where key is the largest
			fixHeap(largerSubHeap, k)
		}
	}
}
```

![](https://i.imgur.com/Iufls5t.png)

#### Example
If there are 2 child nodes, this operation will take 2 comparisons:
![](https://i.imgur.com/Oxe7sU2.png)


### Heapify
Method to obtain the maximising heap property from an arbitrary tree.
![](https://i.imgur.com/PtahWsM.png)

Fix the heap from the bottom up:
![](https://i.imgur.com/kFeTu7T.png)

![](https://i.imgur.com/wHMcaX4.png)

#### Complexity for heap construction:
![](https://i.imgur.com/isoTd0Z.png)
![](https://i.imgur.com/C7xbBCa.png)
