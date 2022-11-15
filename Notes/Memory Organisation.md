# Memory Organisation
## Address Binding
A program needs to be loaded into memory to run. The machine code that is generated needs to be mapped to memory addresses in the system.
![](https://i.imgur.com/zbwjCpb.png)
Possible binding stages:
1. Compile time: memory location is known a priori, generating **absolute code** that must be recompiled if the starting location changes
2. Load time: compiler generates **relocatable code** and the binding is performed by the loader.
	![](https://i.imgur.com/48kWnKH.png)
3. Execution time: binding is delayed until run time and the process can be moved during its execution from one memory segment to another. Uses a logical address that is relative to a starting 0 point.
![](https://i.imgur.com/RAMNzgU.png)
## Fragmentation
>[! ]
>Internal Fragmentation: allocated memory may be larger than requested memory, this results in unusable memory within the partition
>
>External Fragmentation : unusable memory between partitions
# Memory Allocation
How to assign memory to different processes?
## Contiguous allocation
Logical address space of process remains contiguous in physical memory.
![](https://i.imgur.com/ZKxu4vz.png)
### Fixed partitioning
Memory is partitioned into regions with fixed boundaries. OS decides which partition to assign a process to depending on the available partitions. 
![400x400](https://i.imgur.com/8l3OiKG.png)
*Internal fragmentation*: as each partition is fixed, a process assigned to a partition might not take up the entire space of the partition, resulting in wasted unusable memory internal to the partition. 
### Dynamic partitioning
Do not partition the memory. Rather, the OS allocates the exact chunk of memory which a process requires, and keeps tracks of *holes* or available blocks of memory.
![](https://i.imgur.com/vEdUH9c.png)
*External fragmentation*: memory space between partitions (the holes) may be enough to satisfy a new request but is not contiguous and cannot be used. This can be solved by performing compaction, which shuffles memory contents to produce contiguous block of available memory.
### Dynamic Allocation Policies
![](https://i.imgur.com/QYCVh9D.png)
## Paging
^b8969e
Allow process to be allocated physical memory whenever it is available. 
> [!Idea:]
> 1. Divide the physical memory into fixed sized *frames*.
> 2. Divide logical memory of the **process** into *pages* the same size as frames.
> 3. Use a page table to map the logical memory to physical memory.
### Fragmentation
- Eliminating external fragmentation as every available physical memory space can be utilised. 
- Internal fragmentation still possible as the last page may not use up the entire frame.
### Address Translation
Split the logical address to map page to frame:
![](https://i.imgur.com/3pFbZJN.png)
Offset necessary to locate the byte-addressable piece of physical memory.
![](https://i.imgur.com/QPaWk69.png)
![](https://i.imgur.com/zOHdEXK.png)
### Hardware Support
The page table is stored in main memory with a page table base register (PTBR) pointing to it.
#### Translation Look-aside Buffers (TLB)
The page table is stored in memory and thus, to access a piece of physical memory, we require 1 memory access to the page table and 1 memory access to the actual memory. We can speed this up with a specialised cache for the page table.
![](https://i.imgur.com/YdHgJgB.png)
#### Effective access time
With TLB:
- 1 TLB access and 1 mem access on hit
- 1 TLB access and 2 mem access on miss
![](https://i.imgur.com/C0yZp2Q.png)
### Shared Pages
Reentrant or code which never changes during execution can be shared among processes by having processes used the same pages.
![500x500](https://i.imgur.com/A6xWe1W.png)
### Multi-level Paging
A page table can be large. Not efficient to have to fetch the entire page table for every memory LOAD/STUR instruction.
![](https://i.imgur.com/7pH5hvu.png)
#### Paging the page table
![](https://i.imgur.com/Xllezzl.png)
- Final level represents the physical memory space, 12 bits is needed for byte-addressing the 4KB memory. This leaves 20 bits for indexing pages.
- Second level represents the index to the first level page: There are $2^{20}$ pages. Each page in this level is 4 bytes so as to map to the address of a 4byte page address.  
- The root level represents the index to the 2nd level page: Since 1 block can store 4KB, to store information about the 4MB page table in 1 block will require $2^{20}\times4/2^{12}=2^{10}$ entries
Hence, 10 bits to access 1 out of $2^{10}$ pages which itself contains 10 bits to access 1 out of $2^{10}$ pages. Total of $2^{20}$ pages.
![](https://i.imgur.com/dhOBFTt.png)
### Inverted Page Table
> [! Idea:]
> Rather than each process having its own page table, we only keep a single table with 1 entry for each physical frame.
> 1. Each entry contains the `process-id` and `page-number` 
> 2. To find a matching entry for a logical address, we need to find the entry that has the equivalent proces_id and page_number pair

![](https://i.imgur.com/1syY6Pr.png)
We gain in terms of memory, as we no longer have a page table size that is proportional to logical addressing space. We lose in terms of speed, as we need to search the page table rather than addressing it directly with a page index.
## Non Contiguous Allocation (Segmentation)
![](https://i.imgur.com/r7Z4FUg.png)
### Addressing
Similar logic as with a regular paging scheme:
![](https://i.imgur.com/eTJ6naG.png)

![](https://i.imgur.com/KECm1QL.png)
### Fragmentation
- *External fragmentation*: as processes leave the system, occupied segments become holes in the memory
## Practice Problems
![](https://i.imgur.com/YwvOJ6S.png)

![600](Pics/Memory%20Organisation%202022-10-10%2023.41.06.excalidraw.md)
c. First fit lower overhead. Only moved 1 block compared to 3 blocks in best fit.
![](https://i.imgur.com/mv4UR97.png)
a. Fixed partitioning
b. Process memory is based on absolute addresses and is unable to be relocated for memory compaction. 
![](https://i.imgur.com/exWOYRb.png)
a. 200ns to access the page table. Another 200ns to access the memory frame. Total time: 400ns
b. $$\text{Total time}=0.75\times200+0.25\times400=250ns$$
![](https://i.imgur.com/2Wu9M80.png)
a.
To address a byte in a 1-Kbyte page will require 10 bits
Logical address: 22 bit page index, 10 bit offset
1 Gigabyte ($2^{30}$) physical memory will require, 30 bits to represent each byte address
![](https://i.imgur.com/bD0lkow.png)
b.
$2^{22}$ pages.
c.
Number of entries = Number of physical frames: $2^{30}/2^{10}=2^{20}$
![](https://i.imgur.com/zciAu6J.png)
a.
There are 8 pages, 3 bits are required to determine the page index.
Remaining 7 bits are used for the page offset, to address the individual byte in the page.
1000011011 -> Page 100 Offset 0011011 -> Page = 4 -> Frame number 01001
Physical address: $010010011011$
b.
There are 4 segments, 2 bits required to determine segment number
Remaining 8 bits used to address the physical memory unit
1000011011 -> Segment 10 Offset 00011011 -> Segment = 2
![](https://i.imgur.com/873Y5nz.png)

![](https://i.imgur.com/M0XVmUz.png)
a.
Number of pages = $2^{30}/1024=2^{20}$
Size of table = $2^{20}\times4=2^{22}$
b.
$$
\begin{align}
&\text{Size of page table 1st level}=2^{22}\\
&\text{Number of pages 2nd level}=2^{22}/2^{10}=2^{12}\\
&\text{Size of 2nd level}=2^{12}\times4=2^{14}\\
&\text{Number of pages 3rd level}=2^{14}/2^{10}=2^4\\
&\text{Size of 3rd level}=2^4\times4=2^6<1024B\\
&\text{Total no. of levels}=3
\end{align}
$$
c.
3rd level holds $2^4$ pages and requires 4 bits to index
Each page table can hold $2^{10}/2^2=2^8$ entries
8 bits needed to index into the 2nd level
Each page table in the 2nd level holds $2^8$ entries
8 bits needed to index into the final level
![](https://i.imgur.com/tIVXHKn.png)