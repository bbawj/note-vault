---
title: "Race Condition"
date: 2022-11-08
lastmod: 2022-11-21
---
# Race Condition
Access to shared data from concurrent processes resulting in data inconsistency. This is due to [ context switches](Notes/Context%20Switch.md) between concurrent processes which result in non-sequential order of execution.

Take 2 processes producer and consumer:
![](https://i.imgur.com/QFgly25.png)
