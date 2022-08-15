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
This is done using [[Combinational Circuits#Multiplexer|multiplexers]] to choose source registers and using a [[Combinational Circuits#Decoder|decoder]] to select a destination register.
![](https://i.imgur.com/iW9cx1f.png)

![](https://i.imgur.com/u5IrSTh.png)
### Memory organization
#### Instruction memory
![](https://i.imgur.com/qCzi7WY.png)
#### Data memory
![](https://i.imgur.com/L0PnDz1.png)
### Instructions
Based on the system we can design a set of computer [[Instructions]].