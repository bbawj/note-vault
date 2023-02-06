---
title: "Internet Protocol"
date: 2022-11-08
lastmod: 2022-11-21
---
# Internet Protocol (IP)
A protocol designed to deliver datagrams from the source to destination host based on their addresses. It provides host-to-host routing and addressing.
## IPv4
![](https://i.imgur.com/ZZbq7uu.png)
### Datagram Fragmentation
Not all link-layer protocols can carry network layer packets of the same size. For example, Ethernet frames can carry up to 1500 bytes while wide area links can carry no more than 576 bytes. This maximum amount of data is called **Maximum Transmission Unit** (MTU).
![](https://i.imgur.com/ZTq36MP.png)
When the router determines that the outgoing link has an MTU that is smaller than the length of the IP datagram:
1. The payload is fragmented into 2 or more pieces. Each fragment shares the same identification number, but have different fragment offset bits
2. Using this information, end systems can identify fragmented packets and piece them back together.
### Addressing
A host typically has 2 links (Ethernet and wireless 802.11). A router has multiple links. Each boundary between a host/router and the physical link is an *interface*, and each interface needs its own unique IP address.
#### Subnets
The isolated islands of network interfaces formed when disconnected from the router/host is a subnet. A topology with 6 subnets:
![500](https://i.imgur.com/Ka0I6dZ.png)
- Example: the subnet address 223.1.2.0/24 indicates a 24 bit subnet mask which says that the leftmost 24 bits define the subnet address.
![](https://i.imgur.com/RASHKku.png)
## Obtaining an IP address
An organisation can request for a block of IP addresses from an ISP. An ISP can split up its own allocated block in this way:
![](https://i.imgur.com/ggSAoym.png)
The ISP itself obtains its set of IP addresses through a global authority called Internet Corporation for Assigned Names and Numbers (ICANN).
## Dynamic Host Configuration Protocol (DHCP)
Once an organisation has obtained a block of addresses, it can assign individual addresses to the host an router interfaces. This can be done manually, but DHCP allows the host to obtain an IP address automatically from a DHCP server.
![400](https://i.imgur.com/klcMGOm.png)
1. Discover: client sends a discover message with UDP on port 67 to the broadcast address 255.255.255.255 since it does not know the IP address of the network. 
2. Offer: server broadcasts a message (on 255.255.255.255) containing transaction ID of the received message, proposed IP address and IP address lease time
3. Request: client chooses from 1 or more offers and responds
4. ACK: server responds to the request message
## IPv6
The IPv4 32 bit address space was beginning to be used up. In February 2011, IANA allocated out the last remaining pool of unassigned IPv4 addresses.
### Datagram
![](https://i.imgur.com/nNQk2xY.png)
- Address: 128 bit address space
- Flow label: identify datagrams of the same "flow" (maybe such as audio or video transmission)
- Next header: identifies the protocol which the contents of the data will be delivered (to TCP or UDP)
- Hop limit: decremented each time each router forwards the datagram (discarded at 0)
Some fields from the IPv4 datagram were also removed:
- Fragmentation: IPv6 does not allow for fragmentation and reassembly at intermediate routers. This can only be done at the source and destination. If the datagram is too large for the outgoing link, it is dropped and the sender is asked to resend using a smaller datagram size.
- Checksum: removed to reduce processing time
- Options: moved out of the headers portion

