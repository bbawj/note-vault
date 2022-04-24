## Components of MDP
![[Pasted image 20220415205145.png]]

Formulating an MDP 
	![[Pasted image 20220415223757.png]]	
	![[Pasted image 20220415223941.png]]

## The Bellman Equation
$$ V_{i+s}(s) =max_a(\sum P(s'|s,a)(r(s,a,s')+\gamma V(s')) $$
### Value iteration
![[Pasted image 20220415160057.png]]
![[Pasted image 20220415160224.png]]

[[Grid World Scenario]]:

| $V_i$ | (1,1) | (1,2)                                               | (1,3) | (2,1)                                                 |                                                                                 (2,2)                                                                                  | (2,3) |
| ----- | ----- | --------------------------------------------------- | ----- | ----------------------------------------------------- |:----------------------------------------------------------------------------------------------------------------------------------------------------------------------:|:-----:|
| $V_0$ | 0     | 0                                                   | 0     | 0                                                     |                                                                                   0                                                                                    |   0   |
| $V_1$ | 0     | 0                                                   | 0     | 0                                                     | $Up = 0.8\times0+0.1\times5+0.1\times0+0.9\times0$ $Down=0.8\times0+0.1\times5+0.1\times0+0.9\times0$ $Left=0.8*0+0.1*5+0.1*0=0.5$ $Right=0.8*5+0.1*0+0.1*0=4$ Max = 4 |   0   |
| $V_2$ | 0     | $Up=0.8\times(0+0.9*4)+0.1\times0+0.1\times-5=2.38$ | 0     | $Right=0.8\times(0+0.9*4)+0.1\times0+0.1\times0=2.88$ |$Right=0.8\times(5+0.9*0)+0.1\times(0+0.9*4)=4.36$                                                                      | 0      |

#### Obtain the optimal policy
![[Pasted image 20220415211535.png]]
Calculate V(s) for each state and action to obtain the best policy.
![](https://i.imgur.com/Z8mwlad.png)

### Policy iteration
Steps
1. Start with random policy and V(s)=0 for 1st iteration
2. Policy evaluation (calculate V) until stable
3. Policy Improvement (calculate new policy)

![[Pasted image 20220415210325.png]]
Asynchronous refers to in series: value calculated from previous steps are used in the next state calculations  

Without the transition function or probabilities we need [[Monte Carlo Policy]] reinforcement learning.