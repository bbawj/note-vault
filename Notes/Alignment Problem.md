---
title: "Alignment Problem"
date: 2022-11-08
lastmod: 2022-11-21
---
# Alignment Problem
![](https://i.imgur.com/RVNlE5M.png)
![](https://i.imgur.com/q0DPYhS.png)
## Problem Formulation
Let $n1$ and $n2$ represent the position of the character in the respective subsequence S1 and S2. The cost to align characters up till $n1$ and $n2$ can be found by finding the solutions to the cost of aligning characters $n1-1$ or $n2-1$.

![](https://i.imgur.com/vDg5i7a.png)

If the last 2 characters are equal, the cost to align is simply the cost to align the rest of the $n1-1$ and $n2-1$ characters.

If they are not equal, we can ignore 1 character, either from n1 or n2 (_resulting in $n1-1$ or $n2-1$_) by replacing it with an underscore: _resulting in a +1 cost_. Take the minimum of this 2.
## Strategy
![](https://i.imgur.com/fosblnp.png)

## Pseudocode
![](https://i.imgur.com/VdDY0lH.png)
