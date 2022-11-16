# Transmission Control Protocol (TCP)
## Three-way Handshake
Every TCP connection begins with a 3-way handshake:
![](https://i.imgur.com/yvkT1aC.png)
- SYN: Client picks a random sequence number x and sends a SYN packet, which may also include additional TCP flags and options.
- SYN ACK: Server increments x by 1 and picks own random sequence number for y, appends its own set of flags and options
- ACK: Client increments both x and y by 1 and completes the handshake
### Performance Implications
The client can send a data packet immediately after the ACK packet but the server must wait for the ACK packet before it can dispatch any data.

Each new connection will have a full roundtrip of latency before any application data is transferred.
### TCP Keep-Alive
Rather than having to complete 3-way handshake for each data transfer, allow long-lasting connections to immediately transfer application data.
### TCP Fast Open (TFO)
Allow data transfer within the SYN Packet.
## Stateful: Order of transmission
TCP is capable of transmitting messages spread across multiple packets without explicit information from the packets themselves.

How? TCP is stateful and connection state is allocated on both ends of the connection, allowing data to be sequenced, delivered in order and retransmitted when lost.
## Congestion Control
![](https://i.imgur.com/mmi1F8C.png)
### Flow Control
Flow control is a mechanism to prevent the sender from overwhelming the receiver with data it may not be able to process—the receiver may be busy, under heavy load, or may only be willing to allocate a fixed amount of buffer space.
#### Receive Window (rwnd)
Transmit a receive window (rwnd) value in each ACK packet between both sides to communicate the size of available buffer space to hold incoming data.
### Slow Start
The problem was that flow control prevented the sender from overwhelming the receiver, but there was no mechanism to prevent either side from overwhelming the underlying network: neither the sender nor the receiver knows the available bandwidth at the beginning of a new connection.

Idea: Estimate available capacity by transmitting data. 
#### Congestion Window (cwnd)
Sender side limit on the amount of data the sender can have in flight before receiving and ACK from client. For every received ACK, cwnd doubles.
### Performance Implications
The maximum amount of un-acked data is $min(rwnd, cwnd)$. The server can send up to that amount of network segments to the client at which point it must stop and wait for an acknowledgement.

The performance of the connection is often limited by the round trip time (latency) or the congestion window
Time to reach a cwnd = N:
![](https://i.imgur.com/8i8jDM4.png)
![](https://i.imgur.com/bf30Siq.png)
## Congestion Avoidance
The implicit assumption in congestion avoidance is that packet loss is indicative of network congestion: somewhere along the path we have encountered a congested link or a router, which was forced to drop the packet, and hence we need to adjust our window to avoid inducing more packet loss to avoid overwhelming the network.
## Bandwidth Delay Product
If either the sender or receiver exceeds the maximum unacknowledged data, they will have to wait before they can send any more. 
![](https://i.imgur.com/tE0o5TM.png)
To maximize throughput, *send so much data that there is always an ACK returning back to the sender at the same time we are about to send a data packet*.
$BDP = \text{Data Link Capacity}\times\text{Round Trip Time}$
Example, 10 Mbps available bandwidth and 100ms RTT
$BDP=10\times10^6\times0.1=1\times10^6$ bits
The window size needs to be at least this size to saturate the data link.
## Head-of-Line Blocking
If one of the packets is lost en route to the receiver, then all subsequent packets must be held in the receiver’s TCP buffer until the lost packet is retransmitted and arrives at the receiver. Because this work is done within the TCP layer, our application has no visibility into the TCP retransmissions or the queued packet buffers, and must wait for the full sequence before it is able to access the data. Instead, it simply sees a delivery delay when it tries to read the data from the socket. 
>[! Note]
>Benefits
>1. Applications do not need to deal with packet reordering and reassembly
>
>Cons
>1. Introduces unpredictable latency variation in packet arrival times (jitter)
>2. Application may not need reliable or in-order delivery
