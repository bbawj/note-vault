---
title: "Time Abstractions"
date: 2023-03-08
---
# Time Abstractions
## Clock drift
Every clock $C$ has an error $\rho$. The error bounds for *1* unit of time is defined as: $$1-\rho\le \frac{dC}{dt}\le1+\rho$$
## Leader-leases
A way to support faster reads by allowing direct reads from the leader's local state without any communication with the followers.

Problem: a leader can be disjoint from the rest of the network, causing another leader to be elected that changes the state of the system
![](https://i.imgur.com/dZKOxV6.png)
Leader leases ensure that there can only be 1 leader at a time and during this time reads from local state are allowed.
![](https://i.imgur.com/U3i0QTN.png)
- $t_L$: time since prepare was sent out. Process $p_1$ must start counting from $t_0$ to give a conservative estimate of possibly how long it can stay the leader. The actual time it can be a leader is starts at $t_2$ when majority promise received and ends at $t_3$ which assumes the clock is faster (lesser time) i.e. 1 unit of time is $1-\rho_1$
- $t_{prom}$: time since promise was sent out. Process $p_2$ will reject rounds assuming the clock is slower (waits longer) within the next 10s + time drift $10\rho$
## Interval Clocks
$C_i$ represents an interval $[lo, hi]$
![](https://i.imgur.com/eakVECm.png)
- Wait until the current low bound of timestamp of $p_1$ crosses the previous high bound for the operation timestamp to ensure linearizability $t_1 < t_2$ 