---
title:"Buffer Pools"
---
# Buffer Pools
An area of memory used as a buffer between the disk and the database system
![](https://i.imgur.com/vISJSv2.png)
## Page table
A page table is used to keep track of the pages loaded in the buffer pool. This helps the system to determine if the page is already in buffer without having to go to the disk.
## Buffer Manager
Used to control the memory in the buffer pool and provide the following features:
### Prefetching
While the current page is being processed, we can prefetch the next required pages to be accessed based on a query plan. This reduces total I/O time as operations are done in parallel.
### Scan sharing
If a query starts a scan and if there is one already doing this, it would attach to that query’s cursor. Once that current query is complete, the new query can continue to scan those pages that were initially skipped.
### Buffer Replacement Policies
Similar to [Cache Replacement Policies](Notes/Cache%20Replacement%20Policies.md).
[Page Replacement Policies](Notes/Page%20Replacement%20Policies.md)
## Practice Problems
![](https://i.imgur.com/H5D6bBe.png)
Process flow:
1. Fetch block 1
2. Process block 1 and Fetch block 2
3. Process block 2 and Fetch block 3
4. Process block 3
a. Total time is 4P; only 4 cycles needed
b. R + P + 2R = 3R + P
c. R + P + 2P = 3P + R

If no pre-fetching:
3(R+P) = 3R + 3P
![](https://i.imgur.com/06YYaaD.png)
![](https://i.imgur.com/SAiKBBL.png)
| Reference | LRU     | Optimal |
| --------- | ------- | ------- |
| 5         | ABCDE   | ABCDE   |
| 6         | ABCED   | ABCDE   |
| 7         | BCEDF/A | ABCDF/E |
| 8         | BCEFD   | ABCDF   |
| 9         | CEFDG/B | ABCDG/F |
| 10        | CEFGD   | ABCDG   |
| 11        | EFGDH/C | ABCDH/G |
| 12        | EFGHD   | ABCDH   |
| 13        | FGHDC/E   | ABCDH   |
The LRU is suboptimal in this case because it chooses to replace useful pages like B/C which are needed later. A more optimal strategy is to choose pages for replacement based on the corresponding level of the page in the B-Tree.
