# Virtual Memory
In [memory organisation](Notes/Memory%20Organisation.md), we assumed that for each program, its entirety has to be loaded into the memory. This means that the overall program size must be restricted to the size of physical memory.
>[!Aim: size of virtual memory limited by the address scheme of the computer and not the actual size of physical memory]
![](https://i.imgur.com/bFCvOzP.png)
## Swapping
A process can be swapped temporarily out of memory into a backing store.
- Backing store: fast disk large enough to accommodate copies of all memory images for all users
![500](https://i.imgur.com/WU02p1P.png)
Swapping time is primarily contributed by transfer time.
# Demand Paging
To support implementation of virtual memory, demand paging is used. Each process is divided into pages and each page can be loaded when it is in demand. *Initially, all pages are not in memory.*
![](https://i.imgur.com/fZZbIXq.png)
## Page Fault
![](https://i.imgur.com/Onvq9IR.png)
- 2,6: incurs context switch costs by the OS
- 4: incurs disk I/O cost to bring in the page
![](https://i.imgur.com/GjawaYa.png)
### Thrashing
Consider what occurs if a process does not have “enough” frames—that is, it does not have the minimum number of frames it needs to support pages in the working set. The process will quickly page-fault. At this point, it must replace some page. However, since all its pages are in active use, it must replace a page that will be needed again right away. Consequently, it quickly faults again, and again, and again, replacing pages that it must bring back in immediately. **This high paging activity is thrashing**.
![](https://i.imgur.com/swSweEg.png)
To prevent thrashing, we must provide a process with as many frames as it needs.
#### Working Set Model
Based on the concept of locality of reference: processes tend to refer to pages in a localised manner
Temporal locality: locations referred to recently are likely to be referenced again
Spatial locality: code and data are usually clustered physically
![](https://i.imgur.com/8KemQbT.png)
Implementation:
![](https://i.imgur.com/6dzOR3i.png)
![](https://i.imgur.com/64zPdG0.png)
Additional metrics:
![](https://i.imgur.com/hcbkUG6.png)
#### Page Fault Frequency
![](https://i.imgur.com/rnLcCTo.png)
- Establish an upper and lower bound for acceptable page-fault rate and allocate and deallocate frames accordingly
## Page Replacement
If there are no empty frames, OS needs to locate a victim to evict:
![](https://i.imgur.com/AfKxz30.png)
### Belady's Anomaly
![500](https://i.imgur.com/ubzcJYj.png)
> [!Inclusion Property]
> Pages loaded in *n* frames is always a subset of pages in *n+1* frames
>
> An algorithm does not suffer from Belady's anomaly if it satisfies the inclusion property. This is because such an algorithm will only increase its total coverage of available frames and does not replace any frame that was previously loaded.
>
> An example with FIFO:
>
> ![400](https://i.imgur.com/Z2DZRrv.png)
### Policies
[Page Replacement Policies](Notes/Page%20Replacement%20Policies.md)
### Allocation of Frames
#### Fixed allocation
Give a process a fixed number of frames in memory for execution.
#### Variable allocation
Allow the number of frames allocated to the process to vary across its execution lifetime.
### Scope of Replacement
#### Global
Process selects a replacement frame from the set of all frames; one process can take a frame from another process. Implication: performance of the process depends on external processes.
#### Local
Each process selects only from its own set of allocated frame. Implication: may hinder other processes by not making available its less used pages/frames
![](https://i.imgur.com/BEphKf6.png)
## Other Considerations
### Program Structure
Specific choice in data structures used and program structure can affect the performance of demand paging. 
![](https://i.imgur.com/ZSGI7sq.png)
- Use of data structures like a hash set or linked list might offer *lesser* locality of reference than a data structure like an array, possibly reducing performance
# Practice Problems
![](https://i.imgur.com/4ukwOxR.png)
a. FIFO will replace the earliest loaded page. Page 2
b. Replace the first page with R=0. Page 0.
c. Replace the oldest accessed page. Page 1.
![](https://i.imgur.com/zybj7sG.png)
| Tick | Page Ref | FIFO      | Clock     | LRU       | Loaded     | R       | Accessed   |
| ---- | -------- | --------- | --------- | --------- | ---------- | ------- | ---------- |
| 1    | 0        | 0         | 0         | 0         | 1          | 0       | 1          |
| 2    | 1        | 0,1       | 0,1       | 0,1       | 1,2        | 0,0     | 1,2        |
| 3    | 6        | 0,1,6     | 0,1,6     | 0,1,6     | 1,2,3      | 0,0,0   | 1,2,3      |
| 4    | 0        | 0,1,6     | 0,1,6     | 0,1,6     | 1,2,3      | 1,0,0   | 4,2,3      |
| 5    | 3        | 0,1,6,3   | 0,1,6,3   | 0,1,6,3   | 1,2,3,5    | 1,0,0,0 | 4,2,3,5    |
| 6    | 4        | 4,1,6,3 F | 0,4,6,3 F | 0,4,6,3 F | 6,2,3,5    | 0,0,0,0 | 4,6,3,5    |
| 7    | 0        | 4,0,6,3 F | 0,4,6,3   | 0,4,6,3   | 6,7,3,5    | 1,0,0,0 | 7,6,3,5    |
| 8    | 1        | 4,0,1,3 F | 0,4,1,3 F | 0,4,1,3 F | 6,7,8,5    | 0,0,0,0 | 7,6,8,5    |
| 9    | 0        | 4,0,1,3   | 0,4,1,3   | 0,4,1,3   | 6,7,8,5    | 1,0,0,0 | 9,6,8,5    |
| 10   | 3        | 4,0,1,3   | 0,4,1,3   | 0,4,1,3   | 6,7,8,5    | 1,0,0,1 | 9,6,8,10   |
| 11   | 4        | 4,0,1,3   | 0,4,1,3   | 0,4,1,3   | 6,7,8,5    | 0,0,0,1 | 9,11,8,10  |
| 12   | 6        | 4,0,1,6 F | 0,4,6,3 F | 0,4,6,3 F | 6,7,8,12   | 0,0,1,1 | 9,11,12,10 |
| 13   | 3        | 3,0,1,6 F | 0,4,6,3   | 0,4,6,3   | 13,7,8,12  | 0,0,1,1 | 9,11,12,13 |
| 14   | 4        | 3,4,1,6 F | 0,4,6,3   | 0,4,6,3   | 13,14,8,12 | 0,1,1,1 | 9,14,12,13 |
The first 4 page loads are always page faults.
a. $4+6=10$
b. $4+3=7$
c. $4+3=7$
![](https://i.imgur.com/Br1EZaM.png)
a.
If N <= M:
No more page faults will occur after all distinct pages are loaded into memory.
Lower bound = N, Upper bound = N
If N > M:
Lower bound occurs on the minimum number of page faults. Every unique page will result in a page fault = N
Upper bound occurs when every page reference is a page fault = L
b. No. LRU does not guarantee optimality.
![](https://i.imgur.com/As7tUFf.png)
a.
| Page Ref | LRU     | Accessed        | Fault |
| -------- | ------- | --------------- | ----- |
| -        | 1,0,3,2 | 161,160,162,163 | N     |
| 4        | 1,4,3,2 | 161,164,162,163 | Y     |
| 0        | 0,4,3,2 | 165,164,162,163 | Y     |
| 0        | 0,4,3,2 | 166,164,162,163 | N      |
| 0        | 0,4,3,2 | 167,164,162,163 | N      |
| 2        | 0,4,3,2 | 166,164,162,167 | N      |
| 4        | 0,4,3,2 | 166,168,162,167 | N      |
| 2        | 0,4,3,2 | 166,168,162,169 | N      |
| 1        | 0,4,1,2 | 166,168,170,169 | Y      |
| 0        | 0,4,1,2 | 171,168,170,169 | N      |
| 3        | 0,3,1,2 | 171,172,170,169 | Y      |
| 2        | 0,3,1,2 | 171,172,170,173 | N      |
4 Page Faults.
b.
| Page Ref | Working Set | Fault |
| -------- | ----------- | ----- |
| -        | 0,1,3,2     | -     |
| 4        | 1,3,2,4     | Y     |
| 0        | 3,2,4,0     | Y     |
| 0        | 2,4,0       | N     |
| 0        | 4,0         | N     |
| 2        | 0,2         | Y     |
| 4        | 0,2,4       | Y     |
| 2        | 0,4,2       | N     |
| 1        | 4,2,1       | Y     |
| 0        | 4,2,1,0     | Y     |
| 3        | 2,1,0,3     | Y     |
| 2        | 1,0,3,2     | N     |
7 page faults.
![](https://i.imgur.com/oAKQN2f.png)
Illustrates a thrashing situation.
a. F. CPU is already under utilised
b. ~~T. Increase available memory for pages to be loaded for processes~~ False. A larger paging disk does not allow for more page frames in memory.
c. F. Already not enough memory for existing programs
d. T. Swap out some programs to the backing store to allow for more pages for existing programs
e. T. More pages can be stored in memory
f. T. Less time spent in I/O
g. F. 
- Increasing page size will result in fewer page faults if data is accessed sequentially
- If data access is random, more paging actions could occur because fewer pages can be kept in memory