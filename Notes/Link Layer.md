---
title: "Link Layer"
date: 2023-02-26
---
# Link Layer
The link layer is responsible for data transfer between *nodes* over *links*.
- Nodes: hosts/routers/switches/access points
- Links: communication channel which connects the nodes, such as broadcast channels (used by WiFi) or point-to-point (used by Ethernet)
- Link layer protocol: can be thought as the "mode of transport" such as either Ethernet or WiFi
*Difference between [Network Layer](Notes/Network%20Layer.md) and link layer: the network layer finds to best route for the datagram while the link layer provides the actual transportation.*

*Where is it implemented?*:  usually in a Network Interface Card (NIC), which contains the hardware to send datagrams to the link, and also software in terms of the controller which bridges the interface between the host CPU and hardware.
## Multiple Access Problem
How to coordinate the access of multiple sending and receiving nodes to a shared link? The link layer implements multiple access protocols.
### Channel Partitioning Protocols
Partition the broadcast channel's bandwidth among nodes sharing the channel.
![500](https://i.imgur.com/jr4iOiA.png)
- **Time-division multiplexing** (TDM) splits up time into time frames with each frame split amongst the nodes. Each node is allowed to transmit during this time. 
- **Frequency-division multiplexing** (FDM): splits up bandwidth into available frequencies. Each node takes 1 frequency for transmission
Problem: what if only 1 node wants to transmit something? For a link bandwidth R, it is capped at R/N bps.
- **Code division multiple access** (CDMA): assign each node a unique code used to encode the data. Each node can transmit simultaneously and the receiver can use the code to determine the sender. 
### Random Access Protocols
Each node transmit at full rate, when a collision occurs, independently choose a random delay before retransmission. Probable for the node to *sneak* in its packet.
#### Carrier Sense Multiple Access
Rather than independently making a decision:
- Carrier sensing (listen before speaking): node listens to the channel before transmitting and waits until it is quiet before transmission
- Collision detection (stop talking if someone else begins talking): node listens to the channel while transmitting and stops if there is interference.
![](https://i.imgur.com/6ebTVjn.png)
### Taking turns
Each node take turns to transmit some data.
- Polling: a master node polls each node in a round robin fashion to tell them when and how much they can transmit.
- Token passing: each node passes a token to another in a fixed order.
## Link layer addressing
The link layer requires its own addresses (MAC address) in order to forward datagrams to the correct network adapter. Each MAC address is unique no matter the location, unlike [IP addresses](Notes/Internet%20Protocol.md#Addressing) which change depending on the network the host is connected to.

*Why link layer address + network layer address (IP)*?: the network is not meant for just IP, without MAC addresses, it cannot easily support other protocols.
### Address Resolution Protocol (ARP)
When a host wants to send a datagram to another over IP, it must give the IP datagram and the destination MAC address. ARP is used to determine the MAC address given an IP address **on the same LAN or subnet**.
![](https://i.imgur.com/Dt0njq9.png)
What if there is no entry in the ARP table for a desired destination?
1. Sender construct ARP packet and indicate to use the MAC broadcast address `FF-FF-FF-FF-FF-FF` before passing to the adapter
2. Adapter encapsulates the packet into the link layer frame and transmits
3. Each adapter on the subnet passes the frame to its own ARP module which checks if the destination IP is its own. If it is, sends the packet back with the desired mapping
#### Sending across different subnets
ARP can only be used to obtain MAC addresses of IP addresses on the same subnet. To send a packet outside:
1. Sender construct the datagram with the IP address of the final destination but the destination MAC address of the router interface.
2. Router deconstructs the datagram (because its own MAC address was found), and forwards the datagram to the MAC address of the final destination IP address (obtained with ARP) using its forwarding table.
## Link Layer Protocols
### Ethernet
- Unreliable: ethernet drops frames which do not pass the CRC check without sending any acknowledgements. The layer above implements reliability while Ethernet transmits unawares of whether it is a retransmission or not.
- Connectionless: no handshaking required
An ethernet frame: 
![](https://i.imgur.com/JP5CjY8.png)
- Preamble: used to synchronise clock rates between nodes
- Destination/Source address: a MAC address
- Type: identifier for the type of network protocol it is meant for
- CRC: used for detecting bit errors in the frame
## Link Layer Switches
A switch receives incoming link-layer frames and forwards them onto outgoing links. 
### Forwarding and filtering
Filtering determines whether a frame should be forwarded or dropped
Forwarding takes the frame's destination MAC address and forwards it to the interface. This is unlike a [router which forwards based on IP addresses](Notes/Building%20Blocks%20of%20the%20Internet.md#Forwarding%20Tables%20and%20Routing%20Protocols)
Switch table:
![](https://i.imgur.com/5jsOdlt.png)
The switch table is built automatically without any intervention:
1. Switch table is initially empty
2. For each incoming frame received on an interface, store the MAC address of the frame's source address, interface which it arrived and the current time
3. Delete address if no frame received with that address as its source after some period of time
Elimination of collisions: a switch buffers frames and never transmits more than 1 frame on a segment at any 1 time. This means that no collision can occur and the [Multiple Access Problem](Notes/Link%20Layer.md#Multiple%20Access%20Problem) is solved.