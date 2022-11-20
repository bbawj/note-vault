---
title: "Instruction Level Parallelism"
---
# Instruction Level Parallelism
We can group multiple independent instructions and execute them concurrently in different functional units of a single processor.
![](https://i.imgur.com/yzE277X.png)
## Approaches
Hardware approach: Superscalar processor
Software approach: compiler based using a Very Long Instruction Word (VLIW) processor
![](https://i.imgur.com/Ox1n5Kf.png)
### Superscalar Processing
![](https://i.imgur.com/ljdhC05.png)

Way-2 can be assigned load and store instructions while Way-1 will handle other instructions:
![](https://i.imgur.com/LPv8fUr.png)
We can introduce previous techniques to reduce data hazard and improve overall CPI:
![](https://i.imgur.com/3eIU4dS.png)
### Very Long Instruction Word
![](https://i.imgur.com/NV8jkzd.png)

![](https://i.imgur.com/at4qsxH.png)
## Practice Problems
![](https://i.imgur.com/TonQ5Bf.png)
a.
P1 in order:
| Time | Lane 1 | Lane 2 |
| ---- | ------ | ------ |
| 1    | A      |        |
| 2    | B      |        |
| 3    | C      | D      |
| 4    | E      | F      |
| 5    | G      |        |
| 6    | H       |        |
P2 out of order:
| Time | Lane 1 | Lane 2 |
| ---- | ------ | ------ |
| 1    | A      | D      |
| 2    | B      | E      |
| 3    | C      | F      |
| 4    | G      |        |
| 5    | H      |        |

b. $Speedup = 6/5=1.2$
