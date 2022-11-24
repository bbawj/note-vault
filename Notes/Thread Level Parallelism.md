---
title: "Thread Level Parallelism"
date: 2022-11-08
lastmod: 2022-11-21
---
# Thread Level Parallelism
Distribute the workload among a set of concurrently running threads. Uses MIMD model.
## Multicore Processors
Not to be confused with *multiprocessors*:
- Multiprocessors: 2 or more CPUs in the same computer. Executes multiple programs faster.
- Multicore: 2 or more processors in a single CPU. Executes a single program faster through [](Notes/Threads.md#^7d353c%7Cmulti-threading)
![](https://i.imgur.com/fNXcCn7.png)
### Challenges
#### ILP wall
![](https://i.imgur.com/vPnlfce.png)
#### Power wall
Overcome power wall using multiple slow cores  
- Cores running at lower clock frequency and lower voltage can still deliver the desired performance using less power  
- Scale up the number of cores rather than frequency
![](https://i.imgur.com/qhjCvl6.png)
#### Memory wall
Widening gap between compute bandwidth and memory bandwidth could not be bridged - resulting in memory latency.

Overcome memory wall with memory parallelism via multiple threads
![](https://i.imgur.com/x8wuo0k.png)
#### Interconnect problem
![](https://i.imgur.com/DC9WD8x.png)
#### Cache Coherence
![](https://i.imgur.com/FjEitrp.png)

![](https://i.imgur.com/ktGNvyv.png)
![](https://i.imgur.com/uhtYbF5.png)
