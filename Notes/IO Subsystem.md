# I/O Subsystem
![](https://i.imgur.com/5Y5N0mC.png)
## I/O Hardware
The hardware communication between I/O devices and the CPU is done through the [signal chain subsystem](Notes/Signal%20Chain%20Subsystem.md).
## Kernel I/O Subsystem
- Device drivers are the only aspects which interface directly with the hardware.
- This layering system allows devices to be added and removed without having to change the kernel
### I/O Scheduling
The OS is responsible for using hardware efficiently. Schedule I/O requests by rearranging the order of services.
- For disk drives, we need to minimise the seek time as sequential access is much faster. OS needs to perform [](Notes/Disk.md#Disk%20Scheduling%7Cdisk%20scheduling).
### Buffering
Store data in memory while transferring between devices to handle device speed mismatch and transfer size mismatch.
### Caching
Cache copies of data to improve efficiency for files that are being written and reread rapidly.
### Spooling
Store the output for a device to be used when a separate device can serve it, e.g. for devices that can serve only 1 request a time such as printers.
## Performance
I/O operations require a lot of overhead
- CPU needs to execute device driver code
- Context switch due to interrupts
- Data copying between controllers and memory
![](https://i.imgur.com/kX7xvZo.png)
### Asynchronous I/O
![](https://i.imgur.com/Wl6afzK.png)
> [!Non-blocking IO]
> The process remains in the running state (not ready state!)
### Double buffer
![](https://i.imgur.com/ZQFfLjn.png)
## Practice Problems
![](https://i.imgur.com/1ybEL8w.png)
a. False. Buffers are used to support different device transfer speeds. What is described is the role of cache
b. False. Non-blocking I/O call puts the process back in the ready or running state
c. False.
![](https://i.imgur.com/jHFvSlM.png)
a. Double buffer and async i/o
```c
buffer2 <- async read block;
while (not end of file) {
	while iO;
	buffer1 <- buffer2;
	buffer2 <- async read block;
	process buffer1;
}
```
b. Contiguous file allocation is best as the data being read is the entire file. If this entire file is stored contiguously, the time needed for the disk to access the data is minimised.
![](https://i.imgur.com/elRvkyd.png)
a. Not necessarily, if requests are issued one at a time, the disk driver has no opportunity for SCAN optimisation (SCAN = FCFS). This can be solved by concurrently generating IO requests.
b. Under light load, the overhead for scheduling might become greater than the average seek time. ~~Performance of a disk scheduling algorithm can be affected based on the file allocation system utilised. For example, a SCAN algorithm on a linked file allocation method would have poor performance. In linked file allocation, the data blocks accessed are located all over different sectors in the disk. This means that a complete file access operation might require the operation to wait for the disk arm to move from one end to the other.~~
![](https://i.imgur.com/rzvq9fN.png)

![](https://i.imgur.com/kZDIjOP.png)
~~Seek order: 4,10,23,35,35,40,45,50,70,132
$$
\begin{align}
&\text{Single cylinder seek time}=20.1ms\\
&\text{Total seek time}=(4+6+13+12+5+5+5+20+52)\times20.1=2452.2ms\\
&\text{Total rotational latency and transfer time}=(8+2)\times10=100ms\\
&\text{Average time}=2452.2+100=2552.2ms
\end{align}
$$
