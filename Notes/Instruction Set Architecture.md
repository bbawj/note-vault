---
title: "Instruction Set Architecture"
date: 2022-11-08
lastmod: 2022-11-21
---
# Instruction Set Architecture (ISA)
A set of specifications a programmer must know to write correct and efficient programs for a specific machine
## RISC vs CISC
__RISC__: Reduced Instruction Set Architecture
__CISC__: Complex Instruction Set Architecture
![](https://i.imgur.com/8lGNNw6.png)
![](https://i.imgur.com/YhLB2FB.png)
## ARM ISA
Advanced RISC Machine (ARM)
### Register specification
![](https://i.imgur.com/BaxAdZi.png)
#### Register File
A register file is a set of registers that can be read and written by supplying a register number.
This is done using [](Notes/Combinational%20Circuits.md#Multiplexer%7Cmultiplexers) to choose source registers and using a [](Notes/Combinational%20Circuits.md#Decoder%7Cdecoder) to select a destination register.
![](https://i.imgur.com/iW9cx1f.png)

![](https://i.imgur.com/u5IrSTh.png)
### Memory organization
#### Instruction memory
![](https://i.imgur.com/qCzi7WY.png)
#### Data memory
![](https://i.imgur.com/L0PnDz1.png)
### Instructions Format
Based on the system we can design a set of computer [Instructions](Notes/Instructions.md).
### Addressing Modes
![](https://i.imgur.com/o2BH98s.png)
## Practice Problems
![](https://i.imgur.com/jgFhIqL.png)
```
ADDI X2, X3, #101 ;save loop termination index
loop: 
	LDUR X4, [X11, #0] ;x4 = b[i]
	ADDI X11, X11, #8 ;x11 = x11 + 8
	ADD X4, X4, X1 ;x4 = b[i] + c
	STUR X4, X10, #8 ;a[i] = x4
	ADDI X10, X10, #8 ;a[i] = a[i+1]
	SUBI X2, X2, 1 ;x2 = x2-1
	CBNZ X2, loop
exit END
```
ii. 
1 is run once: 1
2 -> 8 is run 101 times = 1 + 707 = 708
iii.
Line 1 and 4 are memory references, each done 101 times: 202 references.
