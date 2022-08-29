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
### Transfer Time
$$t = \frac{block\ size}{transfer\ rate}$$
### Random Disk Access
Average seek time: let i be the cylinder of the block just accessed and j be the cylinder of the block to be accessed, N be the total number of cylinders
$$t = \frac{\sum_{i=1}^{N}\sum_{j=1}^{N}seektime(i-j)}{N^2}$$
Average rotational delay: half circle rotation time
### Sequential Disk Access
Average seek time is approximately 0 as the block to be accessed is likely to be in same cylinder
Average rotational delay is approximately 0 as the head points to the next block after current access

![](https://i.imgur.com/M0ylOeH.png)
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