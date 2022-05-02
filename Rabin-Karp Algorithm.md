# Rabin-Karp Algorithm
<iframe width="560" height="315" src="https://www.youtube.com/embed/qQ8vS2btsxI" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>
## General Idea
1. Convert the pattern (length m) to a number p
2. Convert the first m-characters (the first text window) to a number t with a hash function
3. If p and t are equal, there is possibility of pattern: verify against the actual pattern
4. Rolling hash function: shift the text window one character to the right and convert the new string
5. Repeat until pattern is found or exit

### Rolling hash function
The new hash value can be calculated in $\theta(m)$ time.![](https://i.imgur.com/zZEsAFj.png)
### Improving the hash function
![](https://i.imgur.com/SigphCG.png)

![](https://i.imgur.com/wuyA5GR.png)
![](https://i.imgur.com/9X2K8Yh.png)

![](https://i.imgur.com/fFE0KIp.png)

## Pseudocode
![](https://i.imgur.com/Hf4xSoY.png)

## Complexity
![](https://i.imgur.com/mHXrm1y.png)

## Examples
Rabin-Karp can support wildcard matches by simply considering the position of the wildcard to be a value of 0:
![](https://i.imgur.com/1ZsBQ48.png)
