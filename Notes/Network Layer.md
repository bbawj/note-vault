---
title: "Network Layer"
date: 2023-01-21
lastmod: 2023-02-10
---
# Network Layer
The network layer offers **logical communication between hosts**, which is distinct from the [logical communication from the Transport Layer, which is within the host itself](Notes/Transport%20Layer.md). The network layer provides 2 important functions:
- Forwarding: move packets from router input to appropriate output based on a **forwarding table**
- Routing: use routing algorithms to determine the route for the packet to take 
![](https://i.imgur.com/lzfb7Cm.png)
Routers in the network topology communicate with one another using routing packets in order to determine the best route. This information is populated in the forwarding table.
## Forwarding Table
In the case of a 32 bit IP address, a brute force implementation of the forwarding table would have 1 entry for each of the 4 billion possible addresses. To make things scale, forwarding tables use ranges instead and match based on the longest prefix for which interface to forward to:
![](https://i.imgur.com/r0DQ9qq.png)
## Internet Protocol (IP)
The [Internet Protocol](Notes/Internet%20Protocol.md) handles addressing conventions, datagram format and packet handling conventions.
## Network Address Translation (NAT)
[Network Address Translation](Notes/Network%20Address%20Translation.md) allows local home subnets to grow bigger without having to request for additional address blocks from the ISP.
## Internet Control Message Protocol (ICMP)
![](https://i.imgur.com/LNEfMNs.png)

![](https://i.imgur.com/CBk42YD.png)
## Routing Algorithms
The goal of the routing algorithm is to determine the least cost path to be populated in  the forwarding table.
What is the "least cost"?:
- Number of hops
- Bandwidth
- Delay
- Cost/Load
### Classifications
- Centralised: algorithm has complete information about connectivity and link costs (link-state algorithms)
- Decentralised: no node has complete information about all network links. Each node exchanges information with its neighbours.
- Static: routes change very slowly over time
- Dynamic: routes change as the network traffic loads or topology change
### Link-State Algorithm
Each node is able to have the identical and complete view of the network by broadcasting link-state packets to all other nodes in the network. With this, [Dijkstra's Algorithm](Notes/Dijkstra's%20Algorithm.md) can be used to find the least cost paths.
### Distance Vector Algorithm
Let $d_x(y)$ be the cost of the least-cost path from node x to node y. Then the least costs are related by the Bellman-Ford equation, namely, $dx(y) = min_v \{c(x, v) + dv( y)\}$, which takes the minimum across all neighbours v to x, of the sum of x to v and v to y.
[[Bellman-Ford Algorithm]]
![](https://i.imgur.com/wAdh2pW.png)
Each node x begins with $D_x(y)$, an estimate of the cost of the least-cost path from itself to node y, for all nodes, y, in N. Let Dx = [Dx(y): y in N] be node x’s distance vector, which is the vector of cost estimates from x to all other nodes, y, in N. Each node x maintains the following routing information:
- For each neighbour v, the cost c(x,v) from x to directly attached neighbour, v 
- Node x’s distance vector, that is, Dx = [Dx(y): y in N], containing x’s estimate of its cost to all destinations, y, in N 
- The distance vectors of each of its neighbours, that is, Dv = [Dv(y): y in N] for each neighbour v of x
![](https://i.imgur.com/zQ1kAKC.png)

![](https://i.imgur.com/ymde8HZ.png)
#### Link cost change
![](https://i.imgur.com/TaeDW6Q.png)
a. "Good news travels fast":
- t0: y detect the cost change (4 to 1), updates its DV and sends it to x and z
- t1: z receives the update and recompute its DV. $D_zx=2$
- t2: y receives update from z, but computation results in the same distance vector. Stability is achieved in 2 iterations.
b. "Bad news travels slow":
- t0: y detects cost change (4 to 60) and updates its DV according to $D_y(x)=min\{c(y,x)+D_x(x), c(y,z) + D_z(x)\}=min\{60+1, 1+5\} = 6$
  This is wrong as $D_z(x)\ne5=50$
- t1: z receives an update from y and computes its DV. $D_z(x)=min\{50+0, 1+6\}=7$
- t2: y receives an update from z and recomputes its DV. $D_y(x)=min\{60+0, 1+7\}=8$
This routing loop is because in order to get to x, y routes through z. But in order to get to x, z goes through y. This continues on and on for 44 iterations.
#### Poisoned reverse
If z routes through y to get to destination x, then z will advertise to y that its distance to x is infinity, that is, z will advertise to y that Dz(x) = ∞ (even though z knows Dz(x) = 5 in truth). z will continue telling this little white lie to y as long as it routes to x via y. Since y believes that z has no path to x, y will never attempt to route to x via z, as long as z continues to route to x via y (and lies about doing so).

*Loops involving 3 or more nodes will not be detected by this technique*
![](https://i.imgur.com/R9WakJH.png)
#### Routing Information Protocol (RIP)
RIP uses the distance vector algorithm with its metric being the number of hop counts. 
![](https://i.imgur.com/cdaoCkz.png)
## Intra-AS: Open Shortest Path First (OPSF)
### Autonomous Systems
In above routing algorithms, the model of the network of routers was too simplistic
- Scale: as number of routers become large, up to 600 million routers today, it would incur large amounts of overhead to store all destination routing tables. A DV algorithm iterated among them would surely never converge
- Administrative autonomy: each network admin may want to control the routing in its own network, such as which routing algorithm to use
This can be solved by organizing routers into autonomous systems (ASs), with each AS consisting of a group of routers that are under the same administrative control.
![](https://i.imgur.com/lc7ldXT.png)
Each AS needs to be unique, and is identified by its AS number, which is assigned by ICANN regional registries
### OSPF
![](https://i.imgur.com/ZwT5nz4.png)
OSPF uses IP directly, without UDP or TCP, meaning it has to implement functionality such as reliable message transfer.
Each router
1. Actively test the status of all neighbours/links  
2. Build a Link State Advertisement (LSA) from this information and propagate it to all other routers within an area.  
3. Using LSAs from all other routers, compute a shortest path delivery tree, typically using Dijkstra shortest path algorithm.
![](https://i.imgur.com/g59bgpK.png)
## Inter-AS: Border Gateway Protocol (BGP)
To route a packet across multiple ASs, we need an inter-autonomous system routing protocol. Since an inter-AS routing protocol involves coordination among multiple ASs, communicating ASs must run the same inter-AS routing protocol. In the Internet, all ASs run the same inter-AS routing protocol, the BGP.
![](https://i.imgur.com/HInOBzf.png)
Each router sends messages over these connections. For example, when a new subnet x appears in AS3, the gateway router in AS3 sends a message to AS2 "AS3 x". AS2 then uses iBGP to advertise the existence of x amongst internal AS2 routers before sending a message over eBGP "AS2 AS3 x" to AS1.