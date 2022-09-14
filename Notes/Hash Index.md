# Hash Index
Idea: use a [[Notes/Hash Tables|hash table]]
1. Take a search key and hash it into an integer in the range of 0 to B-1 where B is the number of buckets
2. A bucket array holds the headers of B linked lists, one for each bucket
3. If a record has search key K, store the record by linking it to bucket list number h(K)
Implementations
1. We can directly hash a key which points to the record
- ![](https://i.imgur.com/RR2MGif.png)
2. Add a level of indirection: use an array of pointers to blocks to represent the buckets rather than an array holding data itself
- ![](https://i.imgur.com/FSUVAYH.png)
## Static Hash
### Insertion and Deletion
Bucket overflow can be handled naively by adding additional pointer to a separate block.
![](https://i.imgur.com/t1kBxL9.png)

![](https://i.imgur.com/JQjULyk.png)
## Extensible Hash Index
If the number of buckets is fixed from the start, we will end up with many additional pointers to additional overflow buckets. This incurs more I/O as more records are added.

Idea: use only the first i bits output from the hash function to point to a directory.
![](https://i.imgur.com/ObdGXq8.png)
### Insertion and Deletion
Increase i when overflow:
![](https://i.imgur.com/aYHZa3R.png)
Update the directory structure:
![](https://i.imgur.com/y7xlEcb.png)
Delete implementations:
1. Merge blocks when removing record
2. Do not merge blocks at all
### Duplicate Keys
Unable to allocate different directory for duplicate keys:
![](https://i.imgur.com/u7dl7Gs.png)
## Practice Problems
![[Pics/Pasted image 20220910143851.png]]
![[Excalidraw/Drawing 2022-09-10 14.41.50.excalidraw|400x400]]
i. 2 I/O
ii. All blocks needs to be searched: 11 I/O
![](https://i.imgur.com/1y39vic.png)
![[Excalidraw/Drawing 2022-09-10 14.52.53.excalidraw|400x400]]
![](https://i.imgur.com/O5QA06H.png)
a. Hash index on ID
b. B+ tree index on ID to support better range query
c. Multi key index on ID and name
![](https://i.imgur.com/lfjgONP.png)

![](https://i.imgur.com/tP2mU1d.png)
