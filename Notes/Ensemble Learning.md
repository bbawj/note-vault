---
title: "Ensemble Learning"
date: 2023-03-02
---
# Ensemble Learning
The idea behind ensemble learning is to combine independent and diverse classifiers with the hopes of obtaining better predictions.
## Bootstrap Aggregating (Bagging)
Use replicates of the training set by sampling with replacement to train each model. Combine $B$ such models together, by running the test data on each replicate. The classification which received the most "votes" from the replicates is the decided value.
![500](https://i.imgur.com/vslHrD5.png)
## Random Forest
For use in [Decision Trees](Notes/Decision%20Trees.md). Bagging + random feature selection (randomly select a feature to split a node) for every node.
### Estimating accuracy
Out-of-bag error: About 1/3 of the training set is not used in bagging. These data points can be used to test the accuracy of the random forest.
## Boosting
Use the data to train a set of weak learners/classifiers. Combine them to create a single strong classifier.
### Adaboost
Let the sample data $S = \{(x_1,y_1),...(x_m,y_m)\}$
1. Each sample in the train data is given the same weight: $w_i = \frac{1}{m}$ 
2. Train a weak classifier (for example a *stump* which is a decision tree with only 1 fork) using S and their weights. Choose the weak classifier $h$ which minimizes the training error.
3. Compute the **reliability coefficient** $a=log_e(\frac{1-error}{error})$ for the chosen weak classifier. This can be seen as the *amount of say for this classifier*. 
4. Use the reliability to scale the weights of each training sample. $w_{t+1}=w_{t}\times e^{-a_tyh_t(x)}$. A misclassification causes the weight to be increased and vice versa.
5. Place more emphasis on correctly classifying the higher weighted sample, e.g. by randomly sampling based on a probability distribution using the weights (higher weight means more likely to be chosen).

