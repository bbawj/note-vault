---
title: "Application Layer"
date: 2023-01-18
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
[[DNS]]