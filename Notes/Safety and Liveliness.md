---
title: "Safety and Liveliness"
date: 2023-04-11
lastmod: 2023-04-11
---
# Safety and Liveliness
Correctness properties which are common in computer science.
## Safety
Properties that state that nothing bad ever happens. It can only be:
- satisfied in infinite time (you cannot be sure you are safe)
- violated in finite time (when bad happens)
![](https://i.imgur.com/Ym9AsqW.png)
The **prefix** of a trace T is the first k (for k ≥ 0) events of T  
- cut off the tail of T  
- finite beginning of T  
An **extension** of a prefix P is any trace that has P as a prefix
>[!Formal definition]
> A property P is a safety property if given any execution E such that P(trace(E)) = false, there exists a prefix of E, s.t. every extension of that prefix gives an execution F s.t. P(trace(F))=false

![500](https://i.imgur.com/9w2eSWC.png)
## Liveliness
Properties that state that something good eventually happens. It can only be:
- satisfied in finite time (when good happens)
- violated in infinite time (there is always hope)
>[!Formal definition]
>A property P is a liveness property if given any prefix F of an execution E, there exists an extension of trace(F) for which P is true

![500](https://i.imgur.com/gZEIagM.png)