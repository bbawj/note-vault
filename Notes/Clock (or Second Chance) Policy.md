# Clock Replacement Algorithm
<iframe width="560" height="315" src="https://www.youtube.com/embed/b-dRK8B8dQk?start=319" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>
## Idea
1. Keep a circular list of items in memory
2. A "clock hand" is used to suggest the next item for eviction
3. Maintain a _use-bit_ for each item: this will tell us if the item has been accessed recently. Initially, all items have use-bit = 0.
4. Each time an item is accessed, change the use-bit to 1
5. When choosing item to be evicted:
	1. If the item has use-bit = 1, we reset it back to 0 (this item has used its second chance)
	2. Else we evict it (no second chance, replace it)
![](https://i.imgur.com/BhmQ5Ll.png)
At the worst case, the algorithm behaves the same as FIFO. E.g. when all the pages have their used bit set to 1, eventually the clock hand will return back to the oldest page and replace. 