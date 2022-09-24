# Real Time OS
Systems whose correctness depends not only on logical aspects but also on the temporal aspects i.e. able to meet specific deadlines.
## Real Time Process
![](https://i.imgur.com/yvgNQ66.png)
Notice that C is often impossible to determine except for small specific applications.
### Recurrent RTOS process
Usually executes some function or goes through a set of steps in a regular manner repeated over time. e.g. Collect data from sensors, execute control laws, send actuator commands
### Periodic RTOS Process
![](https://i.imgur.com/5j6oIYb.png)
### Sporadic RTOS Process
![](https://i.imgur.com/W8FOywe.png)
## Real Time Process Scheduling
[[Notes/Process scheduling|Classical scheduling algorithms]] will fail to schedule RTOS processes as they do not take into account the deadlines of the processes.
### Fixed priority scheduling
Priorities are fixed across instances of recurrent processes. Easy to implement with low time complexity. If all types of processes are known and small, a [[Notes/Hash Tables|hash map]] is able to do this in constant time.
#### Rate Monotonic Scheduler
![](https://i.imgur.com/gWXCXDK.png)
Since the scheduler is based on periodic time, it still does not prioritise based on deadlines. P3 misses the deadline if it is <20,2,14>: less time to complete the process in first appearance.
#### Deadline Monotonic Scheduler
A natural solution to RM scheduler:
![](https://i.imgur.com/IhxEkPa.png)
### Dynamic Priority Scheduling
#### Earliest Deadline First Scheduler
Priority is based on the instance level rather than the process level.
![](https://i.imgur.com/CCILxlc.png)
## Comparisons
![](https://i.imgur.com/TNHIVzZ.png)

![](https://i.imgur.com/sQZTbLD.png)
