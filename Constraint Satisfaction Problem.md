# Constraint Satisfaction Problem
There are a set of constraints which specify the allowable combinations of values

**Forward checking**: Do not waste time searching (expanding the search tree) if constraints have already been violated. Backtrack to the previous node. This is uninformed and can be improve with heuristics:		


## Constraint Propagation
Propagate constraints from assigning 1 variable to the other variables

![](https://i.imgur.com/L6wFN1R.png)Useful to optimize the order of variable assignments

![](https://i.imgur.com/6fdpYrr.png)Useful to prevent deadlocks, reduce backtracking

3## Example Problems  
![](https://i.imgur.com/v7cEIT7.png)
![](https://i.imgur.com/Up48NUi.png)

Cryptarithmetic Puzzle
![](https://i.imgur.com/pRygTVJ.png)
![](https://i.imgur.com/bjOreuP.png)
