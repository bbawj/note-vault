---
title: "NuSMV"
date: 2023-04-12
---
# NuSMV
## Vending Machine Example
What we want to model:
![](https://i.imgur.com/h4DxwCB.png)

![](https://i.imgur.com/VrqaUcj.png)
### Properties
![](https://i.imgur.com/QZsp5lw.png)
#### Iteration 1
Livelock where we make a choice and cancel:
![](https://i.imgur.com/Rfxgite.png)
#### Iteration 2
Deadlock where we make a payment when no choice is made
![](https://i.imgur.com/Vi9xJ32.png)
#### Iteration 3
*We want to accept payment only when choice is made*.
Introduce new variable `acc_payment` which is `TRUE` only if expect payment and payment are true. 
![](https://i.imgur.com/UyFFa8y.png)
Deadlock because `acc_payment` can be set to `TRUE` even when we cancel our choice:
![](https://i.imgur.com/qBabm5n.png)
## Pitfalls
Case statement must be exhaustive:
![](https://i.imgur.com/XOKPCPD.png)
## Modules
- The `main` module instantiates other modules.
- Each module runs in parallel
- Share state using parameters
![](https://i.imgur.com/Zf7geTg.png)
