---
title: "Q-Learning"
date: 2022-11-08
lastmod: 2022-11-21
---
# Q Learning
Use temporal difference to update Q values at each time difference when the agent interacts with the environment.

![Pasted image 20220415193044](Pics/Pasted%20image%2020220415193044.png)
_New sample_: This refers to the maximum utility that can be achieved in the state we are entering, i.e. $V(S_{t+1})$ using previous estimates (previous iteration if synchronous)

[Grid World Scenario](Notes/Grid%20World%20Scenario.md):
Trial 1
| (1,1 right) | (1,2 right) |
| ----------- | ----------- |
| 0           | $Q_{new}=0+0.1\times(-5+0.9\times0-0)=-0.5$|
Trial 2
| (1,1 right) | (1,2 right)                                     | (2,2 right) |
| ----------- | ----------------------------------------------- | ----------- |
| 0           | $Q_{new}=-0.5+0.1\times(0+0.9\times0-(-0.5))=-0.45$ | $Q_{new}=0+0.1\times$<br>$(5+0.9\times0-0)=0.5$|

