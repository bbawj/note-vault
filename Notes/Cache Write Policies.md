---
title:"Cache Write Policies"
---
# Cache Write Policies
How do we keep memory updated while writing on the cache?
## Write-through
Every write to the cache will lead to subsequent writes to the rest of the memory hierarchy, L1 -> L2 -> Main Memory -> Disk.
### Advantages
- Memory coherency
### Disadvantages
- High bandwidth requirement, every cache write results in high latency
- Memory becomes slow as size increases
## Write-back
Only write memory to rest of the memory hierarchy on cache replacement. 

Maintain the state of each cache line as the following
- Invalid: not present
- Clean: present and unmodified
- Dirty: present and modified
Update the state bit on modification. 
### Disadvantages
- Cache coherence problem: in multi-processors where each core maintains its own level of cache, if one core needs to access the data that has been modified by another core, they will get the stale data from memory as updated data is still in that core's own cache and has not been propagated.
- Coherent I/O: I/O devices are able to use [DMA](Notes/Direct%20Memory%20Access.md) and access stale copies of data in main memory.
