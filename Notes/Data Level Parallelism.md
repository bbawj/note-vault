---
title: "Data Level Parallelism"
---
# Data Level Parallelism
The same operation is performed on multiple data values concurrently in multiple processing units. Can reduce the Instruction Count to enhance performance
![](https://i.imgur.com/He0shTf.png)
## Processors
Different types of hardware can support different levels of data parallelism.
### Flynn's Processor Taxonomy
![](https://i.imgur.com/DkNRPll.png)
Advantages of SIMD > MIMD
- Allow sequential thinking yet achieves parallel speedup
- Reduced energy usage
- More efficient parallel efficiency
### Single Instruction Multiple Data (SIMD)
![](https://i.imgur.com/Pf45HmC.png)

![](https://i.imgur.com/pOAXefH.png)
#### Vector processor
One vector instruction can perform N computations, where N is the vector length.
- Reduces the number of instructions: less branching
- Less execution time with lower instruction count
- Simpler design as there is no requirement for data dependency check since each execution is independent
![](https://i.imgur.com/Afeo77i.png)
#### Array processor
![500](https://i.imgur.com/XpIX34N.png)
- Array processor works more like a true parallel system, with each processor able to run same instruction on different data
- Vector processor works in more of a pipelined fashion.
#### Multimedia Extensions (MMX)
![](https://i.imgur.com/bGaQAKF.png)
