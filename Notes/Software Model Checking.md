---
title: "Software Model Checking"
date: 2023-04-28
lastmod: 2023-04-29
---
# Software Model Checking
[Model Checking](Notes/Model%20Checking.md) is traditionally applied to protocols/algorithms and specifications. Software model checking tries to find failures in well, software.

Software model checking tools can backtrack and explore different [thread schedules](Notes/Threads.md) for a concurrent program and in the process find all possible failures.
## Java Pathfinder
[Java Pathfinder](https://github.com/javapathfinder/jpf-core) is a software model checker for Java bytecode. 

An example with the [Dining Philosophers problem](Notes/Process%20Synchronization.md#Dining%20Philosophers):
| Thread Name/ Trans | main                                   | thread 1                           | thread 2                           | thread 3                           | thread  4                          | thread 5                           |
| ------------------ | -------------------------------------- | ---------------------------------- | ---------------------------------- | ---------------------------------- | ---------------------------------- | ---------------------------------- |
| 0                  | create Forks in loop                   |                                    |                                    |                                    |                                    |                                    |
| 1-12               | create Philosophers and launch threads |                                    |                                    |                                    |                                    |                                    |
| 13-14              |                                        | obtain lock 1c4; try to obtain 1c5 |                                    |                                    |                                    |                                    |
| 15-16              |                                        |                                    | obtain lock 1c5; try to obtain 1c6 |                                    |                                    |                                    |
| 17-18              |                                        |                                    |                                    | obtain lock 1c6; try to obtain 1c7 |                                    |                                    |
| 19-20              |                                        |                                    |                                    |                                    | obtain lock 1c7; try to obtain 1c8 |                                    |
| 21-22              |                                        |                                    |                                    |                                    |                                    | obtain lock 1c8; try to obtain 1c4 |

![300](Pics/Software%20Model%20Checking%202023-04-29%2015.44.45.excalidraw.svg)
%%[🖋 Edit in Excalidraw](Pics/Software%20Model%20Checking%202023-04-29%2015.44.45.excalidraw.md), and the [dark exported image](Pics/Software%20Model%20Checking%202023-04-29%2015.44.45.excalidraw.dark.svg)%%

| Thread Name/ Trans | main                                 | thread 1                               | thread 2 |
| ------------------ | ------------------------------------ | -------------------------------------- | -------- |
| 1-3                | creates workers and starts 2 threads |                                        |          |
| 4                  |                                      | obtains lock on Queue                  |          |
| 5                  |                                      | puts data into Queue                   |          |
| 6                  |                                      | tries to remove data but enters wait() |          |
| 7                  |                                      |                                        | tries to put data but queue is full so wait()|

| Thread Name/ Trans | main                                      | thread 1                        | thread 2                     | thread 3                                         | thread 4                     |
| ------------------ | ----------------------------------------- | ------------------------------- | ---------------------------- | ------------------------------------------------ | ---------------------------- |
| 0-44               | starts 2 producers and 2 consumer threads |                                 |                              |                                                  |                              |
| 45                 |                                           | tries to put data; obtains lock |                              |                                                  |                              |
| 46-51              |                                           |                                 | tries to remove data; wait() |                                                  |                              |
| 52-53              |                                           |                                 |                              | tries to put data                                |                              |
| 54-56              |                                           |                                 |                              |                                                  | tries to remove data; wait() |
| 57                 |                                           | puts data                       |                              |                                                  |                              |
| 58-59              |                                           | notify()                        |                              |                                                  |                              |
| 60                 |                                           |                                 |                              | obtains lock on queue; tries to put data; wait() |                              |

In QueueTest, there are only 2 workers, each having to put data first before removing data. If the queue is full or there is no data for the respective operations, they wait. The signal from notify() can only be lost if 1 worker finishes execution before the 2nd worker begins. However, in that case, the queue would be empty and the 2nd worker would not need to wait() in the first place.