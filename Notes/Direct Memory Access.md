# Direct Memory Access (DMA)
Feature that allows certain hardware subsystems to access main system memory independently of the central processing unit (CPU).
![](https://i.imgur.com/v3djnZ9.png)
- Critically, it allows the CPU to be free during read/write operations to perform other work which does not involve the system bus.
The OS is responsible for setting up the memory blocks and counters etc. required.
![](https://i.imgur.com/6BCponw.png)
## Modes
### Burst
DMA controller transfers multiple units of data before returning control.
- Fast data transfer rate
- CPU inactivity for longer periods of time as it needs to wait for a long time for control of the data bus
### Cycle stealing
Release data bus after transferring 1 unit of data. Executes between CPU instructions and pipeline stages.
- Slow transfer rate
- CPU inactive time is very short, making it favourable for applications which need to be responsive
### Transparent
Transfer data only when CPU is not using the data bus
- Potentially slowest transfer rate as CPU could always be using the data bus
- CPU basically has no inactive time as transfer only done when it is not using data bus
- Complex to detect when CPU is not using the data bus
