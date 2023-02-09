---
title: "Application Layer"
date: 2023-01-18
lastmod: 2023-01-19
---
# Application Layer
## Network Application Architectures
![500](https://i.imgur.com/9xLCsuj.png)
Examples:
- Webmail
![500](https://i.imgur.com/mzwgHPb.png)
Examples:
- File sharing
- Bittorrent
### CS vs P2P File Distribution
![500](https://i.imgur.com/0pCgt40.png)
#### Client Server
- The server must transmit one copy of the file to each of the N peers. Thus the server must transmit NF bits. Since the server’s upload rate is us, the time to distribute the file must be at least NF/us.
- Let $d_{min}$ denote the download rate of the peer with the lowest download rate, that is, $d_{min} = min \{d1, dp, . . . , dN\}$. The peer with the lowest download rate cannot obtain all F bits of the file in less than $F/d_{min}$ seconds. Thus the minimum distribution time is at least $F/d_{min}$.
$$D_{CS} \ge max\{\frac{NF}{u_s},\frac{F}{d_{min}}\}$$
From this we can observe that distribution time increases linearly with the number of peers N.
#### P2P
- To get this file into the community of peers, the server must send each bit of the file at least once into its access link. Thus, the minimum distribution time is at least F/us.
- The peer with the lowest download rate cannot obtain all F bits of the file in less than $F/d_{min}$ seconds. 
- The total upload capacity of the system as a whole is equal to the upload rate of the server plus the upload rates of each of the individual peers, that is, $u_{total} = u_s + u_1 + ... + u_N$. The system must upload F bits to each of the N peers, thus delivering a total of NF bits. The minimum distribution time is also at least $NF/(u_s + u_1 + ... + uN)$.
![500](https://i.imgur.com/DwxC3nt.png)
## Process Communication
Network applications on different hosts need a way to communicate with each other (sometimes across different operating systems). 
Client: process that initiates the communication
Server: the other part of the pair
### Addressing Processes
To receive messages, a process must have an identifier. Each host has a unique IP address but this is not enough as there are many processes which can be running on the same host. A **port number** is needed to identify the receiving process/socket:
HTTP server: 80
Mail server: 25
### Transport Service Requirements
1. Data integrity: the amount of fault tolerance an application needs
2. Throughput: rate which sending process can deliver bits to receiver. Because communication lines are shared, some bandwidth-sensitive applications (such as multimedia) may need a set throughout value.
3. Timing: the amount of latency. An example guarantee might be that every bit that the sender pumps into the socket arrives at the receiver’s socket no more than 100 msec later. Such a service would be appealing to interactive real-time applications, such as multiplayer games.
4. Security: encryption
## Application Layer Protocols
An application layer protocol defines:
- The types of messages exchanged, for example, request messages and response messages
- The syntax of the various message types, such as the fields in the message and how the fields are delineated
- The semantics of the fields, that is, the meaning of the information in the fields 
- Rules for determining when and how a process sends messages and responds to messages
### HTTP
The Web's application layer protocol is [HTTP](Notes/HTTP.md).
### Electronic Mail
A typical message starts its journey in the sender’s user agent, travels to the sender’s mail server, and travels to the recipient’s mail server, where it is deposited in the recipient’s mailbox. When Bob wants to access the messages in his mailbox, the mail server containing his mailbox authenticates Bob (with usernames and passwords)
![](https://i.imgur.com/iVbsCx3.png)
Each user agent uses a separate mail server rather than directly connecting with each other such that there is some recourse (able to keep retrying to send a message) when the destination is currently unreachable.
![](https://i.imgur.com/jm02rCR.png)
#### SMTP
The heart of Internet electronic mail is [[SMTP]], which allows for the transfer of messages.
#### Mail Access Protocol
SMTP is a push protocol. Mail access protocols are needed to retrieve mail from the mail server via a pull operation:
- [[POP3]]
### DNS
Many application protocols are built on top of [[DNS]].
### BitTorrent
The most popular P2P protocol for file distribution is [[BitTorrent]]
### Distributed Hash Table
Another application of P2P is a [[Distributed Hash Table]]
## Socket Programming
How are network applications actually created? Processes running on different machines communicate with each other through sockets.
![](https://i.imgur.com/m6lthQk.png)
### TCP
Network applications may communicate through TCP, and hence the connection socket must support TCP. TCP provides a reliable **byte-stream** service:
![](https://i.imgur.com/2CO9GiP.png)
Basic byte I/O classes in Java
![](https://i.imgur.com/zpjfz02.png)
#### Client
The client must perform the following operations:
1. Open TCP connection to the server
2. Send data
3. Receive data on the connection
4. Close the connection
![](https://i.imgur.com/nisae4A.png)
#### Server
![](https://i.imgur.com/WrmIrHf.png)
![](https://i.imgur.com/AfPgXhj.png)

![](https://i.imgur.com/qqHEpRB.png)
### Encoding/Decoding
To transfer a string between two processes over the network, we must decide how to represent the string as a sequence of bytes.
#### ASCII
![](Pics/Pasted%20image%2020230120000437.png)
#### UTF-8
- Unicode Transformation Format – 8-bit  
- Variable length encoding  
- Up to four bytes per symbol  
- The first 128 are the same as for ASCII  
- Backwards compatibility – ASCII text is also valid UTF-8  
- Dominating format on the Web
![](https://i.imgur.com/nQ6XDpH.png)
### Helpful Java classes
![](https://i.imgur.com/puWh81X.png)

![](https://i.imgur.com/kudOIAw.png)
## Exercises
![](https://i.imgur.com/8tknIWb.png)
$$
\begin{align}
&\text{First packet delay} = N*(L/R)\\
&\text{The 2nd packet must wait for the first packet to reach the 2nd router before it is sent}:\\
&\text{2nd packet delay} = N*(L/R)+(L/R)\\
&...\\
&\text{Pth packet delay} = N*(L/R)+(P-1)*(L/R)\\
&\text{All packets sent after Pth delay}:\\
&d_{end-to-end} = (N+P-1)*(L/R)
\end{align}
$$
![](https://i.imgur.com/FjVNvWQ.png)
a. $d_{prop}=m/s$
b. $d_{trans}=L/R$
c. $d_{end-to-end}=d_{prop}+d_{trans}$
d. At the start of the link
e. In the link
f. At the destination
g. 
$d_{trans}=\frac{120}{56\times10^3}=0.00214s$
$0.00214=m/2.5\times10^8$
$m=535.7km$
![](https://i.imgur.com/Oq5N82i.png)
$$
\begin{align}
&\text{All bits must be generated before it can be grouped into packets}:\\
d_{generate}&=56\times8/(64\times10^3)=7ms\\
d_{trans}&=56\times8/(2\times10^6)=0.000224s\\
d_{prop}&=0.01s\\
Total&=17.224ms
\end{align}
$$
![](https://i.imgur.com/fJ0zOXS.png)
![](https://i.imgur.com/IobqAEp.png)
$$
\begin{align}
d_{trans}=(1500\times8\times2)/(4\times10^6)=0.00075s\\
\end{align}
$$

