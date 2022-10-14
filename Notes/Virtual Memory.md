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
## Page Replacement
If there are no empty frames, OS needs to locate a victim to evict:
![](https://i.imgur.com/AfKxz30.png)

![](https://i.imgur.com/ubzcJYj.png)
### FIFO
![](https://i.imgur.com/f6t69Lk.png)

