# Computer Performance
## Execution time
One indicator of performance is __execution time__ of a program
$$\text{Performance} = \frac{1}{\text{Execution Time}} = \frac{1}{time_{end} - time_{start}}$$
$$\text{Execution Time} = \text{Instruction Count} \times \text{Clocks Per Instruction} \times \text{Clock Period}$$
### Instruction Count
### Clocks Per Instruction (CPI)
### Clock Period
Clock period is the inverse of clock frequency
__Memory wall problem__: a higher clock frequency may not result in better performance if the the time needed for memory access operations is slower than the CPU

## Speed-up
Speed-up is the factor over the original machine of improved performance
$$\text{Speedup} = \frac{Perf_a}{Perf_b}$$
We can define the execution time of an enhanced machine by the proportion of the program _E_ that is improved and _T_ the original time taken and _S_ the enhancement factor
$$T' = (T\times (1-E))+\frac{T\times E}{S}$$
#### Example
![](https://i.imgur.com/pM0zGxx.png)

### Amdahl's Law
__If the program is of a fixed workload:__
Let _E_ be the fraction of program that is enhanced via _parallelism_, with maximum enhancement factor $S = \infty$, the maximum speedup is $$\text{Max Speedup} =lim_{s\rightarrow \infty}\frac{1}{1-E+\frac{E}{S}}=\frac{1}{1-E}$$
### Gustafson's Law
__If the program is set to a fixed time period instead:__ do more parallel work in the same amount of time
![](https://i.imgur.com/aaL1XLz.png)

## Other performance metrics
![](https://i.imgur.com/O5MdwfF.png)
![](https://i.imgur.com/37SjOfC.png)
![](https://i.imgur.com/aR1Aglx.png)
