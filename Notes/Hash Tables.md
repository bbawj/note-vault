# Hash Tables
The problem with direct addressing: when the number of keys known to us is large, we are unable to map each key to their own slot.

## Hash functions
A hash function allows us to compute the slot from the key and reduce the amount of storage required to store all the keys.
![illustration of hashing](https://i.imgur.com/b99dAuw.png)

The time complexity for searching an element is O(1)

_A good hash function satisfies (approximately) the assumption of simple uniform hashing: each key is equally likely to hash to any of the m slots, independently of where any other key has hashed to._

### Division method
Divide the key by some prime number not too close to a power of 2 (bits are in powers of 2)
$h(k)=k\ mod\ m$
### Multiplication method
_I am not too sure how this works_. Refer to “Introduction to algorithms”, 2009, p. 263

## Collision Resolution
### Linked lists
We can combine the hash table with a [[Linked Lists]], placing all the elements that hash to the same slot into the same linked list:
![](https://i.imgur.com/9PRrarD.png)

| Search                                 | Insert | Delete |
| -------------------------------------- | ------ | ------ |
| Proportional to the length of the list | O(1)   | O(1) if doubly-linked       |
### Probing
The extra memory freed by not storing pointers provides the hash table with a larger number of slots for the same amount of memory, potentially yielding fewer collisions and faster retrieval.
#### Open addressing
We systematically examine table slots until either we find the desired element or we have ascertained that the element is not in the table.
```
HASH-INSERT(T, k) 
i = 0
repeat
	j = h(k,i)
	if T[j] == NIL
		T[j] = k
		return j
	else i++
until i==m
error "hash table overflow"
```