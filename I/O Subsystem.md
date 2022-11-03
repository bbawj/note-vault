# I/O Subsystem
![](https://i.imgur.com/5Y5N0mC.png)
## I/O Hardware
The hardware communication between I/O devices and the CPU is done through the [[Notes/Signal Chain Subsystem|signal chain subsystem]].
## Kernel I/O Subsystem
- Device drivers are the only aspects which interface directly with the hardware.
- This layering system allows devices to be added and removed without having to change the kernel
### I/O Scheduling
The OS is responsible for using hardware efficiently. Schedule I/O requests by rearranging the order of services.
- For disk drives, we need to minimise the seek time as sequential access is much faster. OS needs to perform [[Notes/Disk#Disk Scheduling|disk scheduling]].
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
### Double buffer
![](https://i.imgur.com/ZQFfLjn.png)
