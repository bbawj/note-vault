# Query Execution
How do database management systems execute a particular query plan?
## Processing Models
The processing model defines the organisation and execution of the operators.
### Iterator Model - Top Down Approach
Pipelining: the tuples produced by 1 operation is passed directly to the operation that called it.
![](https://i.imgur.com/riHAIqW.png)
As a result, several operations share main memory at any time and each tuple is processed 1 at a time.
### Materialisation Model - Bottom Up Approach
Each operator processes its input all at once and then emits the output all at once.
![](https://i.imgur.com/xeYn0ur.png)
### Vectorisation Model - Hybrid
Each operator implements a next function similar to the iterator model, but each operator emits a batch of tuples rather than just 1 by 1.
![](https://i.imgur.com/Z7HmgMN.png)
