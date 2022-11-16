---
title:"Multi Key Index"
---
# Multi Key Indexes
Motivation: we want to be able to have efficient queries on multiple attributes
`SELECT * WHERE DEPT = "TOY" AND SAL > 50`
![](https://i.imgur.com/cWhVNxn.png)
## Geographic Data
Build index on y and ax iteratively until each partition contains at most 2 records:
![](https://i.imgur.com/8JgjdxB.png)
## Grid Index
![](https://i.imgur.com/9Od9nth.png)
Issues:
- Records may not be allocated into the cells evenly depending on the partitioned ranges: result in overflows or inefficient use of space
![](https://i.imgur.com/AGipsAE.png)

![](https://i.imgur.com/KXrMNcL.png)
## Partitioned Hash
Idea: combine the hash value of each key from [hash index](Notes/Hash%20Index.md) to form a single index.
![](https://i.imgur.com/oCT4h3M.png)

