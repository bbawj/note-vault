---
title: "Dimensionality Reduction"
date: 2023-03-12
---
# Dimensionality Reduction
## Principle Component Analysis
A method for dimensionality reduction, by finding the variables which most account for the variance in the data.
### A 2D example
Plot the data, with each axis being one of the variables. 
![](https://i.imgur.com/bDAPXmM.png)
Center the plot around the origin. Find the best fitting line which passes through the data. The best fit is that which minimizes the sum of the distances from data to line. By the Pythagoras theorem, it is also the one which maximises the distance from origin to projected data.
![](https://i.imgur.com/ugrrtr9.png)
The best fit line is Principle Component 1 (PC1).
![](https://i.imgur.com/g7kBiCK.png)
Since this is a 2d example, PC2 is now simply the line perpendicular to PC1. Why? Idk...
![](https://i.imgur.com/Ov7SNcL.png)
We can then remove variables which account for less of the variation in the data.
## Linear Discriminant Analysis
LDA is a method of dimensionality reduction, by finding a new axis (or set of axes) which maximizes the separation amongst the categories in data.
![](https://i.imgur.com/wWqaMh8.png)
When we have n-dimensions of data, LDA allows us to find the axes which can separate the categories the best.
![](https://i.imgur.com/UmXOi4O.png)
## Subspace Methods
Samples in the same class are similar to each other. We can think of them as localized in a subspace spanned by a set of basis vectors. If we project the new test data onto this subspace, we can find the similarity of it to the class.
![](https://i.imgur.com/anKDVuc.png)
One method to find similarity is to choose the subspace which maximizes the projection length:
![](https://i.imgur.com/O1xgDGd.png)
