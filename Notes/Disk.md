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