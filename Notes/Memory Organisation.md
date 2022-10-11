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
# Memory Allocation
How to assign memory to different processes?
## Contiguous allocation
Logical address space of process remains contiguous in physical memory.
### Fixed partitioning
Memory is partitioned into regions with fixed boundaries. OS decides which partition to assign a process to depending on the available partitions. 
![400x400](https://i.imgur.com/8l3OiKG.png)
*Internal fragmentation*: as each partition is fixed, a process assigned to a partition might not take up the entire space of the partition, resulting in wasted unusable memory internal to the partition. 
### Dynamic partitioning
Do not partition the memory. Rather, the OS allocates the exact chunk of memory which a process requires, and keeps tracks of *holes* or available blocks of memory.
![](https://i.imgur.com/vEdUH9c.png)
*External fragmentation*: memory space between partitions (the holes) may be enough to satisfy a new request but is not contiguous and cannot be used. This can be solved by performing compaction, which shuffles memory contents to produce contiguous block of available memory.
## Non Contiguous allocation (Paging)
- Allow process to be allocated physical memory whenever it is available. 
- Eliminating external fragmentation as every available physical memory space can be utilised. 
- Internal fragmentation still possible as the last page may not use up the entire frame.
Idea:
1. Divide the physical memory into fixed sized *frames*.
2. Divide logical memory of the process into *pages*.
3. Use a page table to map the logical memory to physical memory.
### Address Translation
Split the logical address to map page to frame:
![](https://i.imgur.com/3pFbZJN.png)
Offset necessary to locate the byte-addressable piece of physical memory.
![](https://i.imgur.com/QPaWk69.png)
![](https://i.imgur.com/zOHdEXK.png)
### Translation Look-aside Buffers (TLB)
A cache for the page table.
![](https://i.imgur.com/YdHgJgB.png)
#### Effective access time
![](https://i.imgur.com/C0yZp2Q.png)
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
## Practice Problems
![](https://i.imgur.com/YwvOJ6S.png)

![[Pics/Memory Organisation 2022-10-10 23.41.06.excalidraw|600]]
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
b.
$2^{22}$ pages.
c.
