# Cache
## Memory Organisation
Memory in cache is stored as cache lines. 
![](https://i.imgur.com/NUtGrGp.png)
A CPU will try to access cache through a memory address:
![](https://i.imgur.com/6Otcpnl.png)
### Instruction cache and Data cache
Storing these separately will allow in better parallelism. CPU is able to fetch instructions from instruction cache while writing to data cache for STUR instructions.
![](https://i.imgur.com/tA6cvb2.png)
## [[Cache Placement Policies]]
## [[Notes/Cache Replacement Policies|Cache Replacement Policies]]
## [[Cache Write Policies]]
## Performance
The key factor affecting cache performance is the effects of cache misses. When a cache miss occurs, compute cycles are needed to find a victim, request the appropriate data from memory, fill the cache line with this new block and resume execution.
### Types of Misses
#### Compulsory miss
First reference to a given block of memory. This is an inevitable miss as data has not been accessed before.
#### Capacity miss
This occurs when the current working set exceeds the cache capacity. Current useful blocks have to be replaced.
#### Conflict miss
When useful blocks are displaced due to placement policies. E.g. fully associative cache mapping
### Design considerations
- Number of blocks
	- More blocks means that we will have larger capacity and result in lesser capacity misses
- Associativity
	- Reduce conflict misses
	- Increases access time
- Block size
	- Larger blocks exploit spatial locality. More data in the same area is loaded together when requested.
	- Reduces compulsory misses since more data is loaded at once.
	- Increases miss penalty as more data needs to be replaced on a miss.
![](https://i.imgur.com/PKhDsDq.png)
### Measuring Impact with CPI
![](https://i.imgur.com/38DnZ86.png)
L1 cache hit can be considered to be part of CPI ideal as it is often possible to complete the data access within the ideal clock cycles.
#### Example
![](https://i.imgur.com/pcXWyud.png)
#### Multi-level cache example
![](https://i.imgur.com/pN98QXK.png)
### Measuring Impact with Average Memory Access Time (AMAT) 
We need a way to measure the performance of cache, standalone from the performance of the CPU.
AMAT is the average time to access memory considering both hits and misses
$$AMAT=\text{Time for hit}+\text{Miss Rate}\times\text{Miss Penalty}$$
![](https://i.imgur.com/4RDpPr0.png)
