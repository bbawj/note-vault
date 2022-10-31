# Concurrency Control
The DBMS needs to ensure consistency during concurrent execution of transactions, as concurrency can result in the database being in an inconsistent state despite preserving the correctness of transactions and without encountering a failure.
## Scheduler
A schedule is a sequence of interleaved actions from all transactions.
### Serial schedule
A schedule is serial if its actions consists of all the actions of one transaction, then all the actions of another transaction, and so on.
![500](https://i.imgur.com/1K60gAt.png)
### Serializable schedule 
Result is equivalent to a serial schedule, but actions of one transaction do not have to occur before the actions of another.
![500](https://i.imgur.com/EvCDCZb.png)
### Conflict Serializable Schedule
![](https://i.imgur.com/sgLNvTW.png)
![](https://i.imgur.com/hpD2zat.png)
#### Precedence Graph
We can use a precedence graph to determine if a set of transactions are conflict serializable: 
![](https://i.imgur.com/01GqFEg.png)

![](https://i.imgur.com/PzNgNGH.png)

![](https://i.imgur.com/hqlUQN3.png)
