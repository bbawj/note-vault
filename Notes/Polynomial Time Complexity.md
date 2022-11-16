---
title:"Polynomial Time Complexity"
---
# Polynomial Time Complexity
_Polynomial Bound: Worst case complexity is bounded by a polynomial function of **input size**_
> [!NOTE] Sorting algorithm that sorts arrays of 32-bit integers. 
> If you use something like selection sort to do this, the runtime, as a function of the number of input elements in the array, will be $O(n^2)$. 
> 
> But how does n, the number of elements in the input array, correspond to the the number of bits of input? The number of bits of input will be $x = 32n$. Therefore, if we express the runtime of the algorithm in terms of x rather than n, we get that the runtime is $O(x^2)$, and so the algorithm runs in polynomial time.
