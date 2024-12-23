---
title: "Cache"
date: 2022-11-08
lastmod: 2022-11-21
---
# Cache
## Memory Organisation
Memory in cache is stored as cache lines. 
![](https://i.imgur.com/NUtGrGp.png)
A CPU will try to access cache through a memory address:
![](https://i.imgur.com/6Otcpnl.png)
### Instruction cache and Data cache
Storing these separately will allow in better parallelism. CPU is able to fetch instructions from instruction cache while writing to data cache for STUR instructions.
![](https://i.imgur.com/tA6cvb2.png)
## [Cache Placement Policies](Notes/Cache%20Placement%20Policies.md)
## [Cache Replacement Policies](Notes/Cache%20Replacement%20Policies.md)
## [Cache Write Policies](Notes/Cache%20Write%20Policies.md)
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
	- Reduces number of cache blocks for a fixed block size. This leads to increase in conflict miss as higher chance for different data map to the same blocks.
	- Increases miss penalty as more data needs to be replaced on a miss.
- Levels of cache
	- Using multi-level cache will reduce the miss penalty
![](https://i.imgur.com/PKhDsDq.png)
#### False sharing
An issue arises when different cores have cached values which share the same cache line. The picture below depicts how 2 cores try to access different independent values (X and Y) that resides on the same cache line in L3 cache.
![](https://i.imgur.com/5r29qAP.png)
If X and Y are highly used variables

- Writing to X invalidates the cache line in Core 2
- Writing to Y invalidates the cache line in Core 1
### Measuring Impact with CPI
$$
\begin{align}
&CPU_{time}=(CPU_{\text{execution cycles}}+\text{Memory stall cycles})\times\text{Cycle Time}\\
&CPU_{time}=((IC\times CPI)+(IC\times\%\text{Memory Access}\times\text{Miss Rate}\times\text{Miss Penalty}))\times \text{Cycle Time}
\end{align}
$$
![](https://i.imgur.com/38DnZ86.png)
L1 cache hit can be considered to be part of CPI ideal as it is often possible to complete the data access within the ideal clock cycles.
#### Example
![](https://i.imgur.com/pcXWyud.png)
#### Multi-level cache example
![](https://i.imgur.com/pN98QXK.png)
### Measuring Impact with Average Memory Access Time (AMAT) 
We need a way to measure the performance of cache, standalone from the performance of the CPU.
AMAT is the average time to access memory considering both hits and misses
$$
\begin{aligned}
AMAT&=\text{Hit Time}\times(1-\text{Miss Rate})+\text{Miss Rate}\times(\text{Hit Time}+\text{Miss Penalty})\\
&=\text{Time for hit}+\text{Miss Rate}\times\text{Miss Penalty}\\
\end{aligned}
$$
Note here that *Miss Penalty* is the loss in cycles for a miss, and not just the cost of main memory access. e.g If time to hit cache = 1, time to hit main memory = 100, miss penalty is $100-1=99$.
![](https://i.imgur.com/4RDpPr0.png)
One example to show that AMAT is superior would be to consider two different caches with similar miss rates, but drastically different hit times. Using the miss rate metric, we would rate both caches the same. Using the AMAT metric, a cache with a lower hit time or lower miss penalty will outperform a cache with a higher respective time, assuming all other variables are the same.
## Practice Problems
![](https://i.imgur.com/bo5A0np.png)
3 bit index, 2bit tag with 0 bit offset.
| Addr  | Index | Tag | H/M | State  |
| ----- | ----- | --- | --- | ------ |
| 10011 | 011   | 10  | M   | 001,10 |
| 00001 | 001   | 00  | H   |        |
| 00110 | 110   | 00  | H   |        |
| 01010 | 010   | 01  | M   | 010,01 |
| 01110 | 110   | 01  | M   | 110,01 |
| 11001 | 001   | 11  | M   | 001,11 |
| 00001 | 001   | 00  | M   | 001,00 |
| 11100 | 100   | 11  | M   | 100,11 |
| 10100 | 100   | 10  | M   |   100,10     |
![](https://i.imgur.com/8uzwhdW.png)
Bits for offset = $log_24=2$
2 way set associative cache: Each set contains 2 cache lines -> 2 blocks per set -> 8 bytes per set -> 2 sets
Bits for index -> 1
| Access | 10001101 | 10110010 | 10111111 | 100001100 | 10011100 | 11101001 | 11111110 | 11101001 |
| ------ | -------- | -------- | -------- | --------- | -------- | -------- | -------- | -------- |
| Tag    | 10001    | 10110    | 10111    | 10001     | 10011    | 11101    | 11111    | 11101    |
| Offset | 01       | 10       | 11       | 00        | 00       | 01       | 10       | 01       |
| Index  | 1        | 0        | 1        | 1         | 1        | 0        | 1        | 0        |
| H/M    | M        | M        | M        | H         | M        | M        | M        | H        |
| LRU1   | 10001    | 10110    | 10111    | 10001     | 10011    | 11101    | 11111    | 11101         |
| LRU2   | NA       | NA       | 10001    | 10111     | 10001    | 10110    | 10001    | 10110         |

Hit rate: 2/8 = 25%
![](https://i.imgur.com/ZUWtOC2.png)
a.
$$
\begin{align}
&T=IC\times CPI\times Period\\
&CPI_{ideal}=1.25\\
&\text{L1 stall cycles}=0.2\times0.2\times8=0.32\\
&\text{L2 stall cycles}=0.2\times0.2\times0.1\times30=0.12\\
&\text{CPI stall}=1.25+0.32+0.12=1.69\\
\end{align}
$$
b.
$$
\begin{align}
&\text{L1 stall cycles}=0.2\times0.2\times30=1.2\\
&\text{CPI stall}=1.25+1.2=2.45
\end{align}
$$
