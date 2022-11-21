---
title: "User Datagram Protocol"
date: 2022-11-08
lastmod: 2022-11-21
---
# User Datagram Protocol (UDP)
UDP offers flexibility and is commonly used for bootstrapping new transport protocols.
> The appeal of UDP is not in what it introduces, but rather in all the features it chooses to omit.

Datagram: packets delivered via an unreliable service, without delivery guarantees and no failure notifications.

UDP encapsulates user messages into its own packet structure on top of the [Internet Protocol](Notes/Internet%20Protocol.md):
![](https://i.imgur.com/DeoXhWu.png)
> [! Non services]
> 1. No guarantee of message delivery
> 2. No guarantee of order
> 3. No connection state tracking
> 4. No congestion control
## Stateless
Each datagram is carried in a single IP packet with no support for bytestreams. Hence each read will yield the full message and datagrams are not fragmented.
### Problems 
Each connection relies upon [Network Address Translation](Notes/Network%20Address%20Translation.md). Translation tables rely on the connection state in order to create and remove entries as needed, but UDP does not have any processes to define its state (no handshake, no termination sequence).

One solution: UDP routing records are expired on a timer.
## Why UDP
UDP is a simple stateless protocol suitable for bootstrapping other application protocols on top of. Virtually all design decisions are left to the application layer. To leverage UDP, three is a set of knwon best practices and recommendations, one of them is the [RFC 5405](https://datatracker.ietf.org/doc/html/rfc5405).