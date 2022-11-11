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
### Perterson's Solution
![](https://i.imgur.com/TaegYbO.png)
Mutual exclusion:
- Pi enters its critical section only if either flag[j] == false or turn == i. Also note that, if both processes can be executing in their critical sections at the same time, then flag[0] == flag[1] == true. These two observations imply that P0 and P1 could not have successfully executed their while statements at about the same time, since the value of turn can be either 0 or 1 but cannot be both. Hence, one of the processes—say, Pj —must have successfully executed the while statement, whereas Pi had to execute at least one additional statement (“turn == j”). However, at that time, flag[j] == true and turn == j, and this condition will persist as long as Pj is in its critical section; as a result, mutual exclusion is preserved.
Progress:
- Pi can be prevented from entering the critical section only if it is stuck in the while loop with the condition flag[j] == true and turn == j
- If Pj is not ready to enter the critical section, then flag[j] == false, and Pi can enter its critical section. If Pj has set flag[j] to true and is also executing in its while statement, then either turn == i or turn == j. If turn == i, then Pi will enter the critical section. If turn == j, then Pj will enter the critical section.
Bounded Waiting:
- When $P_i$ exits its critical section, flag[j] == false and $P_i$ is allowed to enter its critical section. Assume $P_j$ resets flag[j] == true, it will subsequently set turn == i. Since $P_i$ cannot change the turn value while in the loop, it is allowed to enter the critical section after at most 1 entry by $P_j$
## Hardware Solution
### Synchronization Hardware
Race condition is a result of context switches. We can prevent that in hardware to have atomic instructions.
![](https://i.imgur.com/gJrBVWr.png)
*Difficult to control the disabling of context switches in user level as there may be many critical regions and regions execute for unknown amounts of time*
TestAndSet is now an assembly instruction which can be used to acquire a lock:
![](https://i.imgur.com/ML23PqH.png)
- No context switches can occur while setting the lock value
- This means that whoever runs this instruction first will run first, no other process will be able to enter critical region
- If lock == true, someone is in the critical section: we are blocked
- If lock == false, we set lock to true and move into the critical section
![](https://i.imgur.com/zwFGrJq.png)
Hardware has no memory of process trying to access the lock. P0 able to indefinitely take the lock without giving P1 a chance.
## Operating System Solution
### Mutex Locks
![](https://i.imgur.com/pLZl78y.png)
### Semaphore
![](https://i.imgur.com/NFRVyYv.png)
- A binary sempahore behaves similar to mutex locks.
- A counting semaphore is used to control access to a given resource consisting of a finite number of instances. **It is initalised to the number of resources available**.
#### Busy waiting *solution*
Also known as a *spinlock*, where a thread trying to acquire a lock is caused to wait in a loop ("spin") while repeatedly checking if the lock is available.
![](https://i.imgur.com/aPAwIXL.png)
Atomicity is not possible for this solution on a single-core. If a process P0 must loop to execute `Wait(S)`, no other processes can execute `Signal(S)` in order to allow P0 to continue. If we cannot context switch then there is no solution.
#### Blocking Solution
![](https://i.imgur.com/tiVca7I.png)

![](https://i.imgur.com/IgHj0f5.png)
- Process is in the waiting state because the process cannot use the CPU (and following which enter its critical section) if another process is currently in its critical section
- Process woken up is changed to ready state but the CPU may not switch from the currently running process to this newly ready process depending on scheduling
![](https://i.imgur.com/Mc1Ihj1.png)
Atomicity needed for these system calls:
![](https://i.imgur.com/YRNgQVD.png)
## Classical Problems of Synchronization
### Bounded Buffer
![](https://i.imgur.com/Y7Jf4tR.png)
![](https://i.imgur.com/1R40zE1.png)
The order of access to the shared variables matter. Logically, each process should check if the resource is available (in the case of consumer) OR if there is currently no instances (for producers), before trying to access the buffer through the mutex lock.
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
b. 
- Mx not satisfied: 
	- P1: turn = 0, while(flag and turn == 0) *flag is false*, critical section 
	- context switch
	- P2: flag = true, while(turn == 1), critical section
- Progress is not satisfied. P1 can only run if flag = false but flag is only set to false after the critical section of P1.
![](https://i.imgur.com/9ZAwLZl.png)
a. False. If all instructions can complete in 1 cycle, there will not be instructions being executed halfway before context switch occurs. There are also other reasons for race condition, not only due to translation. If implementation using a temporary variable, a race condition can also occur.
b. True. If no context switch can occur during critical section, only 1 process will be in its critical section at one time.
c. False. Satisfies progress only means that 1 program will always be chosen to enter critical section. To satisfy bounded waiting, each program must have a chance to enter its critical section. A program where only 1 process always enter critical section while another is waiting indefinitely satisfies progress but not bounded waiting.
![](https://i.imgur.com/rpsqnqG.png)
hmm not too sure abt this question
idea:
1. use a boolean lock value initialized to false as a shared variable, and a register boolean as a flag
2. continuously try to swap `true` into the lock
3. if register becomes false, we got the lock
4. critical section
5. set the lock to false 
```go
while(1){
	register = true
	while (register) {
		swap(&lock, register)
	}  //entry section
	critical section...
	lock = false
	remainder section...
}
```
![](https://i.imgur.com/lNZUTeX.png)
a. -4
b. -6. All `Wait(S)` runs before a single `Signal(S)`. Each process is added to blocked queue until OS chooses to execute a critical region.
c. 2. There can be at most 2 processes holding S simultaneously as 2 processes are able to complete `wait(S)` (*not blocked*) before the block list starts to increase.
![](https://i.imgur.com/Mu27bAq.png)
To ensure there is no deadlock, there should not be any nesting i.e. two semaphores are not acquired together:
```go
wait(A)
apple++
signal(A)
wait(O)
if (at least 2 oranges in basket){
	oranges += 2
} else {
	signal(O)
	wait(A)
	apple--	
	signal(A)
}
```
![](https://i.imgur.com/wHtLNOo.png)
Identify critical section for each function and use TestAndSet to acquire lock into the section:
![](https://i.imgur.com/mEBJuch.png)

```go
Wait(S){
	while (TestAndSet(&lock));
	S.value--
	if (S.value < 0) {
		S.L = append(S.L, process)
		*lock = false
		sleep(process)
	} else {
		*lock = false
	}
}

Signal(S) {
	while (TestAndSet(&lock));
	S.value++
	if (S.value <= 0) {
		p = S.L[0]
		*lock = false
		//add to ready queue
	} else {
		*lock = false
	}
}
```