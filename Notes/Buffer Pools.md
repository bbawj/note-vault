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
Similar to [[Cache Replacement Policies]].

1. [[Least Recently Used Policy]]
2. [[Clock (or Second Chance) Policy]]
