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
### TCP
[Transmission Control Protocol](Notes/Transmission%20Control%20Protocol.md) provides a reliable channel service to the applications which invoke it.
## Principles of Reliable Data Transfer
![](https://i.imgur.com/lcBKkXz.png)
### Fundamentals
We can incrementally build a reliable data transfer (rdt) protocol using [[Finite State Machines|finite state machines]]
#### RDT 1.0 reliable under channel
![](https://i.imgur.com/CXSayAv.png)
#### RDT 2.0 error checking
Add error checking through checksum calculation, acknowledgements (ACKs) and negative acknowledgements (NAKs), and have the sender retransmit the corrupted sentence.
![](https://i.imgur.com/ULJ2VIw.png)
#### RDT 2.1 ACK corruption
*The fatal flaw in rdt 2.0 is that the ACK or NAK packets may in itself be corrupted. How should the protocol recover from such errors?* Here are some ideas:
- Keep adding more "ACK" types. Reply with "What did you say?". This can continue infinitely
- Add enough checksum bits to recover from bit errors
- Retransmit packets if the sender receives a corrupted ACK or NAK. However, we won't be able to tell if its new data or a retransmission on the receiver side!
We can add a new field to the data packet called the **sequence number**. The receiver need only check this to determine whether or not the received packet is retransmission.
- For a stop-and-wait protocol, a 1 bit number will allow the sender to tell: no change -> retransmission, change -> new packet.
Sender:
![](https://i.imgur.com/lfXGhV8.png)
Receiver:
![](https://i.imgur.com/pUKCgOO.png)
#### RDT 2.2 NAK-less
We can remove the need for NAKs by sending an ACK only for the last correctly received packet. If the sender receives 2 ACKs for the same packet, it knows that the following packet was not received correctly.
![](https://i.imgur.com/uWZ4eqn.png)
#### RDT 3.0 Lossy channels
The data packet, along with ACKs can be lost. We need ways to detect the loss, and actions to take to recover from the loss.
Sender side recovery: Wait for some timeout delay to receive an ACK, else retransmit. 
1. Start a timer each time a packet is sent
2. Respond to a timer interrupt by retransmission. This introduces duplicate packets which are handled by RDT 2.2
3. Stop the timer
![](https://i.imgur.com/JmdbEgP.png)

![](https://i.imgur.com/kz8htxG.png)
### Pipelining
RDT 3.0 has a big performance implication due to its stop-and-wait protocol.
![](https://i.imgur.com/JBrYwTi.png)
We can boost performance by allowing the sender to send multiple packets without waiting for an ACK.
![](https://i.imgur.com/HDtzlWY.png)
- The range of sequence numbers must be increased as more in-transit unacknowledged packets are allowed
- Need to buffer the packets which have transmitted but unacknowledged
#### Go-Back-N (GBN)
We need a way to determine the range of sequence numbers needed. This depends on how error recovery is performed. Go-Back-N allows the sender to transmit multiple packets without waiting for ACK but this number is constrained to no more than some maximum N.
A **sliding window** protocol: ![](https://i.imgur.com/LJ0QadF.png)
- [0, base-1] packets which are sent and acked
- [base, nextseqnum-1] packets sent and not acked
- [nextseqnum, base+N-1] packets that can be sent immediately should data arrive
- > [base+n] cannot be used until a packet is acked
Sender:
![](https://i.imgur.com/Q0VL5tp.png)
- The timer is for the oldest unacked packet. This is restarted when we get an ack for a new sequence number.
- All packets in the window are retransmitted on timeout
Receiver:
![](https://i.imgur.com/qKzbKpQ.png)
- Out of order packets are discarded. By default, it will send ACK for the expected sequence number - 1 which it keeps track of internally.
![500](https://i.imgur.com/ZyILnBG.png)

