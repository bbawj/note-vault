---
title:"Monte Carlo Policy"
---
# Monte Carlo
![Pasted image 20220415190047](Pics/Pasted%20image%2020220415190047.png)
Estimate the value function from sampling:
![Pasted image 20220415190342](Pics/Pasted%20image%2020220415190342.png)

**First visit MC**: average returns only for first time (s,a) is visited in an episode/trial
Repeated visits of (s,a) in the trial does not constitute a new learning condition

[Grid World Scenario](Notes/Grid%20World%20Scenario.md):
Discount factor $\gamma = 1$
| Trial                      | (1,1)                    | (2,2)   |
| -------------------------- | ------------------------ | ------- |
| (1,1)->(1,2)->(1,3)        | $G_t=0+0+1^2\times-5=-5$ | NA       |
| (1,1)->(1,2)->(2,2)->(2,3) | $G_t=0+0+5=5$            | $G_t=5$ |
| (1,1)->(2,1)->(2,2)->(2,3) | $G_t=5$                  | $G_t=5$        |
Monte Carlo Estimates Q for (1,1): $\frac{5+5-5}{3}=\frac{5}{3}$
Monte Carlo Estimates Q for (2,2): $\frac{5+5}{2}=5$

This only works when we have the entire path ending in a goal state, what if we do not have this whole path? Use [Q-Learning](Notes/Q-Learning.md)

