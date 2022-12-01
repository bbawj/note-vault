---
title: "Transport Layer Security"
date: 2022-11-21
lastmod: 2022-11-21
---
# Transport Layer Security
A protocol designed to provide encryption, authentication and data integrity. It is a standardisation over [[SSL]]. 
- Encryption: obfuscate what is sent from one computer to another
- Authentication: verify the validity of provided identificatin
- Integrity: detect message tampering and forgery
![](https://i.imgur.com/0124CY8.png)
## TLS Handshake
![500x500](https://i.imgur.com/griUQzm.png)
1. TLS runs ontop of an existing TCP connection. Hence, establishing a TLS handshake requires going through the full round trip to set up a [three-way handshake](Notes/Transmission%20Control%20Protocol.md#Three-way%20Handshake) first.
2. The client sends a number of specifications in plain text such as the version of TLS and list of supported ciphersuites
3. The server picks the TLS protocol version, decides the ciphersuite, attaches its certificate and sends the response back
4. Client generates a new symmetric key and encrypts it with the servers public key. **Up until now, the data has been exchanged in clear text with exception of the new symmetric key**.
5. Server decrypts the symmetric key, checks the message integrity by verifying the MAC and returns an encrypted *Finished*.
The entire process can add a lot of extra latency!
## Session Resumption
One way to reduce the extra latency, is to add ways to share the same negotiated secret key data between connections.
### Session ID
A s