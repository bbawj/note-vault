---
title:"Strategy Pattern"
---
# Strategy Pattern
#design-patterns 

## Problems we want to solve
1. A set of **interchangeable** algorithms or objects that can be decided at run-time
2. Extensible set of strategies: [Open-Closed Principle](Open-Closed%20Principle)

![](https://i.imgur.com/iL2Zc46.png)

![](https://upload.wikimedia.org/wikipedia/commons/4/45/W3sDesign_Strategy_Design_Pattern_UML.jpg)

Context refers or uses the Strategy interface for performing the algorithm. A and B classes implement the Strategy interface which gives the concrete algorithms.

## Pros
1. Encapsulation 
2. Hides implementation
3. Allows behaviour change at runtime

## Cons
1. Complexity if overused
