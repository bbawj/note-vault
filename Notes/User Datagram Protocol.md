---
title: "User Datagram Protocol"
date: 2022-11-08
lastmod: 2022-11-21
---
# User Datagram Protocol (UDP)
UDP is a barebones transport protocol. Aside from the [multiplexing/demultiplexing function](Notes/Transport%20Layer.md#Multiplexing%20and%20Demultiplexing) and some light error checking, it adds nothing to IP.
Sending:
- UDP takes messages from the application process, attaches source and destination port number fields for the multiplexing/demultiplexing service, adds two other small fields, and passes the resulting segment to the network layer. The network layer encapsulates the transport-layer segment into an IP datagram and then makes a best-effort attempt to deliver the segment to the receiving host. 
Receiving:
- UDP uses the destination port number to deliver the segment’s data to the correct application process
> [!Non services]
> 1. There is no handshaking between sending and receiving transport layer entities and hence UDP is said to be *connectionless*. This also means no handshake delay!
> 2. No connection state tracking. This means more active clients on UDP than TCP
> 3. No congestion control.
> 4. Small header size. Less overhead
## UDP Segment
Datagram: packets delivered via an unreliable service, without delivery guarantees and no failure notifications.

UDP encapsulates user messages into its own packet structure on top of the [Internet Protocol](Notes/Internet%20Protocol.md):
![](https://i.imgur.com/DeoXhWu.png)
### UDP Checksum
Sender: Perform a 1s complement sum of all the 16 bit words in the UDP segment, with overflow being wrapped around.
Receiver: Sum all the 16 bit words including the checksum, the result should be 16 1s, else an error is detected.
*Note: this checksum is optional*
## Stateless
Each datagram is carried in a single IP packet with no support for bytestreams. Hence each read will yield the full message and datagrams are not fragmented.
### Problems 
Each connection relies upon [Network Address Translation](Notes/Network%20Address%20Translation.md). Translation tables rely on the connection state in order to create and remove entries as needed, but UDP does not have any processes to define its state (no handshake, no termination sequence).

One solution: UDP routing records are expired on a timer.
