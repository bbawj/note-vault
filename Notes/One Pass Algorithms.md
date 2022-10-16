# One Pass Algorithms
Algorithms where the data is read only once from the disk.
## Example using Select
![](https://i.imgur.com/WgOKFsA.png)
- I/O Cost: B(R)
- Space: M >= 1
## Duplicate Elimination
![](https://i.imgur.com/BwMqbS2.png)
- I/O Cost: B(R)
- Space: $M-1 \ge B(distinct(R))$
## Natural Join
![](https://i.imgur.com/excURMh.png)
- I/O Cost: B(R) + B(S)
- Space: $M-1 \ge min(B(R),B(S))$
### Nested Loop Join
Can be considered "one and a half pass algorithm": One argument is read only once while another is read repeatedly
#### Simple Nested Loop Join
Sacrifice some I/O time in order to save on memory:
![](https://i.imgur.com/Rt4LHH6.png)
- I/O Cost: $B(R) + B(S)\times B(R)$
- Space: $M \ge2$
#### Block based nested loop join
We can improve the I/O cost by utilizing all the buffers available:
![](https://i.imgur.com/UZwOtmQ.png)
- I/O Cost: $B(R) + B(S)\times (B(R)/(M-1))$
- Space: $M \ge2$
If M is large -> devolves into the one pass algorithm
If M is 2 -> devolves into the simples nested loop join algorithm
## Practice Problems
![](https://i.imgur.com/cMgeDuA.png)
a. One pass algorithm means that at least one of the relations must be fully loaded into the input buffer. M must be at least 100 + 1 = 101
I/O cost: 100 + 150 = 250
b. 
S in the outer loop: same as in (a)
R in the outer loop: $150+100\times150/100=300$
c. 
When the smaller relation is put in the outer loop and fits within the available M-1 buffers, the block based algorithm becomes a one-pass algorithm and they share the cost.
![](https://i.imgur.com/OxR8CyD.png)
We can load 999 blocks of R and 1 block of S at a time.
$Cost = (20000/999)\times5000+20000\approx120101$
![](https://i.imgur.com/9bIMrwI.png)

![](https://i.imgur.com/AiLTw99.png)

