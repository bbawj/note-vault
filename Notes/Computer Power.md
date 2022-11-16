# Computer Power
## Power dissipation
### Dynamic Power
Dissipated only when computation is performed
![](https://i.imgur.com/2gdh0rR.png)
### Static Power
Due to leakage current and dissipated whenever the system is powered on
![](https://i.imgur.com/F74nRcy.png)
### Total Power
![](https://i.imgur.com/mqmylWP.png)

When voltage is reduced, the threshold that is used to differentiate between a logic 1 and logic 0 output will be reduced. If this threshold is small, a high frequency will be more prone to noise that could alter the output.
## Reducing power consumption
1. Component design
2. Power gating: shutting down unused components
3. Clock gating: reduce unnecessary switching
4. Reduce data movement, number of memory access and register transfer
### The problem between power and energy
![](https://i.imgur.com/89jDDi7.png)
## Practice Problems
![](https://lh3.googleusercontent.com/Sf-kXoA3oxFIRLiK1iNx2GDOwkOTj2jqEnJ8UDBHWEv6oNce1CIu5uc08o4DdkXaCwpQxbFF3G5NlKheJgCumElgdgDplXXXZRCgFGKdiXR_7IuC7SSEHvfjR0TXkCh2c3dC9Z2QoZl99jgrz6B0Cg0)
__Case 1__:
Change in voltage = 3.3 - 3 / 3 = 10%
i. 
New frequency = $1.1 * 300 = 330 MHz$
Change in dynamic power = $\frac{3.3^2 \times 330 }{3^2*300} - 1 = 33.1\%$
ii. Change in static power = 10%
iii. Perf is directly proportional to freq. Change in perf = 10%
iv.
$$
\begin{aligned}
&\text{Perf is = 1/Time} \\
&\text{Dynamic energy} \\
&\text{Increase in performance by 10\% means that the change in time is 1/1.1}\\
&\text{Consumption change} = 1.331 P * 1/1.1T - P*T = 21\%increase\\ 
&\text{Static energy:}\\
&1.1P * 1/1.1T - 1 = 0\%
\end{aligned}
$$

__Case 2__:
i. $\frac{3.3^2 - 3^2}{3^2} = 21\%$
ii. 10% increase
iii. No change in frequency so no change in performance
iv. 
Dynamic energy consumption: 21% increase
Static energy consumption: 10% increase
