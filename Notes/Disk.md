---
title: "Disk"
date: 2022-11-08
lastmod: 2022-11-21
---
# Disk
## Disk Mechanics
A disk is made up of multiple cylinders (platters) each with a set of tracks
![](https://i.imgur.com/qm4b2xs.png)
> [!NOTE]
> Each platter consists of 2 surfaces which data can be read/written

Disk capacity calculation:
![](https://i.imgur.com/wMSn43v.png)
## Disk Access
> [!IMPORTANT]
> Data can only be accessed in units of blocks. Each block must be loaded from the disk into main memory. Only in main memory can we individually address each word. 

![](https://i.imgur.com/EXdyzUk.png)
### Seek time
Seek time depends on the total number of cylinders. However, it is not linear as the time taken is also dependent on the acceleration of the head.
![](https://i.imgur.com/u4VI4Vj.png)
### Rotational Delay
$$t = \frac{Angle}{Rotation \ Speed}$$
On average the rotational delay is 0.5 * t
### Transfer Time
$$t = \frac{block\ size}{transfer\ rate}$$
### Random Disk Access
Average seek time: let i be the cylinder of the block just accessed and j be the cylinder of the block to be accessed, N be the total number of cylinders
$$t = \frac{\sum_{i=1}^{N}\sum_{j=1}^{N}seektime(i-j)}{N^2}$$
### Sequential Disk Access
Average seek time is approximately 0 as the block to be accessed is likely to be in same cylinder
Average rotational delay is approximately 0 as the head points to the next block after current access
![](https://i.imgur.com/M0ylOeH.png)
## Disk Scheduling
### First Come First Serve
![](https://i.imgur.com/pvvLUwZ.png)
### Shortest Seek Time First
Similar to [](Notes/Process%20scheduling.md#Shortest%20Job%20First%20(SJF)%7Cshortest%20job%20first). Selects the request with the minimum seek time
from the current head position. It is susceptible to starvation.
![](https://i.imgur.com/GUJ7jRj.png)
### Elevator / Scan
Disk arm starts at one end of the disk, and moves toward the other end, servicing requests until it gets to the other end of the disk, where the head movement is reversed: 
![](https://i.imgur.com/0zMIHDP.png)
### C-Scan
Variant of elevator: after reversing direction, may not need to service requests immediately as more requests would be on the other end (uniform distribution)
![](https://i.imgur.com/6kbfmik.png)
### C-Look
Rather than reversing only when reaching one end of the disk, reverse after servicing the last request in the current direction.
![](https://i.imgur.com/SV3sASO.png)
### Comparison
- SSTF is common and has a natural appeal
- SCAN and C-SCAN (or LOOK and C-LOOK) perform better for systems that place a heavy load on the disk (since starvation is unlikely)
- Performance depends on the number and types of requests
- [](Notes/File%20Systems.md#Storage%20allocation%7CFile%20allocation%20methods) also affect the effectiveness of the algorithm. A linked or indexed file may generate requests wide apart.
- All the discussed algorithms (except for FCFS) do not solve the underlying issue of starvation. e.g. SCAN can be prevented from servicing the requests on the other end if new requests keep arriving at the same place.
## Disk Management
Formatting
- Divide the disk into sectors which the controller can read and write
Partitioning
- separating the disk into 1 or more groups of cylinders
Logical formatting
- Making a new file system by creating data structures to support file access across different partitions
![](https://i.imgur.com/l1rUNxA.png)
## Disk Reliability
### Striping
Uses a group of disks as one storage unit
- Each block is broken into several sub-blocks, with one sub-block stored on each disk
- Time to transfer a block into memory is faster because all sub-blocks are transferred in parallel
![](https://i.imgur.com/e1URc4E.png)
### Mirroring
Keeps a duplicate of each disk by using 2 physical disks in 1 logical disk. If one fails data can still be read by the other.
![](https://i.imgur.com/5eWJTR2.png)
### Redundant Array of Independent Disks (RAID)
Raid 0: Striping
Raid 1: Mirroring
Raid 0 + 1: Mirror of Stripes
![](https://i.imgur.com/U1dHjtf.png) 
Raid 1 + 0: Strip of Mirror
![](https://i.imgur.com/OnqoqHq.png)
- A difference occurs if there are at least 6 disks involved.
- Raid 10 has better fault tolerance: allows for disk 1,3,5 to be down and still functional
- Raid 01 degrades to Raid 0 when any disk fails. i.e. Will only be able to read a file from one group.
## Storing relational data
![](https://i.imgur.com/T38v3NN.png)
### Fields to Record
![](https://i.imgur.com/imcZwDT.png)

![](https://i.imgur.com/sAwIdeR.png)
### Record to Block
There a a few considerations when storing a record into a block
#### Supporting record separation
![](https://i.imgur.com/e0tWpq7.png)
#### Order of records
We can store records in the order of the primary key. Order can be maintained either physically (in memory) or logically (through a pointer)
## Practice Problems
![](https://i.imgur.com/bRiV76y.png)
a. 10 + 35 + 20 + 18 + 25 + 3 = 111
b. Order: 11->12->9->16->1->34->36
	1+3+7+15+33+2 = 61
c. Order: 11->12->16->34->36->9->1
	1+4+18+2+27+8 = 60
![](https://i.imgur.com/3Sg3EMf.png)
Total bytes per tuple = 8+17+1+4+4+4+1 = 39
Block contains meta data of 40bytes
a. 
Total byte without block meta data: $8\times 1024-40=8152$
Records: $8152\div 39=209.03$
209 records can be stored
b.
Total bytes per tuple:
17 byte character string needs to pad additional 3 bytes: 20 byte
1 byte needs to pad additional 3 bytes: 4 byte
$8+20+4+4+4+4+4=48$ bytes
Records: $8152\div 48 = 169$
169 Records
c.
Total bytes for block header:
$10\times 8 = 80$
Total bytes per tuple:
17 byte character string needs to pad additional 7 bytes: 24 byte
1 byte needs to pad additional 7 bytes: 8 byte
Record header: $2\times 8 + 8 = 24$
$24+8+24+8+8=72$ bytes
Records: $8192-80\div 72 = 112$
112 Records
![](https://i.imgur.com/zXULRoB.png)
a. A “sector” is a physical unit of the disk and a “block” is a logical unit, a creation of whatever software system – operation systems or database systems, for example – is using the disk. As we mentioned, it is typical today for blocks to be at least as large as sectors and to consist of one or more sectors. However, there is no reason why a block cannot be a fraction of a sector, with several blocks packed into one sector. In fact, some older systems did use this strategy.

With the block size increases, the # of blocks to be accessed for a relational table decreases but the transfer time then increases.
b. One block consists of multiple sectors. If these sectors are not sequential, the transfer time will be directly proportional to the RPM which the seek head is able to reach each sector.
![](https://i.imgur.com/R6TPaEL.png)
a. 
Capacity: $8\times2^{13}\times2^8\times2^9=2^{33}$bytes = 8GB
b.
1 round around the track is 256 sectors and 256 gaps, can be completed in $\frac{1}{3840}min$ or 1/64 seconds
To navigate 1 sector and 1 gap: $\frac{1}{64\times256}=0.061ms$
Min time to 1 sector and 0 gap: $0.061\times0.9=0.0549ms$ 
Min time for 8 sector and 7 gaps: 0.482ms

Max rotational delay occurs when we need to traverse 256-8 sectors and gaps to find the block
Max cylinder access occurs when we need to traverse all the tracks
Max time: $0.061\times248+0.482+17.4=33.01ms$
c.
There are 8192 cylinders.
If access is on cylinder 1000: block access time = average rotational delay
If access is on cylinder 1001: block access time = 1 track seek time + avg rotational delay
Average cylinder access: (1000+999...+0+1+...+7192)/8192 = 3218
Average cylinder access time: $3218/500+1=7.44ms$
Average  rotational delay: $\frac{1}{64\times2}=7.8ms$
Average total block access time: 15.24ms
