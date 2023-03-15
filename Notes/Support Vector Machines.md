---
title: "Support Vector Machines"
date: 2023-02-22
---
# Support Vector Machines
A very good explanation: https://www.youtube.com/watch?v=_PwhiWxHK8o
## Linear separation
A set of data points can be separated with a line (also called a *hyperplane*) with each group forming a class.
![](https://i.imgur.com/Xcs63xa.png)
Problem: there can be many acceptable solutions (*structural risk*)
### Reducing structural risk
If we add *margins*, we can restrict the possible lines we can choose from. 
![300](https://i.imgur.com/pIG5Ube.png)
By finding the *maximum* margin we can apply on the linear separator, we are finding the largest distance between the edges of the 2 classes. This gives it a better chance of classifying new data correctly (i.e. if the new data point is on 1 side of the separator, it is closer to one set of data points, this closeness is the intuition for why it should be classified as the same as this set of data points)
## What if we cannot find a linear separation?
![](https://i.imgur.com/0go7VFC.png)
By introducing more dimensionality into the data, the idea is that we can find some dimension in which a linear separation can be found.
![](https://i.imgur.com/4md60Gk.png)
## Kernels
How do we decide how to transform the data into higher dimensions? This is done through *kernel functions*. Kernels provide a possible way to increase the maximization of the margins we can find, but in exchange requires more computing resources.
1. Choose a suitable kernel function 
   ![300](https://i.imgur.com/sE7bl3L.png)
2. Compute $a$
3. Support vectors are the $x_i$ corresponding to $a_i \ne 0$. Support vectors with 1 class has indicator function output of 1 and for the other class the output is -1 (the margin of the street)
4. Classify the new data points via the indicator function. > 0 is positive class, < 0 is negative class
   ![](https://i.imgur.com/4BJ7HUx.png)
Rather than actually computing the data points transformed to higher dimensions, kernel functions only require the original data be used (observe the common kernels only involve dot products of the original data), saving lots of computation.

*What if it is already linearly separable? Are there any benefits for using the kernel?*
Can find a wider margin of separation which would generalize better.
## Slack
Although we can technically always find a separation, we might not want to do so as this could result in increased generalization. Allow some misclassifications or data to enter the margin to achieve slack.
![](https://i.imgur.com/hAKVxJ2.png)
- A support vector has $0<a_i<C$ and lies on the margins.
- A point between the boundary and the margin has $a=C$
- Points outside the margin have $a=0$ and are useless
## Advantages
- Guaranteed to find the global minima, unlike the learning in [Neural Networks](Notes/Neural%20Networks.md).