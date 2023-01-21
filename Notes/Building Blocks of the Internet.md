---
title: "Building Blocks of the Internet"
date: 2023-01-17
lastmod: 2023-01-17
---
# Building Blocks of the Internet
![300](https://i.imgur.com/kouLcOu.png)
The internet consists of billions of end systems (hosts), connected together by a network of communication links and packet switches. 
- Communication links include wired and wireless: copper wires, optical fibre, radio. 
- Packet switches such as routers transmit packets of data through a route in the network:
> Consider, for example, a factory that needs to move a large amount of cargo to some destination warehouse located thousands of kilometers away. At the factory, the cargo is segmented and loaded into a fleet of trucks. Each of the trucks then independently travels through the network of highways, roads, and intersections to the destination warehouse. At the destination warehouse, the cargo is unloaded and grouped with the rest of the cargo arriving from the same shipment. Thus, in many ways, packets are analogous to trucks, communication links are analogous to highways and roads, packet switches are analogous to intersections, and end systems are analogous to buildings. Just as a truck takes a path through the transportation network, a packet takes a path through a computer network.
## Internet socket interface
How does one program running on one end system instruct the Internet to deliver data to another program running on another end system? 
End systems attached to the Internet provide a socket interface that specifies how a program running on one end system asks the Internet infrastructure to deliver data to a specific destination program running on another end system. This Internet socket interface is a set of rules that the sending program must follow so that the Internet can deliver the data to the destination program:
> Suppose Alice wants to send a letter to Bob using the postal service. Alice, of course, can’t just write the letter (the data) and drop the letter out her window. Instead, the postal service requires that Alice put the letter in an envelope; write Bob’s full name, address, and zip code in the center of the envelope; seal the envelope; put a stamp in the upper-right-hand corner of the envelope; and finally, drop the envelope into an official postal service mailbox. Thus, the postal service has its own “postal service interface,” or set of rules, that Alice must follow to have the postal service deliver her letter to Bob. In a similar manner, the Internet has a socket interface that the program sending data must follow to have the Internet deliver the data to the program that will receive the data.
## Protocols
Protocols define format and order of messages sent and received among network entities, and actions taken on message transmission and reception
![](https://i.imgur.com/gsZrCb0.png)
### Protocol Layering
Explicit structure allows identification and relationship of the different pieces  
Modularization eases maintenance and updating of system  
- change of implementation of layer’s service transparent to rest of system  
- For example, a change in gate procedure doesn’t affect rest of system
Taken together, protocols of the various layers form the protocol stack:
![](https://i.imgur.com/ztebTbk.png)
### Encapsulation
Each layer encapsulates the payload with additional header information for the next layer to continue the exchange of data with the next layer.
![](https://i.imgur.com/JTSf2kP.png)
## Network edge
Computers and other devices connected to the Internet are called *end systems* because they sit at the edge of the Internet. End systems include both **clients and servers**.
![](https://i.imgur.com/QsHr4kq.png)
The access network is the network that physically connects the end systems to the first router.
### ISP Access
ISP access is how we connect to the ISPs:
- Digital Subscriber Line: uses existing telephone line (twisted copper wire) to exchange data with the telcos central office
- Fiber To The Home (FTTH): 10 Mbps - 100 Gbps
### Local Access
- Ethernet: uses twister copper wire to connect to an ethernet switch which in turn connect to the larger internet.
- WiFi 802.11
## Network Core
The network core represents the mesh of interconnected routers that make up the ISPs.
![](https://i.imgur.com/GqFmuyN.png)
- Tier 1 ISPs span across the globe. But for customers using different ISPs to exchange data, the ISPs themselves must be connected through Internet Exchange Points (IXP)
- Regional ISPs compete with each other and pay T1 ISPs for their traffic
- Access ISPs connect to any lower tier ISPs for their traffic
- Content provider networks create their own private networks which connects its data centres to the internet, bypassing lower tiered ISPs to bring content close to their customers.
### Packet switching
Data is broken into smaller chunks called packets, which are transmitted through communication links and packet switches at the **full transmission rate** of the link.
![](https://i.imgur.com/wJhBaDB.png)
Store and forward transmission: the packet switch must receive the entire packet before it can transfer the first bit of the packet.
![400](https://i.imgur.com/L82ekOV.png)
### Forwarding Tables and Routing Protocols
*How does the router determine which link it should forward the packet onto?*

For the Internet, the [Internet Protocol](Notes/Internet%20Protocol.md) dictates a destination IP address in each packet. Each router has a forwarding table that maps destination addresses (or portions of the destination addresses) to that router’s outbound links. When a packet arrives at a router, the router examines the address and searches its forwarding table, using this destination address, to find the appropriate outbound link. 

*How does the forwarding table get set?*
Internet has a number of special routing protocols that are used to automatically set the forwarding tables. A routing protocol may, for example, determine the shortest path from each router to each destination and use the shortest path results to configure the forwarding tables in the routers.
### Queueing Delay
![](https://i.imgur.com/mlRrKNs.png)
## Exercises
a) What is a communication protocol ?  
A protocol defines the format and order of messages and the set of procedures performed on a message when it is sent or received.
b) Name the different layers in the Internet protocol stack, and place the following  
protocols/functions/concepts at the correct layer : IP, TCP, Ethernet, HTTP, bit coding , FTP, IEEE 802.11 WLAN, TP Category 6, Routing , UDP.  
| Application | Transport | Network | Link        | Physical   |
| ----------- | --------- | ------- | ----------- | ---------- |
| HTTP        | TCP       | IP      | IEEE 802.11 | TP Cat 6   |
| FTP         | UDP       | Routing | Ethernet    | bit coding | 
c) What layer in the Internet protoco l stack is responsible for the transfer of a data packet over a single link, between two directly connected devices?  
Link layer.
*d) A router has two main functions, which can be described by the two terms “routing” and “forwarding”. What is the difference between routing and forwarding?*
Routing refers to the address translation performed in order to determine the correct destination address for the packet. Forwarding is the actual relay of the data packet to the destination address.
*e) What service does the transport layer describe? Give a short answer*
Service of the transfer of data from one host to another.