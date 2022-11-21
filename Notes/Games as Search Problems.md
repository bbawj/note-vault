---
title: "Games as Search Problems"
date: 2022-11-08
lastmod: 2022-11-21
---
![](https://i.imgur.com/CZ3e50G.png)

Perfect information also means *fully observable* unlike in Poker where you cannot see the opponent's hand.

## Minimax Search
Maximize own utility and minimize opponent's

1. Generate game tree to terminal state or a certain depth
2. Calculate the utility from the bottom-up (MAX turn will maximize own utility, MIN turn will minimize MAX utility)
3. Select the best move (we can assume that we start as MAX)

Tic-Tac-Toe Tree Example:

[Reflective and rotational symmetries:](https://courses.cs.duke.edu/cps100e/current/assign/ttt/#:~:text=There%20are%20four%20reflective%20symmetries,the%20board%20on%20the%20left.&text=This%20means%20there%20are%20eight,board%20on%20each%20line%20above)

![](https://i.imgur.com/MAQHRXu.png)

