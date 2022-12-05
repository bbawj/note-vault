---
title: "HTTP"
date: 2022-12-03
lastmod: 2022-12-05
---
# Hypertext Transfer Protocol
HTTP is an application layer, above the transport or optional encryption layer.
## A brief history rundown
1. HTTP 0.9 began in 1991 with the goal of transferring HTML between client and server. 
2. HTTP 1.0 evolved to add more capabilities such as header fields and supporting more than HTML file types, becoming a misnomer for hypermedia transport. 
   A typical plaintext HTTP request ![400](https://i.imgur.com/NMRHtwo.png)
3. HTTP 1.1 introduced critical performance optimisations such as keepalive connections, chunked encoding transfers and additional caching mechanisms.
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

These connections are considered independent, and hence do not face the same head-of-line blocking issues in parallel server processing.
### Domain Sharding
Although browsers can maintain a connection pool of up to 6 TCP streams per host, this might not be enough considering how an average page needs 90+ individual resources. If delivered all by the same host, there will be queueing delays:
![](https://i.imgur.com/b7BFL36.png)
Sharding can artificially split up a single host *e.g. www.example.com into {shard1,shard2}.example.com*, helping to achieve higher levels of parallelism at a cost of additional network resources.
## Enhancements in HTTP 2.0
HTTP 2.0 extends the standards from previous versions, and is designed to allow all applications using previous versions to carry on without modification.
### Binary Framing Layer
At the core of the performance enhancements, is this layer which dictates how the HTTP messages are encapsulated and transferred. Rather than delimiting parts of the protocol in newlines like in HTTP 1.x, all communication is split into smaller frames and encoded in binary:
![500](https://i.imgur.com/X63wgSm.png)
> [! Frames, Messages and Streams]
> Frames: The smallest unit of communication in HTTP 2.0, each containing a frame header, which at minimum identifies the stream to which the frame belongs. It contains data such as HTTP headers, payload etc.
> 
> Messages: A complete sequence of frames that map to a logical message. It can be a request or response.
> 
> Stream: A bidirectional flow of bytes within an established connection
> 
> *Each TCP connection can carry any number of streams which communicates in messages consisting of one or multiple frames.*
### Request and Response Multiplexing
In [Why not do server processing in parallel?](Notes/HTTP.md#Why%20not%20do%20server%20processing%20in%20parallel?), we find that only one response can be delivered at a time per connection. HTTP 2.0 removes these limitations.
![](https://i.imgur.com/4yenXqI.png)
With this, workaround optimisations in HTTP 1.x such as domain sharding is no longer necessary.
### Request Prioritisation
The exact order in which the frames are interleaved and delivered can be optimised further by assigning a 31 bit priority value (0 represents highest, $2^{31}-1$ being the lowest). HTTP 2.0 merely provides the mechanism for which priority data can be exchanged, and *does not implement any specific prioritisation algorithm*. It is up to the server to implement this.
### Server Push
A document contains dozens of resources which the client will discover. To eliminate extra latency, let the server figure out what resources the client will require and push it ahead of time. Essentially, the server can send multiple replies for a single request, without the client having to explicitly request each resource.
![](https://i.imgur.com/x37U1Im.png)
### Header Compression
Each HTTP transfer carries a set of headers that describe the transferred resource and its properties. In HTTP 1.x, this metadata is always sent as plain text and adds anywhere from 500–800 bytes of overhead per request, and kilobytes more if HTTP cookies are required.
#### Header table
A header table is used on both the client and server to track and store previously sent key value pairs. They are persisted for the entire connection and incrementally updated both by the client and server. Each new pair is either appended or replaces a previous value in the table. 
This allows a new set of headers to be coded as a simple difference from the previous set:
![400](https://i.imgur.com/E09FU77.png)
