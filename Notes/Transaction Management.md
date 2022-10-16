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
# ACID Properties
## Atomicity
A transaction is either performed in its entirety (can be in steps) or not performed at all.
### Logging
DBMS logs all actions so that it can undo the actions of aborted transactions
### Shadow paging
Make copies of pages, perform changes on these copies. Only when transaction commits, the page is made visible to others.
## Consistency
### Database consistency
A database is in consistent state if it obeys all of the consistency (integrity) constraints defined over it. A database may be inconsistent in between states.
### Transaction consistency
Database is in consistent state even if there are a number of concurrent transactions
## Isolation
A transaction should appear as though it is executed in isolation from other transactions. An executing transaction cannot reveal its results to other concurrent transactions before its commitment.
### Concurrency control protocol
#### Pessimistic
Do not let problems arise in the first place (prevention)
#### Optimistic
Assume conflicts are rare and deal with them when they happen (detection and recovery)
### Durability
Changes applied to the database by a committed transaction must persist in the database.
# Operations
![](https://i.imgur.com/O5iaevr.png)



