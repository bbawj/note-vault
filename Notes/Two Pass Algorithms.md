---
title:"Two Pass Algorithms"
---
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
1. Divide data into $\frac{B}{M}$ sublists each of size $M$.
2. Sort each chunk and write it back to the disk
3. Use 1 input buffer for each sorted sublist. Implication: *hence there can only be M-1 sublists*
	1. Take the smallest of the head of each sublist (each in 1 buffer) and move into output
	2. If a buffer is empty: load tuples into the block from the same sorted sublist
	3. If no blocks remain in the sublist, ignore
> [! Note]
> Suppose R fits on B blocks. With M buffers each of 1 block, we can effectively sort M blocks of data each time. We can form $B(R)/M$ sorted sublists. In total we will only read and write B(R) blocks for this step.
> 
> Since we need 1 input buffer to represent each head of a sublist, we will need $B(R)/M \le(M-1), \ or\  B(R)\le M\times (M-1)\approx M^2$
>
> E.g. Suppose blocks are 64K bytes, and we have one gigabyte of  main memory. Then we can afford M  of 16K. Thus, a relation fitting in B  blocks can be sorted as long as B is no more than $(16K)^2 = 2^{28}$. Since blocks  are of size $64K = 2^{14}$, a relation can be sorted as long as its size is no greater  than $2^{42}$ bytes, or 4 terabytes.  

If B(R) cannot be split into sublists of size M $i.e. B(R)/M \ge M$, first split B(R) into  sublists of size $M(M-1)$. Apply 2PMMS to each of these $\frac{B(R)}{M(M-1)}$ chunks to get M sorted sublists. This forms the input for a third pass to form a fully sorted relation.
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
#### Implications
We need all sorted sublists from both relations to be able to fit in memory. 
Number of sorted sublists = $(B_R+B_S)/M$
$(B_R+B_S)/M \le M$
$(B_R+B_S) \le M^2$
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
#### Implications
The $t$ hash buckets saved in memory + the blocks for each head of each hash bucket written to disk must fit entirely in memory.
$$t\times\frac{B(R)}{k}+k-t\le M$$
#### Cost
Average bucket size is $B(S)/k$. We save write and read of $t\times B/k$ blocks for each relation
Total cost: $3(B(R) + B(S)) - \frac{2\times t}{k}(B(R)+B(S))$
## Comparison between Hash & Sort based Join
![](https://i.imgur.com/oxbKLxs.png)
## Practice Problems
![](https://i.imgur.com/zirdVdQ.png)
We can eliminate duplicates through sorting.
1. Divide data into M-1 buffers
2. Sort each buffer and write it back to the disk.
3. 2nd pass: read all the sorted sublists into M-1 buffers
4. Move the smallest of the heads of each sublist into the output if the max of the current output does not already contain this incoming value.
![](https://i.imgur.com/aFtQUQT.png)
a. M = 3. Sort merge join of S and T. Sort merge join of R and the result of S and T. 1 block to read R, 1 block to read S, 1 block to hold intermediate result of $R\Join S$
![](https://i.imgur.com/OzmoV03.png)
b. 
II: $M > B_T$
III: $M > B_T+(\text{Number of sorted sublists of S})+1$. Need all sorted sublists in buffer, and need 1 additional buffer to comb through R
![](https://i.imgur.com/R8RCdvQ.png)
~~Yes. Only refined sort merge join has comparable cost. In this case, refined sort merge join will not be applicable as both relations share multiple common values of attribute Y. Hence we are not able to fully load all tuples with the same attribute value into M buffers.~~
No. Hash join works by partitioning the relations based on the distinct keys. Since there are very few distinct values of Y, there will be very little partitions. 2nd pass will not work well since each partition must be loaded fully into memory buffer.
![](https://i.imgur.com/8v4AFJU.png)
a.
Strategy:
1. Use R as the outer relation as it is smaller.
2. Keep 1 hash bucket in memory. Need to be able to load all hash buckets from one of the relations into the remaining memory.
$$\begin{aligned}
\\\text{Let k be the number of hash buckets}
\\\text{No. of buffers for S}=1
\\\text{No. of buffers to write k-1 buckets to disk}=k-1
\\\text{No. of buffers to keep 1 bucket on memory}=B(R)/k
\\M\ge 1+k-1+B(R)/k=k+B(R)/k
\end{aligned}
$$
![](https://i.imgur.com/Z5Xc1G6.png)
b.
We save the cost of writing and reading 1 bucket of R to/from disk during the hashing process.
Each bucket contains $(400)/20=20blocks$
Total cost: $3(B_R+B_S)-2\times20=2660$
![](https://i.imgur.com/VRlK61d.png)
i. Set union operation involves duplicate elimination. Disk I/O = $B(R)+ B(R)\times B(S)=100010000$
ii. 
For refined sort merge join:
1. Read all blocks to perform 2PMMS: $2(B_R + B_S)=40000$
2. Perform join on merge phase: $1(B_R + B_S)=20000$
Total = 60000 i/o
![](https://i.imgur.com/hSue1Sr.png)
i. The minimum number of main memory blocks needed is that the relation with smaller number of hashed blocks must fit entirely into the main memory. Assuming that each block hashes to its own hash bucket, we need $M=10000$
![](https://i.imgur.com/HT6UlMm.png)
i. $3\times (B_R + B_S) =4500$
ii. $3\times (B_R + B_S) =4500$
![](https://i.imgur.com/4kcfjW9.png)
Procedure:
1. Sort R and S according to the attribute $x$ using two phase multiway merge sort
2. Load the first block of sorted R and S into main memory
3. Find tuple with the smallest value of $x$ and find matching tuples and write them to the output buffer
4. Repeatedly find the smallest value of $x$ once all tuples with the current smallest value are considered
5. Reload new blocks when either relation's blocks are fully considered
Cost:
$B_R=30000/30=1000$
$B_S=9000/10=900$
Sorting: $3(B_R+B_S)=5700$
Joining: $2(B_R+B_S)=3800$
Total: 9500
![](https://i.imgur.com/u0VLkYZ.png)
![](https://i.imgur.com/vib08yl.png)
i.
$B_R=50$
$(B_R+B_S)/10 \le10$
$50+\frac{2000}{x}\le100$
$x\ge40$
ii.
$3(B_R+B_S)=3(50+50)=300$
iii.
Use 9 buffers for one relation.
$B_R+B_R\times B_S/9=328$
iv.
$3(B_R+B_S)=3(50+50)=300$
v.
Choose between refined sort merge join and grace hash join as they have the smallest I/O cost. Choose refined sort merge join because the output will also be sorted.

