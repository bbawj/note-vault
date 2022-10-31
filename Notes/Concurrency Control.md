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
## Locks
Ensure that data items that are shared by conflicting operations are accessed one operation at a time. Same as with [Process Synchronization](Notes/Process%20Synchronization.md). 
![](https://i.imgur.com/0IiEBMH.png)
![](https://i.imgur.com/gHcnbL4.png)
### Two Phase Locking (2PL)
Arbitrary assignment of locks do not lead to a serializable schedule. Two transactions can operate on elements in a different order resulting in different results. We can solve this by ensuring that transactions take up all lock actions before all unlock actions.
![](https://i.imgur.com/IVf95ET.png)
### Lock mechanisms
#### Shared and Exclusive locks
- Shared lock: to allow for multiple transactions to perform `READ`
- Exclusive lock: for `WRITE`
![](https://i.imgur.com/ap4gy0W.png)
A transaction should only ask for an exclusive lock when it is ready to write, so that any read operations can still continue. Upgrade the lock when needed:
![](https://i.imgur.com/3uWyyJ9.png)
#### Update locks
Deadlocks can occur when transactions are unable to upgrade their shared locks to exclusive ones, since there are already shared locks taken. A separate lock type that may be later upgraded to an exclusive lock is needed.
![](https://i.imgur.com/5zCmLxd.png)
#### Compatibility matrix
![](https://i.imgur.com/NqG3L6i.png)
### Workings of Scheduler
![](https://i.imgur.com/8zwE14N.png)
1. Part 1: Inserts appropriate lock actions ahead of all DB access operations and release the locks held by the Transaction when it aborts/commits
2. Part 2: maintains a waiting list of transactions that need to acquire locks

