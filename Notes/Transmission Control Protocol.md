---
title: "Transmission Control Protocol"
date: 2022-11-08
lastmod: 2022-11-21
---
# Transmission Control Protocol (TCP)
TCP is the transport layer above the [internet protocol](Notes/Internet%20Protocol.md), providing an abstraction of a reliable network running over an unreliable channel, hiding most of the complexity of network communication. It optimises for accurate delivery.
## TCP Connection
### Three-way Handshake
Every TCP connection begins with a 3-way handshake:
![](https://i.imgur.com/5ejEhge.png)
1. SYN: Client picks a random sequence number *x* and sends a TCP segment with SYN bit set to 1, which may also include additional TCP flags and options.
2. SYN ACK: Server increments x by 1 and uses this in the acknowledgement number. It picks its own random sequence number for y, appends its own set of flags and options
3. ACK: Client increments both x (sequence number) and y (acknowledgement number) by 1 and completes the handshake
#### Performance Implications
The client can send a data packet immediately after the ACK packet but the server must wait for the ACK packet before it can dispatch any data.

Each new connection will have a full roundtrip of latency before any application data is transferred.
#### TCP Keep-Alive
Rather than having to complete 3-way handshake for each data transfer, allow long-lasting connections to immediately transfer application data.
#### TCP Fast Open (TFO)
Allow data transfer within the SYN Packet.
### Connection stream
![](https://i.imgur.com/fy9tDBn.png)
- Maximum Segment Size (MSS): max data that can be placed in a segment
- Maximum Transmission Unit (MTU): largest link layer frame that can be sent
### Closing
![](https://i.imgur.com/CVxBm3K.png)
## TCP Segment Structure
![](https://i.imgur.com/oeUX3hM.png)
### Sequence Numbers
Consider a 500,000 byte file with MSS = 1000 bytes:
![](https://i.imgur.com/lOu8NFO.png)
Sequence numbers are over the bytes and not the segments. The first segment gets a sequence number 0, second segment gets a sequence number 1000, etc.
### Acknowledgement Numbers
It is the sequence number of the next byte of data that the host is waiting for. 

TCP provides **cumulative acknowledgements** where an acknowledgement number *y* represents all segments < y have been successfully delivered.
*Example: If A received a segment (0-535) and a segment (900-1000), it places 536 as its acknowledgement number.*
![500](https://i.imgur.com/j4EUpai.png)
## Reliable Data Transfer
### Estimating Timeout
TCP uses a timeout retransmit mechanism to recover from lost segments similar to [RDT 3.0 Lossy channels](Notes/Transport%20Layer.md#RDT%203.0%20Lossy%20channels). 
![](https://i.imgur.com/RBWoT52.png)
TCP calculates the estimated RTT based on samples of RTT which are taken approximately once every RTT.
![](https://i.imgur.com/smjIOf1.png)
![](https://i.imgur.com/zIOU5yi.png)
### Retransmission
![](https://i.imgur.com/qClH4n8.png)
Scenarios:
![](https://i.imgur.com/SH8oVnE.png)
![300](https://i.imgur.com/ydvtIuz.png)
#### TCP Fast Retransmit
The timeout period can be relatively long, delaying retransmission of the lost packet. The sender can detect packet loss well before the timeout event by noting **duplicate ACKs**. When *3* duplicate ACKs are received, a fast retransmit is performed before the timer expires.
## Flow Control
TCP places received bytes in a receive buffer. Flow control is a mechanism to prevent the sender from overwhelming the receiver with data it may not be able to process—the receiver may be busy.
### Receive Window (rwnd)
Transmit a receive window value in each ACK packet between both sides to communicate the size of available buffer space to hold incoming data such that the buffer does not overflow:
![](https://i.imgur.com/G2cu6EX.png)
## Stateful: Order of transmission
TCP is capable of transmitting messages spread across multiple packets without explicit information from the packets themselves.

How? TCP is stateful and connection state is allocated on both ends of the connection, allowing data to be sequenced, delivered in order and retransmitted when lost.
## Congestion Control
A mechanism to throttle senders in the face of network congestion (rather than that of application processing speed in flow control).
Issues caused by congestion:
- Large queuing delays occur as sending rate nears the link capacity, as router buffers start to fill up
- Sender must perform retransmissions for dropped packets
- Unneeded retransmissions (premature timeout) use up available bandwidth
### Congestion Window (cwnd)
Sender side limit on the amount of unacknowledged data the sender can send into the network. 
Since there is also the rwnd, the upper bound of unacknowledged data is:
$LastByteSent - LastByteAcked \le min(cwnd,rwnd)$
### Algorithm
Approach: each sender limit the rate at which it sends traffic into its connection as a function of perceived network congestion. (increase if less, decrease if more congestion)
![](https://i.imgur.com/wLRNURJ.png)
- Duplicate ACKs implies a lost segment and and a lost segment implies congestion
- ACK indicates successful receive, rate can be increased
#### Slow Start
![](https://i.imgur.com/iFjnnIM.png)
#### Performance Implications
The maximum amount of un-acked data is $min(rwnd, cwnd)$. The server can send up to that amount of network segments to the client at which point it must stop and wait for an acknowledgement.

The performance of the connection is often limited by the round trip time (latency) or the congestion window
Time to reach a cwnd = N:
![](https://i.imgur.com/8i8jDM4.png)
![](https://i.imgur.com/bf30Siq.png)
#### Congestion Avoidance
When a timeout occurs, `cwnd = 1` and `ssthresh` is set to `cwnd/2`, restarting the slow start process. We can use `ssthresh` to figure out when we are nearing a "reckless" value and stop the slow start process. How should cwnd be adjusted after this? 
![](https://i.imgur.com/mmi1F8C.png)
TCP changes to a linear increase in the cwnd rather than exponential increase.
#### Fast Recovery
TCP Reno incorporates fast recovery, to increase cwnd by 1 MSS for each duplicate ACK  for the missing segment
![](https://i.imgur.com/np1L7hU.png)
#### Summary
![](https://i.imgur.com/qO06hi5.png)
### Throughput
![](https://i.imgur.com/uvKZYQk.png)
### Fairness
![](https://i.imgur.com/THDL0za.png)
![](https://i.imgur.com/i68fmak.png)
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
