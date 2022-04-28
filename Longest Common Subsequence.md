# Longest Common Subsequence
![](https://i.imgur.com/fZELoJ2.png)
## Problem Formulation
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

#### Obtaining the subsequence characters
