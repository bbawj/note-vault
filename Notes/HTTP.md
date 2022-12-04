---
title: "HTTP"
date: 2022-12-03
---
# Hypertext Transfer Protocol
## A brief history rundown
1. HTTP 0.9 began in 1991 with the goal of transferring HTML between client and server. 
2. HTTP 1.0 evolved to add more capabiliteis such as header fields and supporting more than HTML file types, becoming a misnomer for hypermedia transport. 
3. HTTP 1.1 introduced critical performance optimisations such as keepalive connections, chunked encoding transfers and additional caching mecahnisms.
4. HTTP 2.0 aimed to improve transport performance for lower latency and higher throughput.
## Optimisations in HTTP 1.1
![500x500](https://i.imgur.com/WlgrLkY.png)
### HTTP Keepalive
Reuse existing TCP connections paired with [TCP Keep-Alive](Notes/Transmission%20Control%20Protocol.md#TCP%20Keep-Alive) to save 1 roundtrip of network latency
![500x550](https://i.imgur.com/TnE3vW7.png)
### HTTP Pipelining
Persistent HTTP implies a strict FIFO order of client requests: *dispatch request, wait for full response, dispatch next request*. Pipelining moves the queue to the server side, allows the client to send all requests at once, and reduces server idle time by processing requests immediately without delay.
![500x550](https://i.imgur.com/XX16E1l.png)
#### Why not do server processing in parallel?
![500x500](https://i.imgur.com/pElNewg.png)
The HTTP 1.x protocol enforces a requirement similar to that encountered in [TCP Head-of-Line Blocking due to its requirement for strict in-order packet delivery](Notes/Transmission%20Control%20Protocol.md#Head-of-Line%20Blocking), where there must be strict serialization of returned responses. Hence, although the CSS response finishes first, the server must wait for the full HTML response before it can deliver the CSS asset.
### Parallel TCP Connections
Rather than opening one TCP connection, and sending each request one after another on the client, we can open multiple TCP connections in parallel. In practice, most browsers use a value of 6 connections per host.

These conncetions are considered indepedent, and hence do not face the same head-of-line blocking issues in parallel server processing.
### Domain Sharding

