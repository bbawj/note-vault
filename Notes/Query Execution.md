# Query Execution
How do database management systems execute a particular query plan?
## Expression Evaluation
Represent queries in the form of an expression tree.
![](https://i.imgur.com/ql7hzHH.png)
## Processing Models
The processing model defines the organisation and execution of the operators.
### Iterator Model - Top Down Approach
**Pipelining**: the tuples produced by 1 operation is passed directly to the operation that called it.
![](https://i.imgur.com/riHAIqW.png)
As a result, several operations share main memory at any time and each tuple is processed 1 at a time.
### Materialisation Model - Bottom Up Approach
Each operator processes its input all at once and then emits the output all at once.
![](https://i.imgur.com/xeYn0ur.png)
### Vectorisation Model - Hybrid
Each operator implements a next function similar to the iterator model, but each operator emits a batch of tuples rather than just 1 by 1.
![](https://i.imgur.com/Z7HmgMN.png)
## Data Access Methods
How can the database management system access the data stored in tables?
### Sequential Scan
Simply iterate through each page in the table. However, not all blocks need to be accessed.
Optimisation strategies:
#### Zone Maps
![](https://i.imgur.com/Etn6fxL.png)
#### Heap Clustering
![](https://i.imgur.com/iCoSEtY.png)
### Index Scan
Pick an index to find the tuples needed.
Dependent on:
-  What attributes the index contains
-  What attributes the query references
-  The attribute's value domains
-  Predicate composition
-  Whether the index has unique or non-unique keys
#### Multi Index (Bitmap Scan)
![](https://i.imgur.com/BjBtOtS.png)
## Practice Problems
![](https://i.imgur.com/fL7CS7O.png)
Pipelining refers to how tuples are passed from one tuple to another.
An operator is blocking if it requires all of its children to emit all of their tuples. E.g. a sort operation needs all of its child tuples to begin execution, and the entire execution must be complete before any tuples can be emitted to the next operator. The result of a sort is only known at the end.
![](https://i.imgur.com/G0H4KYf.png)
a.
- Index scan: pick an index to find tuples needed
- Bitmap heap scan: sort data in a heap using a clustering index order, select tuples using this index
- Bitmap index scan: when query is to find tuple according to multiple attributes, use 1 index to find a set of tuples and another index to find another set. Take the intersection
b. 
- Heap: a heap is a set of unordered data pages
- Clustered index: data is packed in as few blocks as possible according to sorted order of this index key
- Heap clustering: sort a heap using a clustered index
