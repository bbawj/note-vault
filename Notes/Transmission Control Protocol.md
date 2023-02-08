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
The TCP receiver places received bytes in a receive buffer. Flow control is a mechanism to prevent the sender from overwhelming the receiver with data it may not be able to process—the receiver may be busy.
### Receive Window (rwnd)
Transmit a receive window value in each ACK packet between both sides to communicate the size of available buffer space to hold incoming data such that the buffer does not overflow:
![](https://i.imgur.com/G2cu6EX.png)
### Silly window syndrome
1. When the receiver consumes data slowly, the window becomes smaller to the point where the data transmitted is smaller than the packet header resulting in inefficient data transfer (thrashing).
2. When the sender creates data slowly, a small packet that does not fully utilise the maximum segment size is sent also resulting in inefficient data transfer.
#### Nagle's algorithm (case 1)
Applications such as telnet/rlogin generates a 41 byte TCP packet for each 1 byte of user data.
1. Each TCP connection can have only one outstanding (i.e., unacknowledged) small segment (i.e., a tinygram)  
2. While waiting - additional data is accumulated and sent as one segment when the  ACK arrives, or when maximum segment size can be filled  
3. Self-clock: the faster ACKs come, the more often data is sent. Thus automatically on slow WANs fewer segments are sent
#### Delayed acknowledgements
Rather than sending an ACK immediately, the TCP receiver waits up to 200ms. This prevents the sender from sliding its window. Traffic is reduced and potentially more data can be piggy backed on the ACK.
*Host requirements RFC states the delay must be less than 500ms which is the standard timeout interval. This ensures that retransmit is not triggered.*
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
When a timeout occurs, $cwnd = 1$ and `ssthresh` is set to `cwnd at duplicate ACK/2`, restarting the slow start process. We can use `ssthresh` to figure out when we are nearing a "reckless" value and stop the slow start process. How should cwnd be adjusted after this? 
When `ssthresh` is reached, TCP changes to a linear increase in the cwnd rather than exponential increase.
#### Fast Recovery
The slow start process does not restart (cwnd restart at 1 MSS), instead the cwnd is increased by 1 MSS for each duplicate ACK for the missing segment. This is because we can be sure that the receiver can handle at least the ssthresh + duplicate ACKs number of packets.
![](https://i.imgur.com/np1L7hU.png)
3 duplicate ACKs occur at round 12. $ssthresh = 12/2=6$. 
- TCP Renoe: $cwnd = ssthresh + 3 = 9. Followed by linear increase
- TCP Tahoe: $cwnd = 1$. Exponential increase followed by linear increase at ssthresh.
#### Summary
![](https://i.imgur.com/qO06hi5.png)
## Throughput
![](https://i.imgur.com/uvKZYQk.png)
### Bandwidth Delay Product
If either the sender or receiver exceeds the maximum unacknowledged data, they will have to wait before they can send any more. 
![](https://i.imgur.com/tE0o5TM.png)
To maximize throughput, *send so much data that there is always an ACK returning back to the sender at the same time we are about to send a data packet*.
$BDP = \text{Data Link Capacity}\times\text{Round Trip Time}$
Example, 10 Mbps available bandwidth and 100ms RTT
$BDP=10\times10^6\times0.1=1\times10^6$ bits
The window size needs to be at least this size to saturate the data link.
## Fairness
![](https://i.imgur.com/THDL0za.png)
![](https://i.imgur.com/i68fmak.png)
## Head-of-Line Blocking
If one of the packets is lost en route to the receiver, then all subsequent packets must be held in the receiver’s TCP buffer until the lost packet is retransmitted and arrives at the receiver. Because this work is done within the TCP layer, our application has no visibility into the TCP retransmissions or the queued packet buffers, and must wait for the full sequence before it is able to access the data. Instead, it simply sees a delivery delay when it tries to read the data from the socket. 
>[! Note]
>Benefits
>1. Applications do not need to deal with packet reordering and reassembly
>
>Cons
>1. Introduces unpredictable latency variation in packet arrival times (jitter)
>2. Application may not need reliable or in-order delivery
## Exercises
TCP uses delayed ACKs instead of sending and ACK directly after a correctly received packet. Answer the two following questions related to delayed ACKs in TCP. 
a) An ACK must not be delayed more than 500 ms. Why?    
500ms is the amount of time before retransmission timeout.
b) Assume that a TCP segment arrives with the expected sequence number. The previous segment arrived in correct order and it has not been ACKed yet. What will the receiver do now? 
Receiver will send a TCP packet with acknowledgement number = the latest segment sequence number + 1

TCP uses both flow control and congestion control. Explain the overall difference between these. What do they mean? What are their purposes?
Flow control is used to throttle the sender in the case where the receiver is unable to handle the rate of data being sent. Congestion control is used to throttle the sender in the case where there is congestion in the network link.

An application uses TCP and sends data in full size windows (65 535 bytes) over a 1 Gbps channel having a one-way delay of 10 ms. The transmission time can be neglected.  
a) What is the maximum throughput that can be achieved?  
1 window of data can be sent every RTT:
$65535\times8/(20\times10^-3)=2621400 \ bits/s$
b) What channel utilization can be achieved, i.e., how large part of the available bandwidth can be used?
$\frac{26214000}{1\times10^9}=2.6\%$

A client application establishes a TCP connection to a server application to transfer 15 kB of data. The (one-way) delay is 5 ms, RTT (round-trip time) is 10 ms, and the receive window (rwnd) is 24 kB. Assume that the initial congestion window is 2 kB. There is no congestion in the network, the transmission time can be neglected, and the connection establishment phase can be neglected. Calculate the total transfer time.
$$
\begin{align}
\\ \text{Round 1 2kb of data sent}: cwnd = 2*2=4
\\ \text{Round 2 4kb of data sent}: cwnd = 4*2=8
\\ \text{Round 3 8kb of data sent}: cwnd = 8*2=16
\\ \text{Last 1kb sent in round 4}: 10+10+10+5 = 35ms
\\ \end{align}
$$
![](https://i.imgur.com/eo9PkJO.png)
a,b)
![500](Pics/Transmission%20Control%20Protocol%202023-02-07%2014.20.50.excalidraw.svg)
c. 3 duplicate ACKs
d. Timeout
e. 32 segments
f. 21
g. 15
h. 7
i. ssthresh = 4, cwnd = 7
j. ssthresh = 21, cwnd = 4
k. $1+2+4+8+16+21=52$. Round 22 will have sent 21 packets assuming that we are able to successfully send at least the number of data packets `ssthresh` dictates.
![](Pics/Transmission%20Control%20Protocol%202023-02-07%2014.58.22.excalidraw.svg)
![](https://i.imgur.com/KwdfNTw.png)
a.
$$
\begin{align}
\\&\text{Packets sent per cycle}=\frac{W}{2}+(\frac{W}{2}+1)+...+W
\\&=\frac{3W}{2}\times(\frac{W}{2}+1)\div2
\\&=\frac{3W^2}{8}+\frac{3W}{4}
\\&\text{1 packet loss per cycle:}
\\&L=\frac{1}{\frac{3W^2}{8}+\frac{3W}{4}}
\end{align}
$$
b.
$$
\begin{align}
&Rate = \text{Packets transferred per unit time}
\\&\frac{1}L=\frac{3W^2}8+\frac{3W}4
\\&\frac{1}L\approx\frac{3W^2}8
\\&W=\sqrt\frac{8}{3L}
\\&\text{Avg Rate} = \frac{3W}{4RTT}MSS
\\&\approx\frac{1.22MSS}{RTT\sqrt L}
\end{align}
$$
![](https://i.imgur.com/rs3CQYR.png)
a.
$$
\begin{align}
&\text{Max throughput}=10\times10^6\times150\times10^{-3}=150\times10^{4}
\\&x\times MSS=150\times10^{4}\div8
\\&x=125
\end{align}
$$
b.
$$
\begin{align}
&\text{Avg window size} = \frac{3}4W_{max}=93.75
\\&\text{Avg throughput} = \text{Avg window size}/RTT
\\&=93.75\times1500\times8/(150\times10^{-3})
\\&=7,500,00=7.5\ Mbps
\end{align}
$$
c.
$$
\begin{align}
&\text{cwnd after packet loss}=125/2=62.5
\\&62.5\times150\times10^{-3}=9.375s
\end{align}
$$