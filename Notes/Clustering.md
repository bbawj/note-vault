---
title: "Clustering"
date: 2023-03-02
---
# Clustering
A form of unsupervised learning. Rather than having prior knowledge of the classes which the data can be in, we want the machine to infer the possible groupings/classes from unlabelled data:
![](https://i.imgur.com/MRwM0VL.png)
## K-means clustering
![](https://i.imgur.com/6UJziFi.png)
- Using the Euclidean distance as the minimising function causes it to favour spherical clusters which may not be the case in linear separated data points: ![](https://i.imgur.com/i4TpDhO.png)
## Expectation Maximisation
Rather than minimizing the Euclidean distances of the data points to the cluster, try to maximize the probability of the data point being generated from the cluster.
### Step 1
Introduce a hidden variable $h$ for each desired cluster and initialize the starting parameters for each cluster distribution
### Step 2
![](https://i.imgur.com/oHXtwAL.png)
### Step 3
Update the values of the model parameters iteratively:
![](https://i.imgur.com/l32I2Lm.png)

![350](https://i.imgur.com/5zlL6wB.png)
### Comparison to K-means
![](https://i.imgur.com/kOUHkGk.png)
