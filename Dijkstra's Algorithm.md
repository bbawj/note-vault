# Dijkstra's Algorithm
[[Shortest Path Problem]]
Find the shortest path from source to another vertex. 

> [!NOTE]
> Dijkstra finds the shortest path in terms of the edge weights and not the number of edges. Hence, the shortest path may contain many edges. __This means that a change in edge weights will result in a different shortest path unless the number of edges in each path is the same.__

Features:
Shortest path from a single source to all nodes in weighted and directed graph.

Assumptions:
**Weights must be nonnegative**

### Data Structures Needed
![](https://i.imgur.com/2YaUIHm.png)

### Pseudocode
![](https://i.imgur.com/XJ0HfYt.png)
![](https://i.imgur.com/7e5woJm.png)

### Proof of Correctness
Why the greedy choice is optimal:![](https://i.imgur.com/MPkx0vS.png)

> [!important]
> This step is the reason for why graphs with negative weights do not ensure correctness of Dijkstra's .

![](https://i.imgur.com/NWnXHDo.png)
### Examples
