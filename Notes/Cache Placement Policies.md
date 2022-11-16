---
title:"Cache Placement Policies"
---
# Cache Placement Policies
We need a way to decide where the data is placed when it is first copied into the cache where in the cache a copy of selected memory block will reside.
## Direct Mapped Cache
Each memory block can only be mapped to a single fixed cache line. 
![](https://i.imgur.com/T4JrNrk.png)
This means that there are no sets of cache memory, rather each line is a set itself. If another block maps to this set, the previous data block is replaced. E.g. Block 0, Block 64, Block 0, Block 64 etc. 
![](https://i.imgur.com/KK4AaiZ.png)
## Fully Associative Cache
To increase flexibility, one way is to allow memory block to be placed anywhere in the cache. This can be framed as a single cache set holding all the cache lines. Index bits are no longer required since there is no distinguishing between sets:
![](https://i.imgur.com/U7gSm11.png)
## Set-Associative Cache
Trade-off between direct and fully associative cache.
![](https://i.imgur.com/HNweHLU.png)

![](https://i.imgur.com/VvQ2UVe.png)
