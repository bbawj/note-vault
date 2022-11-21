---
title: "Computer Performance"
date: 2022-11-08
lastmod: 2022-11-21
---
# Computer Performance
## Execution time
One indicator of performance is __execution time__ of a program
$$\text{Performance} = \frac{1}{\text{Execution Time}} = \frac{1}{time_{end} - time_{start}}$$
$$\text{Execution Time} = \text{Instruction Count} \times \text{Clocks Per Instruction} \times \text{Clock Period}$$
### Instruction Count
### Clocks Per Instruction (CPI)
$$\text{Average CPI}=\text{Cycle Count}/\text{Instruction Count}$$
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
Let *E* be the fraction of program enhanced by Speedup $S_1$ and $1-E$ enhanced by Speedup $S_2$.
$$Speedup=\frac{1}{\frac{1-E}{S_2}+\frac{E}{S_1}}$$
### Gustafson's Law
__If the program is set to a fixed time period instead:__ do more parallel work in the same amount of time
![](https://i.imgur.com/aaL1XLz.png)
## Other performance metrics
![](https://i.imgur.com/O5MdwfF.png)
![](https://i.imgur.com/37SjOfC.png)
![](https://i.imgur.com/aR1Aglx.png)
## Practice Problems
![](https://i.imgur.com/c1jF6Xz.png)
$$
\begin{align}
&IC=200+500+300=1000
\\&T_c=100ns\\
&CPI_{average}=(200\times1+500\times2+300\times3)/1000=2.1\\
&T_{execution}=1000\times100\times2.1=210ms
\end{align}
$$
![](https://i.imgur.com/imtvm01.png)
a.
$$
\begin{align}
&10=200\times10^6\times\frac{1}{200\times10^6}\times CPI_{avg}\\
&CPI_{avg}=10\\
&5=160\times10^6\times\frac{1}{300\times10^6}\times CPI_{avg}\\
&CPI_{avg}=9.375
\end{align}
$$
b.
$$
\begin{align}
&IC_a=4\div(\frac{10}{200\times10^6})=80\times10^6\\
&IC_b=3\div(\frac{9.375}{300\times10^6})=96\times10^6
\end{align}
$$
![](https://i.imgur.com/3CA31jX.png)
a.
$$
\begin{align}
&T_{M2}=(1+2+3+4)\times\frac{1}{500\times10^6}\times4\\
&T_{M3}=(2+2+4+4)\times\frac{1}{750\times10^6}\times4\\
&\text{Speedup}=1.25
\end{align}
$$
b.
$$
\begin{align}
&T_{M1}=2\times\frac{1}{500\times10^6}\times1\\
&T_{M2}=1\times\frac{1}{500\times10^6}\times1\\
&T_{M3}=2\times\frac{1}{750\times10^6}\times1\\
&\text{Speedup M2 over M1}=2
&\text{Speedup M3 over M1}=1.5
\end{align}
$$

