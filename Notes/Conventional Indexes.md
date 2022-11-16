---
title:"Conventional Indexes"
---
# Conventional Indexes
Indexes are needed to reduce the I/O required to find a record.

![](https://i.imgur.com/yT8Mx5j.png)
### Updating Indexes
1. Locate the targeted record or the place to hold new record
2. Update data file
3. Update index
## Clustered and Non-Clustered Indexes
Clustering index: indexes on an attribute is such that all the tuples with a fixed value for the search key of this index appear on as few blocks as can hold them.
![](https://i.imgur.com/etCAqtL.png)
If a relation is clustered (it must be sorted and packed together according to some attribute a) another index on another attribute _b_ would likely be non-clustered unless _a_ and _b_ are highly correlated.
![](https://i.imgur.com/QAhs451.png)
### Comparisons
#### Read
A range read of keys that are close together will result in high number of I/O:
![](https://i.imgur.com/q5Vyjfe.png)
#### Update
Clustered indexes will not be as good if the database goes through many update operations.
![](https://i.imgur.com/AeFICtg.png)
## Multi-layer Index
![](https://i.imgur.com/DosKPnI.png)
## Practice Problems
![](https://i.imgur.com/t9W5FRr.png)
a. 
Dense: We need 300 key pointer pairs. Each block can hold 10 pairs. Total blocks = 300 / 10 = 30
Sparse: 1 index pointer can point to a block of 3 records. Each block can hold 10 pointers. 1 index block represents 30 records. $300/30=10$ blocks needed
b. 
Worst case: retrieve the last record -> 10 I/O
c.
Another sparse index to point to a block of sparse index
Since the initial sparse index needs 10 blocks to represent, the second level index can use 1 block (10 pointers) to fully represent it.
I/O for 2nd level: 1
I/O for 1st level: 1
I/O to read record: 1
Total 3 I/O
![](https://i.imgur.com/lhbhUXz.png)
a. 
Best case when inserting a record in the not full block with record 9. Insert 10
1 I/O to read the index block, 1 I/O to load the block with record 9. Total 2 I/O
b.
Worst case when inserting into first data block. Insert 0.
1 I/O to read index block. Need to load every data block to shift records down. Total 1+4=5 I/O.
