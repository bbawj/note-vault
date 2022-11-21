---
title: "Probabilistic Analysis and Randomised Algorithms"
date: 2022-11-08
lastmod: 2022-11-21
---
# Probabilistic Analysis and Randomised Algorithms
## Exercises
### 5.1.2
Describe an implementation of the procedure RANDOM(a, b) that only makes calls to RANDOM(0, 1). What is the expected running time of your procedure, as a function of a and b?
```
Random(a,b):
	left = a
	right = b
	while left < right:
		middle = (left+right)/2
		choice = Random(0,1)
		if choice == 0:
			right = middle - 1
		else:
			left = middle + 1
	return left
```
$O(log_2(b-a))$
### 5.2.1
Probability of hiring 1 time occurs when the best candidate is presented first. This occurs with probability$\frac{1}{n}$. Probability of hiring n times occurs when the order is strictly increasing. This is one order out of $n!$ orders, $\frac{1}{n!}$.
### 5.2.2
You hire twice when you first hire is the candidate with rank i and all the candidates with rank k >i come after the candidate with rank n. There are $n - i$ better suited candidates and the probability of the best one coming first is $1/(n-i)$. Let $T_i$ be the indicator variable for the event that the ith candidate is hired twice. $$Pr(T)=\sum_{i=1}^{n-1}Pr(T_i)=\sum_{i=1}^{n-1}\frac{1}{n\times (n-i)}=\frac{1}{n}log(n-1)+O(1)$$
### 5.2.5
Let $X_i$ be the indicator random variable for the event which the ith customer gets back his own hat. $Pr(T_i)=1/n$ since the hats are given back in random order, each customer has an independent 1/n chance of getting back his own hat. $$E[T]=E{\sum_{i=1}^n T_i }=E(n\times1/n)=1$$
