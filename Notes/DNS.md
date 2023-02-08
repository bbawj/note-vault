---
title: "DNS"
date: 2023-01-18
---
# DNS
Domain Name System is a distributed database implemented in a hierarchy of DNS servers, and an application layer protocol that allows hosts to query the database.
## Features
### Address Translation
To identify a host, people prefer the mnemonic hostname identifier such as google.com. However, routers prefer fixed length hierarchically structured IP addresses. DNS's job is to provide the translation between these two references.

DNS being employed by HTTP:
1. The same user machine runs the client side of the DNS application. 
2. The browser extracts the hostname, www.someschool.edu, from the URL and passes the hostname to the client side of the DNS application. 
3. The DNS client sends a query containing the hostname to a DNS server. 
4. The DNS client eventually receives a reply, which includes the IP address for the hostname. 
5. Once the browser receives the IP address from DNS, it can initiate a TCP connection to the HTTP server process located at port 80 at that IP address
### Host aliasing
DNS can be invoked by an application to obtain the **canonical hostname** for a supplied alias hostname. A host with a complicated hostname can have one or more alias names. For example, a hostname such as relay1.west-coast .enterprise.com could have two aliases such as enterprise.com and www.enterprise.com. The hostname relay1 .west-coast.enterprise.com is said to be a canonical hostname.
### Load distribution
Busy sites can have different servers all replicating the same content, each having their own IP address. DNS stores the entire set of addresses, but is able to rotate their order with each reply. Because the client sends its HTTP request message to the first IP address, this performs load distribution.
## How it works
Rather than having 1 central DNS server which does not scale, DNS servers are distributed and organized in a hierarchical structure: Root -> Top Level Domain -> Authoritative.
![](https://i.imgur.com/ZZScm7J.png)
- Root: provides the IP address of TLD servers. There are 400 root name servers scattered across the world
- TLD: com, org, net etc. and all country TLD uk, sg etc. maintained by companies and countries.
- Authoritative: houses the DNS records of organization host IP addresses e.g. amazon.com. Can be done in house or outsourced to some service provider
- Local: close to the host which acts as a proxy, forwarding queries to the DNS server hierarchy
![400](https://i.imgur.com/sTi4w1K.png)
> [!Note]
> TLD server may not know directly the authoritative server address, but rather some other intermediate server. In this way, there could be 2 more DNS messages required

Query 1 is a recursive query, as it asks to obtain the mapping on its behalf. Subsequent queries are iterative as all replies are directly returned to local DNS server. This is the more typical scenario. There are also queries which are all recursive:
![400](https://i.imgur.com/d6WbQi8.png)
### DNS Caching
In a query chain, when a DNS server receives a DNS reply (containing, for example, a mapping from a hostname to an IP address), it can cache the mapping in its local memory. This DNS server can provide the desired IP address, even if it is not authoritative for the hostname. Because hosts and mappings between hostnames and IP addresses are by no means permanent, DNS servers discard cached information after a period of time (often set to two days).
## DNS Records
![](https://i.imgur.com/z4tkBPG.png)
### Inserting Records
A registrar is a commercial entity that verifies the uniqueness of the domain name, enters the domain name into the DNS database and collects a small fee.
Example registering domain name `networkutopia.com`, two records are inserted:
`(networkutopia.com, dns1.networkutopia.com, NS) 
`(dns1.networkutopia.com, 212.212.212.1, A)`
## DNS attacks
![](https://i.imgur.com/FMtlD4D.png)
## Exercises
![](https://i.imgur.com/WsP2Wtv.png)
![](https://i.imgur.com/PlRfgpa.png)
a.
1. Host sends request to local DNS server
2. Local DNS makes a query to the root DNS server
3. Root DNS returns the Top level domain DNS server for "com"
4. Local DNS makes query to TLD
5. TLD returns the authoritative name server for "fws.com"
6. Local DNS makes query to DNS server for "fws.com"
7. Authoritative DNS returns the IP address for "punchy.fws.com"
8. Local DNS returns this IP address to the host
b. Query 1 is recursive. The rest are iterative
c. Yes
![](Pics/Transmission%20Control%20Protocol%202023-02-07%2017.09.11.excalidraw.svg)
%%[🖋 Edit in Excalidraw](Pics/Transmission%20Control%20Protocol%202023-02-07%2017.09.11.excalidraw.md), and the [dark exported image](Pics/Transmission%20Control%20Protocol%202023-02-07%2017.09.11.excalidraw.dark.svg)%%


