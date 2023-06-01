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
## Operators
### next()
![](https://i.imgur.com/wJamRXj.png)
## Ferryman Problem Example
- Ferryman wants to cross the river with cabbage, goat and wolf
- Goat will eat cabbage if left alone
- Wolf will eat goat if left alone
![](https://i.imgur.com/kWTgMM5.png)

![](https://i.imgur.com/DYJ6Qny.png)

![](https://i.imgur.com/5cSezFY.png)
We can find the solution by negative the property we want to hold, *reach the solution while property is false*. Model checker can then find a counter example where we reach the solution while property is true:
![](https://i.imgur.com/63KShXB.png)
## Bridge and Torch Problem
![](https://i.imgur.com/KF6fRey.png)

![](https://i.imgur.com/rIV6lf0.png)
### Model
- Location of A,B,C,D as array of booleans
- Travelling status of ABCD as another array of booleans
- Torch location as boolean
- Time as a number 0-100
### Transitions
- Torch can change location only if someone travels.
- Model that torch always changes location until solution achieved, since we are interested in efficient solutions, and time does not increase if nobody moves.
- Location is updated iff they want to travel and the torch is at their place
- Timekeeping
	- Time advances according to the slowest person who travels
	- Define a time limit which the problem must be solved
## Modelling Concurrent Programs
![](https://i.imgur.com/vA10931.png)

![](https://i.imgur.com/GQVfAPs.png)
### Ensuring fairness
`running` is a property created by the process keyword internally which indicates which process is running.

Process 2 is always selected by this model "scheduler". This is unfair trace which can be forbidden using the keyword `FAIRNESS running;`
![](https://i.imgur.com/BX0dAGp.png)
### No bounded waiting
Process 1 and 2 alternate between each other but process 2 acquires the semaphore each time before process 1 is able to do so.
![](https://i.imgur.com/uIRAPv6.png)



