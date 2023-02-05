---
title: "Network Layer"
date: 2023-01-21
---
# Network Layer
The network layer offers **logical communication between hosts**, which is distinct from the [logical communication from the Transport Layer, which is within the host itself](Notes/Transport%20Layer.md). The network layer provides 2 important functions:
- Forwarding: move packets from router input to appropriate output based on a **forwarding table**
- Routing: use routing algorithms to determine the route for the packet to take 
![](https://i.imgur.com/lzfb7Cm.png)
Routers in the network topology communicate with one another using routing packets in order to determine the best route. This information is populated in the forwarding table.
## Forwarding Table
In the case of a 32 bit IP address, a brute force implementation of the forwarding table would have 1 entry for each of the 4 billion possible addresses. To make things scale, forwarding tables use ranges instead and match based on the longest prefix for which interface to forward to:
![](https://i.imgur.com/r0DQ9qq.png)
## Internet Protocol (IP)
The [Internet Protocol](Notes/Internet%20Protocol.md) handles addressing conventions, datagram format and packet handling conventions.
## Network Address Translation (NAT)
[Network Address Translation](Notes/Network%20Address%20Translation.md) allows local home subnets to grow bigger without having to request for additional address blocks from the ISP.
## Internet Control Message Protocol (ICMP)
![](https://i.imgur.com/LNEfMNs.png)

![](https://i.imgur.com/CBk42YD.png)
