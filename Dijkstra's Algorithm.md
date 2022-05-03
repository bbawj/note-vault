# Dijkstra's Algorithm
[[Shortest Path Problem]]
Find the shortest path from source to all other vertices. 

> [!NOTE] Adding a positive constant to all edges
> Dijkstra finds the shortest path in terms of the edge weights and not the number of edges. Hence, the shortest path may contain many edges. __This means that a change in edge weights will result in a different shortest path unless the number of edges in each path is the same.__

> [!NOTE] Multiplying all edges by a positive constant
>![](https://i.imgur.com/IOvLCHy.png)
>
> This leads to a contradiction as Q is now a shorter path than P but P is the shortest path in G.

Assumptions:
**Weights must be nonnegative**
### Data Structures Needed
![](https://i.imgur.com/2YaUIHm.png)

### Pseudocode
![](https://i.imgur.com/XJ0HfYt.png)
![](https://i.imgur.com/7e5woJm.png)

We can also obtain the number of distinct shortest paths by using an additional n-size array to store the counts of paths which have the same shortest distance:
![](https://i.imgur.com/V9wRqwl.png)

### Proof of Correctness
Why the greedy choice is optimal:![](https://i.imgur.com/MPkx0vS.png)

> [!important]
> This step is the reason for why graphs with negative weights do not ensure correctness of Dijkstra's .

![](https://i.imgur.com/NWnXHDo.png)
### Examples
Manually computing shortest path:
![](https://i.imgur.com/iSpTwq8.png)
