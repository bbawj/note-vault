# Pipelining
![](https://i.imgur.com/q5XIG5f.png)
## Datapath
Pipelining makes use of extra registers between each pipeline in order to store the necessary data and control signals needed by the current instruction for the next stage. Without it, the next instruction will override the information.
![](https://i.imgur.com/fAFW3V8.png)
## Hazards
A hazard is a drop in efficiency in the pipeline due to stalling.
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
### Control hazard
Conditional and unconditional jumps, subroutine calls, and other program control instructions can stall a pipeline because of a delay in the availability of an instruction.
### Structural hazard
When two instructions require the use of a given hardware resource at the same time that will lead to a stall in the pipeline (one instruction has to wait at least for a clock cycle). 

Consider we have only one memory. For a case when write stage access the memory for writing the result and the instruction fetch stage tries to fetch the instruction from the memory at the same time.