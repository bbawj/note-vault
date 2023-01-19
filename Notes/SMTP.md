---
title: "SMTP"
date: 2023-01-18
---
# SMTP
Simple Mail Transfer Protocol transfers messages from senders' mail servers to receiver mail servers.![](https://i.imgur.com/B29ExA6.png)
> [!Note]
> SMTP does not normally use intermediate mail servers, even when two mail servers are located on opposite ends of the world. The TCP connection is a direct connection between client and server
1. Client SMTP establishes a TCP connection to port 25 at the server SMTP
2. SMTP Handshake
3. Message sent using TCP
4. TCP connection close
![](https://i.imgur.com/UBhnR9y.png)
## Difference with HTTP
- SMTP is a push protocol (sending mail pushes the file to the server)
- SMTP requires each message to be in 7 bit ASCII format.
- All objects are placed in one message rather than having each object on its own response message
## Message Format
RFC 5322 specifies the exact format for mail header lines which are different from the commands used in SMTP (MAIL FROM etc.).
![400](https://i.imgur.com/W26fnOj.png)

