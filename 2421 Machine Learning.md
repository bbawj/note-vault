---
title: "2421 Machine Learning"
date: 2023-01-19
tags: [moc]
lastmod: 2023-01-19
---
# 2421 Machine Learning
#moc 
- [[Decision Trees]]

## Training and validation
Training a machine learning model involves the use of data. However, we need to test the effectiveness of the model, this is called validation. Hence we need to split the data into training and testing sets.
### Overfitting
When the learned models are overly specialized for the training  
samples, leading to low bias and high variance.
<iframe width="560" height="315" src="https://www.youtube.com/embed/EuBBz3bI-aA" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen></iframe>
![](https://i.imgur.com/oZIN13H.png)
Our intuition may tell:  
- The presence of bias indicates something basically wrong with the model and algorithm...  
- Variance is also bad, but a model with high variance could at least predict well on average...  
So the model should minimize bias even at the expense of variance?? Not really!  
Bias and variance are equally important as we are always dealing with a single realization of the data set.
### Cross Validation
How do we know how much % to split between test and train-set data? Cross validation will attempt many different combinations to find the best split.
<iframe width="560" height="315" src="https://www.youtube.com/embed/fSytzGwwBVw" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen></iframe>

