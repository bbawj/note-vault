# Hash Index
Idea: use a [[Notes/Hash Tables|hash table]]
1. We can directly hash a key which points to the record
2. We hash a key that points to a pointer to the record (keeps the index small)
Rather than a 1-1 mapping of hash to record/pointer, we use a bucket to store a set of records/pointers which share same hash value.
## Simple Hash
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
