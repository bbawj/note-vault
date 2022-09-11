# Process Synchronization
[[Race Condition]]

## Critical Section Problem
One method to solve the race condition is to divide processes into critical sections which are segments that shared data is accessed. **One process must be writing.**

Problem: design protocol to ensure that no 2 processes are executing their critical section at the same time.

We need to satisfy 3 properties:

1. **Mutual exclusion**: if process is executing in critical section, no other process can be executing in its critical section at the same time.
	*Why is mutual exclusion not enough? 
	- It can be achieved naively by preventing any process from entering critical section
2. **Progress**: if no process is executing in its critical section and another process needs to enter their critical section, selection of this process to enter cannot be postponed indefinitely
3. **Bounded waiting**: if a process needs to enter their critical section, all other processes are allowed to enter their own critical section only a bounded number of times
## User-level Solutions
Following examples show how it is possible for 2 processes. *For more processes, it becomes unfeasible*
### Turn variable
![](https://i.imgur.com/K9rspxV.png)
Progress is violated:
1. P1 finish critical section and pass the turn over to P0
2. P0 finish its critical section and pass the turn over to P1
3. P1 runs in a long remainder section and never passes turn back to P0
4. Context switch occurs, P0 needs to enter critical section but P1 is stuck running remainder for a long period 
![](https://i.imgur.com/2jrBrZ2.png)
### Flag variable
![](https://i.imgur.com/wU8koJx.png)

![](https://i.imgur.com/YPYh9dL.png)
### Combination
![](https://i.imgur.com/TaegYbO.png)
## Hardware Solution
### Synchronization Hardware
Race condition is a result of context switches. We can prevent that in hardware to have atomic instructions.
![](https://i.imgur.com/gJrBVWr.png)
*Difficult to control the disabling of context switches in user level as there may be many critical regions and regions execute for unknown amounts of time*
TestAndSet is now an assembly instruction which can be used to acquire a lock:
![](https://i.imgur.com/ML23PqH.png)
- No context switches can occur while setting the lock value
- This means that whoever runs this instruction first will run first, no other process will be able to enter critical region
![](https://i.imgur.com/zwFGrJq.png)
Hardware has no memory of process trying to access the lock. P0 able to indefinitely take the lock without giving P1 a chance.
## Operating System Solution
### Semaphore
![](https://i.imgur.com/NFRVyYv.png)
#### Busy waiting *solution*
Atomicity is not possible for this solution on a single-core. If a process P0 must loop to execute `Wait(S)`, no other processes can execute `Signal(S)` in order to allow P0 to continue. If we cannot context switch then there is no solution.
![](https://i.imgur.com/aPAwIXL.png)
#### Blocking Solution
![](https://i.imgur.com/tiVca7I.png)

![](https://i.imgur.com/IgHj0f5.png)
- Process is in the waiting state because the process cannot use the CPU (and following which enter its critical section) if another process is currently in its critical section
![](https://i.imgur.com/Mc1Ihj1.png)
Atomicity need for these system calls:
![](https://i.imgur.com/YRNgQVD.png)
## Classical Problems of Synchronization
### Bounded Buffer
![](https://i.imgur.com/Y7Jf4tR.png)
![](https://i.imgur.com/1R40zE1.png)
Producer:
```go
do 
{
    // wait until empty > 0 and then decrement 'empty'
    wait(empty);   
    // acquire lock
    wait(mutex);  
    
    /* perform the insert operation in a slot */
    
    // release lock
    signal(mutex);  
    // increment 'full'
    signal(full);   
} 
while(TRUE)
```
Consumer:
```go
do 
{
    // wait until full > 0 and then decrement 'full'
    wait(full);
    // acquire the lock
    wait(mutex);  
    
    /* perform the remove operation in a slot */ 
    
    // release the lock
    signal(mutex); 
    // increment 'empty'
    signal(empty); 
} 
while(TRUE);
```
### Dining Philosophers
![](https://i.imgur.com/xeUVrNj.png)

![](https://i.imgur.com/zTf3nWu.png)
- If each process executes the first `wait(chopstick)` and context switches, every process only has 1 chopstick and is stuck in a deadlock
Solutions:
![](https://i.imgur.com/q9Gp6b0.png)
### Readers-Writers
![](https://i.imgur.com/mobJGtN.png)

![](https://i.imgur.com/23qQMhV.png)
Writer:
```go
wait(wrt);
//write
signal(wrt);
```
## Practice Problems
![](https://i.imgur.com/APBjxa2.png)
a. 
1. Mutual exclusion
2. Progression
3. Bounded waiting
b. Progress is not satisfied. P1 can only run if flag = false but flag is only set to false after the critical section of P1.
![](https://i.imgur.com/9ZAwLZl.png)
a. True. If all instructions can complete in 1 cycle, there will not be instructions being executed halfway before context switch occurs.
b. True. If no context switch can occur during critical section, only 1 process will be in its critical section at one time.
c. False. Satisfies progress only means that 1 program will always be chosen to enter critical section. To satisfy bounded waiting, each program must have a chance to enter its critical section. A program where only 1 process always enter critical section while another is waiting indefinitely satisfies progress but not bounded waiting.
![](https://i.imgur.com/rpsqnqG.png)
hmm not too sure abt this question
idea:
1. Initialize a process flag to be true
2. Entry: Swap the flag with register
3. Poll the register value to see if flag is true
4. If true, we successfully acquired the lock (the first process that manages to run step 2 will get the lock)
5. Exit: Swap back the original register value and flag
```go
while(1){
	int processId = 1
	originalVal = swapMemAndReg(processId)
	while (registerVal != processId);  //entry section
	critical section...
	swapMemAndReg(originalVal) // exit section
	remainder section...
}
```
![](https://i.imgur.com/lNZUTeX.png)
a. -4
b. -6. All `Wait(S)` runs before a single `Signal(S)`. Each process is added to blocked queue until OS chooses to execute a critical region.
c. 1. Every process has decremented S before able to increment S. When S is 1, it means that the process executes wait, S -> 0, able to enter critical region without being blocked and executes Signal: S->1. 
![](https://i.imgur.com/Mu27bAq.png)
```go
wait(A)
apple++
wait(O)
if (at least 2 oranges in basket){
	oranges += 2
	signal(O)
	signal(A)
} else {
	apple--	
	signal(O)
	signal(A)
}
```
![](https://i.imgur.com/wHtLNOo.png)
not too sure about this:
```go
Wait(S){
	TestAndSet(S.value)	
	TestAndSet(S.process)	
}
```