---
title: "Decision Trees"
date: 2023-02-09
lastmod: 2023-03-16
---
# Decision Trees
A decision tree is an analysis strategy by asking questions about the target sequentially
![](https://i.imgur.com/IIpEmTq.png)
This type of logical expression is easiest for a decision tree to learn.
## Growing the Tree
1. Choose the best question (measured base on information gain) and split the input data into subsets
2. Terminate when a unique class label is formed (no need for further questions)
3. Grow by recursively extending other branches
### Entropy (measuring information gain)
- [Entropy](Notes/Information%20Theory.md#Entropy)
- [Gini Impurity](Notes/Information%20Theory.md#Gini%20Impurity)
### Choosing attributes
![](https://i.imgur.com/xjafN3v.png)

![](https://i.imgur.com/CmjFdrh.png)
![500](https://i.imgur.com/LucM00J.png)
## Avoid overfitting
- Stop growing when data split not statistically significant
- Grow full tree, then post-prune (e.g. Reduced error pruning)