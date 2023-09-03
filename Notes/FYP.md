---
title: "FYP"
date: 2023-08-23
lastmod: 2023-08-23
---
# Key terms
AIOT
1. A low power device simply transmits data to a powerful server, that replies with a decision or inference.
2. A low power device runs an operating system and applications, built with high overhead (Debian and Python, for example), which allows machine learning to occur locally, but slowly.
3. Low power device transmit data to a higher powered local server, with ML done on powerful remote server
ZephyrOS
- [Real Time Operating Systems](Notes/Real%20Time%20Operating%20Systems.md) for resource constrained systems
- Can run on many platforms
- Supports IPV4 and IPV6 communication
- Does not support onboard ML training. Must train externally before being deployed on the system
- Run on the emulated low powered devices
QEMU
- use for emulating the low powered devices
## Problem
Is there a way to implement some sort of a framework or library to easily spin up qemu-zephyr instances, and provide functions to make network communication calls? This will improve the emulation process.
```c
{
	// get global model
	inputs = lib_get_model()

    // PERFORM TRAINING
    

	// PERFORM PREDICTION

	// send outputs back to global server
	lib_send_outputs()
}
```

![](Pics/Pasted%20image%2020230823225409.png)
https://arxiv.org/pdf/2111.14347.pdf




