# Two Pass Algorithms
The entirety of your data might not always fit in main memory. Two pass algorithms are a class of algorithms used to break down data into chunks which fit into main memory where we can then apply operations.
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
#### Cost
Let B be the number of blocks. B disk I/O to read in the first pass. B disk I/O to write sorted sublists. B disk I/O to read sorted sublists in second pass. **Total 3B disk I/O**.
### Sort-Merge Join
Idea: If the tuples are sorted, we can more easily join them together
Algorithm:
1. Sort R and S according to a key Y using 2PMMS
2. Merge R and S using only 2 buffers, one for the current block of R and another for the current block of S.
	1. Find the least value of y that is currently at front of blocks R and S
	2. If y does not appear at the front of the other relation, remove the tuples with value y
	3. Else, find tuples from both relations having sort key y, If necessary, read blocks from R and S until we are sure that there are no more y's in either relation. **We can use up to M buffers for this purpose
	4. Output all the tuples that can be formed by joining these tuples
	5. If either relation has no more unconsidered tuples in main memory, reload the buffer for that relation
![](https://i.imgur.com/n35JBo0.png)
#### Implications
Tuples with a common value of the sort key from both relations together must fit in M blocks. Consider if there are more than M such tuples. We will not be able to load the needed tuples for joining in 1 pass.
#### Cost
3B disk I/O per relation in step 1 for sorting, 1B per relation additionally to write fully sorted back to disk
1B disk I/O per relation in step 2
Total: $5(B(R) + B(S))$ disk I/O
### Refined Sort-Merge Join
Notice that in sort-merge join, the 2 relations are sorted first and then merged in distinct passes creating the greatest possible numbers of buffers available for joining tuples with common value.

If we do not need to worry about large number of tuples sharing common sort key, we can join the tuples in the merge phase of the sort:
1. Created M sorted sublists using sort key Y
2. Bring the first block of each sublist into a buffer
3. Repeatedly find the smallest y value. Find tuples of both relations that have value y.
#### Cost
Step 1 will require 2 I/O per block in order to read, form the sorted sublists and write back to disk
Step 2 will require 1 I/O per block in order to read and merge
Total: $3(B(R) + B(S))$ disk I/O
![](https://i.imgur.com/pbW2M5z.png)
## Hash Based Algorithms
Idea: hash the data into M buckets in order to fit into M main memory buffers for operations.
![](https://i.imgur.com/S5ERzLt.png)
### Grace Hash Join
1. Hash both relations to M-1 buckets using join key
2. Join every pair of matching hash key buckets in 1 pass
	1. Load all the buckets of one relation into M - 1 buffers
	2. One by one, load buckets from the other relation and join
#### Implications
The relation which smaller number of buckets must at least fit into M - 1 buffers.
$min(\frac{B(R)}{M-1}, \frac{B(S)}{M-1}) \le M-1 \approx min(B(R),B(S))\le M^2$
#### Cost
Step 1: 2 disk I/O per block to hash and write back to disk
Step 2: 1 disk I/O per block as we read 1 block once only before joining
Total: $3(B(R) + B(S))$ disk I/O
### Hybrid Hash Join
What if the size of one of our relations is much smaller than the M? $B(S) << M^2$
We can leave some buckets of S in memory without writing to disk such that we can join with B(R) immediately.
1. Partition S into k buckets, keep $t$ buckets in memory and $k-t$ buckets in the disk
2. Partition R into k buckets, first $t$ buckets are joined with S
3. Join $k-t$ pairs of buckets
#### Cost
Average bucket size is $B(S)/k$. We save write and read of $t\times B/k$ blocks for each relation
Total cost: $3(B(R) + B(S)) - \frac{2\times t}{k}(B(R)+B(S))$
## Comparison
![](https://i.imgur.com/oxbKLxs.png)
## Practice Problems
![](https://i.imgur.com/zirdVdQ.png)
We can eliminate duplicates through sorting.
1. Divide data into M-1 buffers
2. Sort each buffer and write it back to the disk.
3. 2nd pass: read all the sorted sublists into M-1 buffers
4. Move the smallest of the heads of each sublist into the output if the max of the current output does not already contain this incoming value.
![](https://i.imgur.com/aFtQUQT.png)
a. M = 2. External merge sort of S and T. Sort merge join of S and T. Sort merge join of R and the result of S and T.
![](https://i.imgur.com/OzmoV03.png)
b. 
II: $M > B_T$
III: $M > B_T+(\text{Number of sorted sublists of S})+1$. Need all sorted sublists in buffer, and need 1 additional buffer to comb through R
![](https://i.imgur.com/R8RCdvQ.png)
Yes. Only refined sort merge join has comparable cost. In this case, refined sort merge join will not be applicable as both relations share multiple common values of attribute Y. Hence we are not able to fully load all tuples with the same attribute value into M buffers.
![](https://i.imgur.com/8v4AFJU.png)
a.
Keep 1 hash bucket in memory. Need to be able to load all hash buckets from one of the relations into the remaining memory.
Buckets in memory: 1 bucket from S, all buckets from R.
39 blocks of MM must fit 400 blocks of S data. 39 buckets??
b.
We save the cost of writing and reading 1 bucket to/from disk during the hashing process.
Each bucket contains $(500+400)/39=30blocks$
Total cost: $3(B_R+B_S)-2\times30=2640$
