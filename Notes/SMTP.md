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
- HELO and QUIT is performed once
- MAIL FROM, RCPT TO, DATA, actual message is performed as many times as there are different messages.
- Same message can have multiple RCPT TO
- Each command takes 1 round trip of network delay: in the above a total of 6 round trips is needed.
## Difference with HTTP
- SMTP is a push protocol (sending mail pushes the file to the server)
- SMTP requires each message to be in 7 bit ASCII format.
- All objects are placed in one message rather than having each object on its own response message
## Message Format
RFC 5322 specifies the exact format for mail header lines which are different from the commands used in SMTP (MAIL FROM etc.).
![400](https://i.imgur.com/W26fnOj.png)
## Exercises
Describe shortly how an email message in sent from a sending to a receiving email client (user agent), such as MS Outlook or Mozilla Thunderbird . From the description, it should be clear what application protocols and what systems are involved in the transfer.
User agent uses SMTP to send the email message to the user's mail server. The message is placed on a message queue to be sent. The user mail server uses SMTP to send the email message to the receiver mail server.

An email message is sent from an email client (MUA) to a server for outgoing email .  
a) Which protocol is used for the transfer ?  
SMTP
b) The communication delay between the client and the server is 2.5 milliseconds ( in other words, it takes 2.5 milliseconds from that the sender has started sending the message until the complete message has reached the receiver ) . How long time does it take before the entire transfer of the email message to the outgoing server has been completed ? The time it takes to set up and tear down a TCP connection should not be included in the calculation. The message has one recipient.  
$2.5 \times 6 \times 2 = 30ms$
c) Suppose that the message has four recipients . How long time does it take then ?
$(2.5\times2\times2) + (2.5\times2\times7) =45ms$