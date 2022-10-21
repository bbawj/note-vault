# Virtual Memory
In [[Notes/Memory Organisation|memory organisation]], we assumed that for each program, its entirety has to be loaded into the memory. This means that the overall program size must be restricted to the size of physical memory.
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
As the degree of multi-programming increases, there comes a point where CPU utilisation drops dramatically due to more frequent page faults.
![](https://i.imgur.com/swSweEg.png)
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
[[Notes/Page Replacement Policies|Page Replacement Policies]]
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
## Practice Problems
![](https://i.imgur.com/4ukwOxR.png)
