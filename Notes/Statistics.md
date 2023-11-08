---
title: "Statistics"
date: 2023-03-01
---
# Statistics
#moc
- [[Exponential Distribution]]
- [[Poisson Distribution]]
- [Statistical Inference](Notes/Statistical%20Inference.md)
## Median
The median can be thought of as the halfway point of a set, where the number of elements to the left of the median is the same as to the right of the median.

It is also the point which has the minimum distance to all other points in a sorted set.
If we have a point A, with L elements to the left of A and R elements to the right of A. 

Moving it to the *right* will update the total distance from that point to all others by a distance $d$ for all points in the set, whereby $d = |A_{prev} - A_{new}|$
$d\times (L + 1 - R)$
Similarly, moving it to the *left* will change the total distance from that point to all others by  this equation: 
$d\times (L - (R + 1))$

Hence, while $R > L$, distance is decreasing and vice versa. The median minimizes the distance to all other points.
## Probability vs Likelihood
![](https://i.imgur.com/9VJByFM.png)
