---
title:"Longest Common Subsequence"
---
# Longest Common Subsequence
![](https://i.imgur.com/fZELoJ2.png)
## Problem Formulation
Idea: The longest common subsequence up to certain position $i$ in the first sequence and $j$ in another sequence, can be found using the longest common sub sequences of the sequences up till $i-1$ and $j-1$.

The base case _when either subsequence is empty, there will not be any common characters_:
$$LCS(i,j)=0 $$

Common character found at (i,j):
$$LCS(i,j)= LCS(i-1,j-1)+1 $$
![](https://i.imgur.com/fo60yaQ.png)

No common character at position (i,j):
$$LCS(i,j)=max(LCS(i-1,j), LCS(i,j-1)) $$
![](https://i.imgur.com/4VME9tk.png)

## Strategy
![](https://i.imgur.com/kWJhW98.png)

## Pseudocode
![](https://i.imgur.com/J0RXXZu.png)

## Examples: Obtaining the subsequence characters
![Pasted image 20220502231603](Pics/Pasted%20image%2020220502231603.png)

![](https://i.imgur.com/5u32FFZ.png)

![](https://i.imgur.com/zOAFyUO.png)
