---
title: "Markov Decision Process"
date: 2022-11-08
lastmod: 2022-11-21
---
## Components of MDP
![Pasted image 20220415205145](Pics/Pasted%20image%2020220415205145.png)

### Formulating an MDP 
	![Pasted image 20220415223757](Pics/Pasted%20image%2020220415223757.png)	
	![Pasted image 20220415223941](Pics/Pasted%20image%2020220415223941.png)

## The Bellman Equation
$$ V_{i+s}(s) =max_a(\sum_{s'} P(s'|s,a)(r(s,a,s')+\gamma V(s')) $$
### Value iteration
![Pasted image 20220415160057](Pics/Pasted%20image%2020220415160057.png)
![Pasted image 20220415160224](Pics/Pasted%20image%2020220415160224.png)

[Grid World Scenario](Notes/Grid%20World%20Scenario.md):

| $V_i$ | (1,1) | (1,2)                                               | (1,3) | (2,1)                                                 |                                                                                 (2,2)                                                                                  | (2,3) |
| ----- | ----- | --------------------------------------------------- | ----- | ----------------------------------------------------- |:----------------------------------------------------------------------------------------------------------------------------------------------------------------------:|:-----:|
| $V_0$ | 0     | 0                                                   | 0     | 0                                                     |                                                                                   0                                                                                    |   0   |
| $V_1$ | 0     | 0                                                   | 0| 0| $Up = 0.8\times0+0.1\times5+0.1\times0+0.9\times0$ $Down=0.8\times0+0.1\times5+0.1\times0+0.9\times0$ $Left=0.8*0+0.1*5+0.1*0=0.5$ $Right=0.8*5+0.1*0+0.1*0=4$ Max = 4 |   0   |
| $V_2$ | 0     | $Up=0.8\times(0+0.9*4)+$<br>$0.1\times0+0.1\times-5=2.38$ | 0     | $Right=0.8\times(0+0.9*4)+$<br>$0.1\times0+0.1\times0=2.88$ |$Right=0.8\times(5+0.9*0)+$<br>$0.1\times(0+0.9*4)=4.36$| 0      |

#### Obtain the optimal policy
![Pasted image 20220415211535](Pics/Pasted%20image%2020220415211535.png)

> [!NOTE]
> Once we have V*,  we can use V* in the bellman equation for each State and Action to obtain the corresponding Q(S,A) value. $\pi*$ is the action which obtains max Q(S,A) i.e. V(S). 

![](https://i.imgur.com/Z8mwlad.png)

### Policy iteration
> [!NOTE] Steps
> 1. Start with random policy and V(s)=0 for 1st iteration
> 2. Policy evaluation (calculate V) until stable
> 	1. For each state s calculate V(s) using the action in the policy _(this is different from calculating V(s) using max $Q(s,a)$)_ 
> 3. Policy Improvement (calculate new policy)
> 	1. For each state s calculate $Q(s,a)$ of all actions using the stabilized V(s) values.
> 	2. Update the policy with the actions which maximize $Q(s,a)$ of each state

![Pasted image 20220415210325](Pics/Pasted%20image%2020220415210325.png)
Asynchronous refers to in series: value calculated from previous steps (same iteration) are used in the next state calculations.

Synchronous refers to in parallel: All V(s) values are calculated using the previous iteration V(s) estimates.
 
Without the transition function or probabilities we need [Monte Carlo Policy](Notes/Monte%20Carlo%20Policy.md) reinforcement learning.
