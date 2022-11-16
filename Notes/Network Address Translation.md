---
title:"Network Address Translation"
---
# Network Address Translation
IPv4 addresses are only 32 bits long which provides a maximum of 4.29 billion unique IP addresses.
## Solution
Introduce NAT devices at the edge of the network, each of which would be responsible for maintaining a table mapping of local IP and port tuples to one or more globally unique (public) IP and port tuples (Figure 3-3). 
![](https://i.imgur.com/S5Ys071.png)
## NAT Traversal
Problems while using a NAT
1. Client may not know the public IP address: if the client communicates its private IP address as part of its application data with a peer outside of its private network, then the connection will inevitably fail.
2. NAT table may not have the mapping of a public IP of a packet
### Session Travel Utilities for NAT (STUN)
Helps the application obtain the public IP and port tuple of the current connection:
![](https://i.imgur.com/PWoE1Ij.png)
1. Discover the IP address and port tuple for the connection
2. Establish the NAT routing entry for the host application
3. Keepalive pings to keep NAT entries from timing out
### Traversal Using Relays around NAT (TURN)
When STUN fails (blocked by firewall etc.), we can use a relay to transmit the data between peers.
![](https://i.imgur.com/g1gHsly.png)
### Interactive Connectivity Establishment (ICE)
Establishes a set of methods to find the most efficient tunnel between participants: direct connection where possible using STUN if needed and TURN if failure.
