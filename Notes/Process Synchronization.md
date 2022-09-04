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
2. P0 runs in a long remainder section
3. Context switch occurs, P1 needs to enter critical section but P0 is stuck running remainder for a long period and does not return the turn back to P1
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
```
while(1){
	int processId = 1
	originalVal = swapMemAndReg(processId)
	while (registerVal != processId);  //entry section
	critical section...
	swapMemAndReg(originalVal) // exit section
	remainder section...
}
```
