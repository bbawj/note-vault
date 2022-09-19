# Two Pass Algorithms
## Sort Based Algorithms
If data fits in memory, then we can use a standard sorting algorithm like quick-sort. If data does not fit in memory, then we need to use a technique that is aware of the cost of writing data out to disk.
### External Merge Sort
Idea is similar to [Merge Sort](Notes/Merge%20Sort.md). By breaking the data into pairs, we can sort each pair and merge them recursively.
We have 2 buffers for READ and 1 buffer for write: B(R) = 4
![](https://i.imgur.com/iWVMUnT.png)
Since each run may not fully fit inside memory, we can load segments of pairs of runs (e.g. 1 block from each run even though run contains 2 blocks), merge them and immediately write them back to the disk. 
### Two-Phase Multiway Merge Sort
We do not fully utilise all the buffers in external merge sort if M > 3. 
Idea:
1. Divide data until each chunk fits into M buffers
2. Sort each chunk and write it back to the disk
3. Use 1 input buffer for each sorted sublist. Implication: *hence there can only be M-1 sublists*
	1. Take the smallest of the head of each sublist (each in 1 buffer) and move into output
	2. If a buffer is empty: load tuples into the block from the same sorted sublist
	3. If no blocks remain in the sublist, ignore
> [! Note]
> Suppose R fits on B blocks. With M buffers each of 1 block, we can effectively sort M blocks of data each time. We can form $B(R)/M$ sorted sublists. In total we will only read and write B(R) blocks for this step.
> 
> Since we need 1 input buffer to represent each head of a sublist, we will need $B(R)/M \le(M-1), \ or\  B(R)\le M\times (M-1)$
>
> E.g. Suppose blocks are 64K bytes, and we have one gigabyte of  main memory. Then we can afford M  of 16K. Thus, a relation fitting in B  blocks can be sorted as long as B is no more than (16K)2 = 228. Since blocks  are of size 64K = 214, a relation can be sorted as long as its size is no greater  than 242 bytes, or 4 terabytes.  

If B(R) cannot be split into M sorted lists, first split B(R) into M chunks of M sorted lists. Apply algorithm to each of these M chunks to get M sorted lists. This forms the input for a third pass to form a fully sorted relation.
### Sort-Merge Join