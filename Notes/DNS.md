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


