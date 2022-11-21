---
title: "Making Change"
date: 2022-11-08
lastmod: 2022-11-21
---
# Making Change
![](https://i.imgur.com/9zfeU5i.png)

## Problem Formulation
> [!NOTE] Note that
> The minimum number of coins to form value x, is 1 + the minimum of the solutions to value $x-d_1, x-d_2....x-d_k$

We can form the following recurrence equations:
Base case (when n-d less than 0):
$$change(n)=0, \forall i\in\{1,...,m\},n-d_i < 0$$
Finding the minimum of subproblems:
$$change(n)=\min_{i=1...m,d_i\le n}(1+change(n-d_i))$$
## Strategy
![](https://i.imgur.com/6HMagN9.png)

## Pseudocode
```java
int coinchange (int n) {
	int change[n+1]; // initialize dp array
	for (int j=0; j<=n; j++) {
		
		change[j] = MAXINT // MAXINT is the maximum integer value
		
		for(int i=0;i<m;i++){
			if(j-val[i]>=0) {
				change[j] = min(change[j],change[j-val[i]]+1)
			}
		}
		
		if(change[j]== MAXINT) {
			change[j]=0; // If no changes means base case 0
		}
	}
	return change[n]
}
```

![](https://i.imgur.com/zzyfrLX.png)
