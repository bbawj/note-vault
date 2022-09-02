# Indexes
Indexes are needed to reduce the I/O required to find a record.

![](https://i.imgur.com/yT8Mx5j.png)
### Updating Indexes
1. Locate the targeted record or the place to hold new record
2. Update data file
3. Update index
## Clustered and Non-Clustered Indexes
![](https://i.imgur.com/etCAqtL.png)

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
## B+ Tree Index
Idea: build a multi-layer index in the structure of a [[B-tree]]
> [!Properties]
> 1. Each tree node is stored within a *block*
> 2. Each node stores at most n+1 pointers and n keys
> 3. Each level is an index
> 	- sorted within each node
> 	- sorted across nodes at the same level

> [!Leaf Node Properties]
> Consider a leaf node storing k+1 pointers (k <= n) and k keys
> 1. First k pointers are to records and last one is to the next leaf node
> 2. Each key is equal to the key that its corresponding pointer is pointing to in the record

> [!Internal Node Properties]
> Consider an internal node storing k+1 pointers (k <= n) and k keys
> 1. The ith key is the lower bound of the range of the i+1 pointer. The 2nd pointer points to a subtree that has the first key as the first element:![](https://i.imgur.com/yyB4CcW.png)
> 
> 2. There are k keys, each key is equal to the key that its corresponding pointer is pointing to in the record
## Practice Problems
![](https://i.imgur.com/t9W5FRr.png)
a. 
We need 300 key pointer pairs. Each block can hold 10 pairs. Total blocks = 300 / 10 = 30
b. 
Number of blocks for all records: 300/3 = 100 blocks
Each index will point to 1 block.
An index block can hold 3 index pairs.
Number of index blocks: 100 / 3 = 33.333 = 34
Worst case: retrieve the last record -> 34 I/O
c.
Another sparse index to point to a block of sparse index
Total 2nd level index blocks: 34/3 = 12
12 I/O
![](https://i.imgur.com/lhbhUXz.png)
a. 
Best case when inserting a record in the not full block with record 9. Insert 10
1 I/O to read the index block, 1 I/O to load the block with record 9. Total 2 I/O
b.
Worst case when inserting into first data block. Insert 0.
1 I/O to read index block. Need to load every data block to shift records down. Total 1+4=5 I/O.