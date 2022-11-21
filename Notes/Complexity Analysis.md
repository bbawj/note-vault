---
title: "Complexity Analysis"
date: 2022-11-08
lastmod: 2022-11-21
---
# Complexity Analysis
## Asymptotic Notations
Notations used to describe the order of growth of a given function
### Big-O $O(f(x))$
![](https://i.imgur.com/KUrXTHz.png)

The limits when taking the 2 functions to infinity produces a constant C that 
$$\begin{align}\lim_{n\to \infty}\frac{f(n)}{g(n)}=C \\ C=0 \ or\ 0<C<\infty \end{align}$$
### Big-Omega $\Omega(f(x))$
![](https://i.imgur.com/SpdpFpF.png)

The limits when taking the 2 functions to infinity produces a constant C that 
$$\begin{align}\lim_{n\to \infty}\frac{f(n)}{g(n)}=C \\ C=\infty \ or\ 0<C<\infty \end{align}$$
### Big-Theta $\theta(f(x))$
![](https://i.imgur.com/c5JWUJ8.png)

The limits when taking the 2 functions to infinity produces a constant C that 
$$\begin{align}\lim_{n\to \infty}\frac{f(n)}{g(n)}=C \\ 0<C<\infty \end{align}$$

### Properties
![](https://i.imgur.com/En4IxYe.png)

$$f(n)\in O(h(n)),g(n)\in O(h(n)) \implies f(n)+g(n)\in O(h(n))$$
![](https://i.imgur.com/h7PQDHK.png)

### Example function comparisons
![](https://i.imgur.com/1Ngf1uk.png)

### Order of Common Functions
![](https://i.imgur.com/LurnRaE.png)

## Specific Complexities
- [Polynomial Time Complexity](Notes/Polynomial%20Time%20Complexity.md)
- [Pseudo-Polynomial Time Complexity](Notes/Pseudo-Polynomial%20Time%20Complexity.md)
