# Instructions
An example using the ARM ISA
![](https://i.imgur.com/fTodwdd.png)
> [!The datapaths shown below are examples given a single cycle datapath]
## Register Type 
All data values are located in registers  
Addressing Mode: __register addressing mode__
![](https://i.imgur.com/dGc0feL.png)
### Datapath
![](https://i.imgur.com/Ce5IY8W.png)

## Data transfer type
Addressing Mode: __Base/Displacement addressing__
![](https://i.imgur.com/k1DenbQ.png)
### Datapath
LDUR
1. Rn contains the information about WHERE the data in memory is
2. Offset Rn by address value  to get the memory address
3. Store the data from this memory address into Rt
STUR
1. Rn register contains the information about WHERE to store the data
2. Offset the information in Rn by the address value (22 + 64) = 90, to get the destination memory address
3. Store the data inside Rt into this offset value
![](https://i.imgur.com/MPo89Vv.png)
> [!We can utilize a set of extra multiplexers to reuse components for both types]
> 
![](https://i.imgur.com/POeSnMZ.png)
## Immediate type (immediate addressing mode)
![](https://i.imgur.com/XBcUWz2.png)
### Datapath
![](https://i.imgur.com/m6rHt3u.png)
## Conditional Branch type (PC relative addressing mode)
![](https://i.imgur.com/cL7BQNZ.png)
### Datapath
![](https://i.imgur.com/e3nbpwz.png)

## Unconditional Branch type
![](https://i.imgur.com/clXQze5.png)

## Combine all types into a single datapath
![](https://i.imgur.com/Wmjc1KN.png)
Notes:
- Reg2Loc (0) used to select Rm as a source register
- ALUSrc (0) to select register data rather than sign-extended address
- Mem2Reg (0) to select data from ALU rather than memory
![](https://i.imgur.com/yaQJzIZ.png)
Notes:
- Reg2Loc not used as only 1 read
- ALUSrc (1) to select  sign-extended address
- Mem2Reg (1) to select data from memory
![](https://i.imgur.com/PLmtZJS.png)
Notes:
- Reg2Loc (1) to read Rt
- ALUSrc (0) to use data from register rather than address
- Zero-flag in AND-gate together with Branch-flag to select address to add for branching rather than default +4 to load into PC
![](https://i.imgur.com/FZM5SNF.png)
Notes:
- Additional OR-gate to always select the address for branching