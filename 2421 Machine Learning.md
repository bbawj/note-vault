---
title: "2421 Machine Learning"
date: 2023-01-19
tags: [moc]
lastmod: 2023-03-13
---
# 2421 Machine Learning
A subfield of [AI](3005%20AI.md) focused on using algorithms trained on data to perform complex tasks.
#moc 
- [[Decision Trees]]
- [[Regression]]
- [Statistical Inference](Notes/Statistical%20Inference.md)
- [[Support Vector Machines]]
- [[Neural Networks]]
- [[Clustering]]
- [[Ensemble Learning]]
- [[Dimensionality Reduction]]
## Training and validation
Training a machine learning model involves the use of data. However, we need to test the effectiveness of the model, this is called validation. Hence we need to split the data into training and testing sets.
### Bias vs Variance
<iframe width="560" height="315" src="https://www.youtube.com/embed/EuBBz3bI-aA" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen></iframe>
Variance is the accuracy of the estimates compared to the actual values.
![](https://i.imgur.com/oZIN13H.png)
Our intuition may tell:  
- The presence of bias indicates something basically wrong with the model and algorithm...  
- Variance is also bad, but a model with high variance could at least predict well on average...  
So the model should minimize bias even at the expense of variance?? Not really!  
Bias and variance are equally important as we are always dealing with a single realization of the data set.
#### Overfitting
When the learned models are overly specialized for the training samples, leading to low bias and high variance.
![](https://i.imgur.com/ULGnGSl.png)
### Cross Validation
How do we know how much % to split between test and train-set data? Cross validation will attempt many different combinations to find the best split.
<iframe width="560" height="315" src="https://www.youtube.com/embed/fSytzGwwBVw" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen></iframe>
## Interpretability
### Shrinking the number of variables
Among a large number of variables the model there are generally many that have little (or no) effect on Y  
- Leaving these variables in the model makes it harder to see the big picture, i.e. the effect of the “important variables”  
- Would be easier to interpret the model by removing unimportant variables (setting the coefficients to zero)
### Occam's Razor
A principle about choosing the simplest explanation for the observed data, which can involve the number of model parameters, data points and fit to data.