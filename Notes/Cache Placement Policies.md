---
title: "Cache Placement Policies"
date: 2022-11-08
lastmod: 2022-11-21
---
# Cache Placement Policies
We need a way to decide where the data is placed when it is first copied into the cache where in the cache a copy of selected memory block will reside.
## Direct Mapped Cache
Each memory block can only be mapped to a single fixed cache line. 
![](https://i.imgur.com/T4JrNrk.png)
This means that there are no sets of cache memory, rather each line is a set itself. If another block maps to this set, the previous data block is replaced. E.g. Block 0, Block 64, Block 0, Block 64 etc. 
![](https://i.imgur.com/KK4AaiZ.png)
### Advantages and Disadvantages
- This placement policy is power efficient as it avoids the search through all the cache lines.
- It has lower cache hit rate, as there is only one cache line available in a set. Every time a new memory is referenced to the same set, the cache line is replaced, which causes conflict miss
## Fully Associative Cache
To increase flexibility, one way is to allow memory block to be placed anywhere in the cache. This can be framed as a single cache set holding all the cache lines. Index bits are no longer required since there is no distinguishing between sets:
![](https://i.imgur.com/U7gSm11.png)
### Advantages and Disadvantages
- Fully associative cache structure provides us the flexibility of placing memory block in any of the cache lines and hence full utilization of the cache.
- The placement policy provides better cache hit rate.
- Offers the use of a wider variety of replacement algorithms on miss
- The placement policy is slow as it takes time to iterate through all the lines.
## Set-Associative Cache
Trade-off between direct and fully associative cache.
![](https://i.imgur.com/HNweHLU.png)

![](https://i.imgur.com/VvQ2UVe.png)
