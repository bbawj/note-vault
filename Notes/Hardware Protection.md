---
title:"Hardware Protection"
---
# Hardware Protection
## Dual mode operation
Differentiates between at least 2 modes of operations
1. User mode: execution of user processes
2. __Monitor mode__ (supervisor/system/kernel mode): execution of operating system processes

![](https://i.imgur.com/sXUhGdk.png)
> [!Kernel mode vs root/admin]
> Kernel mode is not the same as root/admin privileges. Kernel or user modes are hardware operation moes while the root/admin is just a user account in the OS. 
> 
> The root/admin may execute code in kernel mode indirectly.
## I/O Protection
_All I/O instructions are privileged instructions._ The OS will ensure that they are correct and legal.
## Memory Protection
OS needs to set the range of legal addresses a program may access. This is done using 2 registers.
Base register: holds the first legal memory address
Limit register: contains the size of the legal range
![](https://i.imgur.com/gNNgytr.png)
## Practice Problems
Given a base register value of 0x1000 and a limit register value of 0x1000, access to memory location 0x1FFF will generate a trap.
False. Each access to memory by a process must be in the range [base, base+limit-1]. In this case, it translates to the range [0x1000, 0x1FFF].
