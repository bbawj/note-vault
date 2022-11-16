---
title:"Observer Pattern"
---
# Observer Pattern
#design-patterns

## Problems we want to solve
1. Tight coupling due to a 1-many dependency
2. We need a number of dependent objects to update automatically when one object changes state
3. We need an object to notify a number of other objects

## Push / Pull Mechanisms
**Pull
- 2-way communication: Subject sends a notification and the observer calls back for details explicitly (can be used for selective notification based on interest)
**Push
- 1-way communication: Subject sends the detailed information whether the observer wants it or not (can be used for sending updates based on location etc.)

![](https://i.imgur.com/DMskQSb.png)

![](https://i.imgur.com/KEO4uze.png)

## Example Class Diagrams
![](https://i.imgur.com/fkAbPLG.png)

![](https://i.imgur.com/70E2KU7.png)

![](https://i.imgur.com/6abQdzd.png)

![](https://i.imgur.com/Kd5pTsJ.png)

## Pros
1. Abstracts coupling between Subject and Observer
2. Supports broadcast communication
3. Enable reusability of subjects and observers

## Cons
1. Slower performance
2. Possible unnecessary complexity 
