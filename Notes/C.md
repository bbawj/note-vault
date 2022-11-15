# C Programming
## Special keywords
### Volatile
```c
int volatile foo;
```
Volatile is a qualifier that is applied to a variable when it is declared. It tells the compiler that the value of the variable may change at any time-without any action being taken by the code the compiler finds nearby. 
#### Use in peripheral registers
These registers may have their values changed asynchronously during program flow. Code without this keyword can be *optimised* by the compiler into an infinite loop.
```c
UINT1 * ptr = (UINT1 *) 0x1234;

// Wait for register to become non-zero.  
while (*ptr == 0);  
// Do something else.
```
Compiler interprets the ptr value is being always 0, as it has already loaded the value in the second line, resulting in an infinite loop:
```assembly
mov ptr, #0x1234
mov a, @ptr
loop bz loop

```
Same situations can occur for variables that may be modified in [ISRs](Notes/Interrupts.md) or by [multi-threaded applications](Notes/Thread%20Level%20Parallelism.md).
