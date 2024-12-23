---
title: "Least Recently Used Policy"
date: 2022-11-08
lastmod: 2022-11-21
---
# Least Recently Used Policy (LRU)
1. Keep track of when each item is accessed
2. When an item needs to be replaced, we will choose the one that has the oldest timestamp
## LRU-K
Rather than just taking the most recent access time for consideration, having a history of timestamps allows us to calculate the interarrival between references.
- Consider an item with the most recently accessed time. However, the access before that is longer ago, this could mean that the next access is more likely to not be this item compared to another item whose previous access was closer.
- Item with longest interarrival should therefore be dropped.

LRU-1 is just the normal LRU algorithm

1. Keep track of the K previous access timestamps of the item.
	- This can be done with a HISTORY array
2. When item on memory is accessed, we update the history array with the latest access time in a stack manner
3. When item needs to be evicted, we will choose the one that has the oldest Kth timestamp
## Implementation
### Stack
Swap used pages to the top of stack. Push out pages at the bottom.
![](https://i.imgur.com/ba4zW9K.png)
## LRU Approximation
Hardware support is necessary as exact algorithms are expensive and generally not available. It may be better to simply approximate LRU using an easier more supported implementation.
### [Clock (or Second Chance) Policy](Notes/Clock%20(or%20Second%20Chance)%20Policy.md)
