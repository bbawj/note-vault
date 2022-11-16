---
title:"Constraint Satisfaction Problem"
---
# Constraint Satisfaction Problem
There are a set of constraints which specify the allowable combinations of values

**Forward checking**: Pre-emptively removes inconsistent values from the domains of neighbouring variables. Prevents unnecessary expansion if constraints have already been violated. This is uninformed and can be improve with heuristics:		

## Constraint Propagation
Propagate the implications of constraints from assigning 1 variable to the other variables. 

![](https://i.imgur.com/L6wFN1R.png)Useful to optimize the __order of variable assignments__. This has the effect of making inconsistent assignments to **fail earlier in the search**, which enables more efficient pruning. This means that it may lead to more dead ends than:

![](https://i.imgur.com/6fdpYrr.png)Useful to optimize the __order of value assignments__. This prevents deadlocks and reduces backtracking by choosing the values which are most likely to work.

### Example Problems  
![](https://i.imgur.com/v7cEIT7.png)
![](https://i.imgur.com/Up48NUi.png)

Cryptarithmetic Puzzle
![](https://i.imgur.com/pRygTVJ.png)
![](https://i.imgur.com/bjOreuP.png)
