---
title:"Boyer-Moore Algorithm"
---
# Boyer-Moore Algorithm
Definitions
: T denotes the input text to be searched. Its length is n.
: P denotes the string to be searched for, called the pattern. Its length is m.

Steps
1. Process the text T[1...n] from __left to right
2. Scan the pattern P[1...m] from __right to left.
3. Generate tables to find the maximum steps to slide the pattern after a mismatch
4. For every mismatch, we slide by the maximum amount returned by the 2 heuristics
5. Return once we find the pattern or the pattern does not exist (n-m+1) comparisons
## Bad character rule (charJump)
Case 1 (mismatched character does not exist in the rest of the pattern):
![](https://i.imgur.com/9Xh5LdF.png)

Case 2 (mismatched character is found in the rest of the pattern):
![](https://i.imgur.com/2GquQpN.png)

### Example Array
![](https://i.imgur.com/qIMeHEg.png)

### Pseudocode
![](https://i.imgur.com/gtsvjfQ.png)
Right most occurrence such that we do not over slide.

### Simple BM scan
When using charJump only, taking the max of charJump and $m-k+1$ will ensure that we do not _left  shift_ the pattern and that we always at least move by 1 character.
![](https://i.imgur.com/NHcbWYL.png)

Example of where we could have made a _left shift_ of the pattern, which is not what we want:
![](https://i.imgur.com/wsmOIaK.png)

## Good suffix rule (matchJump)
Derive maximum shift from the structure of the pattern.
Run through each case below in order (1 -> 2 -> 3) so as to shift by the least amount such that a suffix is matched.
> [!NOTE] General formula for slide
> $(m-k)+(m-q$)
> - k is the position of the mismatch
> - q is the position of end of the next matching suffix

Case 1: matching suffix occurs earlier in the pattern and __preceded by a different character.
![](https://i.imgur.com/kSib0oL.png)

Case 2: matching suffix occurs at the __start of the pattern.
![](https://i.imgur.com/SeNC4aR.png)

Case 3: __no occurrence of the matching suffix in the rest of the pattern__.
![](https://i.imgur.com/TzWDiUH.png)

Case 4: mismatch on the first character
The last character array entry is always 1. We have no information about matching suffixes from the text as the first comparison is already mismatched. Hence, we can only safely shift by 1 character.
### Example Array
![](https://i.imgur.com/I0EBLMT.png)

![](https://i.imgur.com/FkttkeI.png)

## Pseudocode
![](https://i.imgur.com/ws3S3db.png)

## Examples
![](https://i.imgur.com/0V5xsxb.png)

![](https://i.imgur.com/OuyvZnO.png)
