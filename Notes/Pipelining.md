# Pipelining
![](https://i.imgur.com/q5XIG5f.png)
## Datapath
Pipelining makes use of extra registers between each pipeline in order to store the necessary data and control signals needed by the current instruction for the next stage. Without it, the next instruction will override the information.
![](https://i.imgur.com/fAFW3V8.png)
## Hazards
A hazard is a drop in efficiency in the pipeline due to stalling.
We can measure the effect of stalls using steady state CPI
$$\text{Steady State CPI} = (No.Instructions+No.Stalls)/No.Instructions$$
### Data hazard
When either the source or destination register of an instruction is not available at the time expected in the pipeline.
![](https://i.imgur.com/PadZEnm.png)
RAW dependencies are difficult to handle and results in stalling for a pipeline architecture.
#### Detect and Wait
Wait until the required value is available in the register file by stalling (hardware) or inserting NOPs (software)
![](https://i.imgur.com/A7fwKHg.png)
#### Data Forwarding
__Data forwarding via register__: We can write and read from register in the same clock cycle. This means that WRITE-BACK and DECODE stage can happen at the same time
![](https://i.imgur.com/GtevmBH.png)
> [!note]
> Forwarding via register is easy to implement. Even when we say there is no forwarding, forwarding via register is considered to still be in place

For other stages, we can only forward from the _previous_ clock cycle:
![](https://i.imgur.com/OIFnJei.png)
Notes
- SUB needs X1 value latest during the execute stage
- AND can obtain X1 latest through register forwarding
#### Dynamic Scheduling
Out-of-order execution and completion:
![](https://i.imgur.com/Uxcv79x.png)
Reordering introduces the possibility of WAR and WAW hazards which were not possible in an in-order execution pipeline. These can be solved via __register renaming__:
![](https://i.imgur.com/E8tAmwX.png)
#### Loop Unrolling
Further optimizations can be made for looping code. Loop segments contain a high level of overhead _(lines that work on the loop variable and branch commands)_, which are not directly contributing to the work of the loop body.
```assembly
for (i=999; i>=0; i=i–1) 
	x[i] = x[i] + s;
"""""""
L.D F0,0(R1) 
DADDUI R1,R1,#-8 //overhead
ADD.D F4,F0,F2  
stall //overhead
stall //overhead
S.D F4,8(R1) 
BNE R1,R2,Loop //overhead
```
![](https://i.imgur.com/fncX13C.png)
Combine unrolling with dynamic scheduling:
![](https://i.imgur.com/OaYIm4y.png)
### Control hazard
Conditional and unconditional jumps, subroutine calls, and other program control instructions can stall a pipeline because of a delay in the availability of an instruction.

A naive (conservative) way would be to stall the pipeline whenever we encounter a branch instruction. Depending on hardware, this results in number of stalls (_branch penalty_) based on which stage of the pipeline the branch address is determined.
![](https://i.imgur.com/QY93XPM.png)
Worst case: One clock cycle stall or flush of one instruction after each branch.
$\text{Pipeline Stall Cycles per Instruction due to Branches} = \text{Branch frequency} \times \text{Branch Penalty}$
#### Static Branch Prediction
![](https://i.imgur.com/nWF7PMD.png)

![](https://i.imgur.com/TCYnvwC.png)
Delayed Branching: schedule an independent instruction in the branch delay slot. If branch penalty is 1, we will have 1 branch delay slot.
![](https://i.imgur.com/uCzBjHJ.png)
### Structural hazard
When two instructions require the use of a given hardware resource at the same time that will lead to a stall in the pipeline (one instruction has to wait at least for a clock cycle). 

Consider we have only one memory. For a case when write stage access the memory for writing the result and the instruction fetch stage tries to fetch the instruction from the memory at the same time.