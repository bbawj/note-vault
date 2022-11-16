---
title: "Clock (or Second Chance) Policy"
---
# Clock Replacement Algorithm
<iframe width="560" height="315" src="https://www.youtube.com/embed/b-dRK8B8dQk?start=319" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>
## Idea
1. Keep a circular list of items in memory
2. A "clock hand" is used to suggest the next item for eviction
3. It initially points to the oldest page
4. Maintain a _use-bit_ for each item: this will tell us if the item has been accessed recently. Initially, all items have use-bit = 0.
5. Each time an item is accessed, change the use-bit to 1
6. When choosing item to be evicted:
	1. If the item has use-bit = 1, we reset it back to 0 (this item has used its second chance)
	2. Else we evict it (no second chance, replace it)
7. When bringing in the item (for our case):
	1. We do not set the use bit for the incoming page
	2. **We advance the clock hand to the next item after bringing in the page**
![](https://i.imgur.com/BhmQ5Ll.png)
At the worst case, the algorithm behaves the same as FIFO. E.g. when all the pages have their used bit set to 1, eventually the clock hand will return back to the oldest page and replace. 
