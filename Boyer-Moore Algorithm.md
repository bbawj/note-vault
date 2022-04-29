# Boyer-Moore Algorithm
Definitions
: T denotes the input text to be searched. Its length is n.
: P denotes the string to be searched for, called the pattern. Its length is m.

Steps
1. Process the text T[1...n] from __left to right
2. Scan the pattern P[1...m] from __right to left.
3. Generate tables to find the maximum steps to slide the pattern after a mismatch
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
## Good suffix rule (matchJump)
Derive maximum shift from the structure of the pattern.
Run through each case below in order (1 -> 2 -> 3) so as to shift by the least amount such that a suffix is matched

Case 1: matching suffix occurs earlier in the pattern and __preceded by a different character.
![](https://i.imgur.com/kSib0oL.png)

Case 2: matching suffix occurs at the __start of the pattern.
![](https://i.imgur.com/SeNC4aR.png)

Case 3: __no occurrence of the matching suffix in the rest of the pattern__.
![](https://i.imgur.com/TzWDiUH.png)

### Example Array
![](https://i.imgur.com/I0EBLMT.png)

![](https://i.imgur.com/FkttkeI.png)

## Pseudocode
![](https://i.imgur.com/ws3S3db.png)
