---
title: "Transport Layer"
date: 2023-01-20
---
# Transport Layer
The transport layer provides for **logical communication between application processes running on different hosts**. This means that application processes can send messages to each other without worrying about the details of the underlying physical infrastructure.
> [!Example]
> 12 kids in Ann’s house sending letters to 12 kids in Bill’s house. Ann and Bill are responsible for mail collection and distribution, and interfaces with the postal carrier.
> - hosts = houses  
> - processes = kids  
> - app messages = letters in envelopes  
> - transport protocol = Ann and Bill who demux to in-house siblings  
> - network-layer protocol = postal service
## Multiplexing and Demultiplexing
Multiplexing and demultiplexing is the extension of the host-to-host delivery service provided by the network layer to a process-to-process delivery service for applications running on the hosts.

A host can have multiple network processes running, and each can have 1 or more sockets for which data passes from the network. 
- Multiplexing: gathering data chunks at the source from different sockets, encapsulating each chunk with header information and passing the segments to the network layer
- Demultiplexing: delivering data in a transport layer segment to the correct socket. In UDP a 2-pair {source port, dest port} can uniquely identify the destination socket. In TCP, a 4-pair {source port, source IP, dest port, dest IP} identifies the destination socket.![](https://i.imgur.com/USGmCjF.png)
## Transport Layer Protocols
### UDP
A barebones transport protocol for the Internet is [User Datagram Protocol](Notes/User%20Datagram%20Protocol.md)

