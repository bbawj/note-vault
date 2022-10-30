# Transaction Management
Transactions are the basic unit of change in a DBMS. It is essential for data recovery and concurrency control.
>[!Idea:]
>1. Take a database as an input
>2. Perform an action
>3. Generate new version of database
## SQL Transactions
- A new transaction starts with `BEGIN`
- Transactions are stopped with either a `COMMIT` or `ABORT`
- `COMMIT`: changes are saved
- `ABORT`: changes are undone
![](https://i.imgur.com/rc3m2AF.png)
## ACID Properties
###  Atomicity
A transaction is either performed in its entirety (can be in steps) or not performed at all.
#### Logging
DBMS logs all actions so that it can undo the actions of aborted transactions
#### Shadow paging
Make copies of pages, perform changes on these copies. Only when transaction commits, the page is made visible to others.
### Consistency
#### Database consistency
A database is in consistent state if it obeys all of the consistency (integrity) constraints defined over it. A database may be inconsistent in between states.
#### Transaction consistency
Database is in consistent state even if there are a number of concurrent transactions
### Isolation
A transaction should appear as though it is executed in isolation from other transactions. An executing transaction cannot reveal its results to other concurrent transactions before its commitment.
#### Concurrency control protocol
- Pessimistic: Do not let problems arise in the first place (prevention)
- Optimistic: Assume conflicts are rare and deal with them when they happen (detection and recovery)
### Durability
Changes applied to the database by a committed transaction must persist in the database.
## Primitive Operations
![](https://i.imgur.com/O5iaevr.png)
## Failures
### Types of Failures
#### Transaction failures
- Logical Errors: Transaction cannot complete due to some internal error condition (e.g., integrity constraint violation).
- Internal State Errors: DBMS must terminate an active transaction due to an error condition (e.g., deadlock).
#### System failures
- Software Failure: Problem with the DBMS implementation (e.g., uncaught divide-by-zero exception).
- Hardware Failure: The computer hosting the DBMS crashes (e.g., power plug gets pulled, disk crash). They can be recoverable or non-recoverable
## Recovery Algorithms
Recovery must kick in to ensure the database satisfies the ACID properties in the case of failure. They must carry in 2 parts:
1. Actions during normal transaction processing to ensure that the DBMS can recover from a failure.
2. Actions after a failure to recover to a state that ensures atomicity, consistency and durability

> Ensuring atomicity and durability -- Logging:
> The database stores additional files called *log files*. Logs record every action of each transaction and are append only.
### Undo Logging
Idea: undo the effects of transactions that may not have completed before failure.
![](https://i.imgur.com/805J1ia.png)
![](https://i.imgur.com/TECxSmC.png)
> [!Rules]
> With undo logging, there are a set of rules the DBMS must follow. Order of writing to disk:
> 1. Log records indicating changed database elements
> 2. Changed database elements themselves
> 3. COMMIT log record

![](https://i.imgur.com/9i2OUpZ.png)
All undo commands are idempotent, if failure occurs during recovery, we can simply restart.
#### Checkpointing
Use periodic checkpoints to prevent having to read the entirety of the log file for recovery. Any transactions executed before the checkpoint will have finished and there will be no need to undo them.
![](https://i.imgur.com/Z54jfvB.png)
Problem
- The database is frozen while performing checkpointing. Active transactions may take a long time to commit or abort and will result in variant performance of the DBMS.
#### Non-quiescent Checkpointing
Start checkpointing at any time using the current incomplete transactions.
![](https://i.imgur.com/3uaHcFg.png)
#### Limitations
Cannot commit a transaction without first writing all its changed data to disk. This means we will need many disk I/O for each transaction.
### Redo Logging
Do not write all changed data to disk before committing. Write to the main memory log file all the changes to the DB, and commit before any changes are written to disk. Perform recovery by redoing effects of committed transactions before the crash.
![](https://i.imgur.com/Zhule0H.png)
> [!Rules]
> If a transaction modifies X, then both <T,x,v> and COMMIT T must be written to disk before OUTPUT(X). Order of writing to disk:
> 1. Log records indicating the changed elements
> 2. COMMIT T log record
> 3. Changes to the elements themselves

![](https://i.imgur.com/WuYLOdQ.png)
![](https://i.imgur.com/RMa262e.png)
#### Checkpointing
Start checkpointing for all active transactions. When we see an `START CHECKPOINT`, we can be sure that all prior transactions have been recorded to the disk and there is no need to redo them. 
*Example where we do not need to redo the actions for T1:*
![](https://i.imgur.com/MCReHYm.png)
#### Limitations
It requires all modified blocks to be kept in buffers until the transaction commits and the log records have been flushed. This increases the average number of buffers needed by transactions.
### Undo/Redo Logging
Increase flexibility by maintaining more information on the log. <T,X,v,w> now stores both old and new values in the log. Before modifying any DB element X, this log record must be written to the disk.
![](https://i.imgur.com/3BiSNNV.png)
We can commit at any time after the <T,B,8,6> record has been written to the disk. 
Recovery process:
1. Redo all committed transactions top-down
2. Undo all uncommitted transactions bottom-up
This is because we may have committed transactions with some of the changes not on disk as well as uncommitted transactions with some of the changes on disk.
#### Checkpointing
![](https://i.imgur.com/WBdr1xt.png)
We are sure we do not need to check any further than the `START CHECKPOINT` as the corresponding `END CHECKPOINT` ensures that all changes prior have been flushed to the disk.



