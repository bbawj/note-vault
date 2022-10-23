# Probabilistic Analysis and Randomised Algorithms
## Exercises
### 5.1.2
Describe an implementation of the procedure RANDOM(a, b) that only makes calls to RANDOM(0, 1). What is the expected running time of your procedure, as a function of a and b?
```
Random(a,b):
	left = a
	right = b
	while left < right:
		middle = (left+right)/2
		choice = Random(0,1)
		if choice == 0:
			right = middle - 1
		else:
			left = middle + 1
	return left
```
$O(log_2(b-a))$
