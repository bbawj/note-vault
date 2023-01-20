---
title: "BitTorrent"
date: 2023-01-19
---
# BitTorrent
The collection of all peers participating in the distribution of a particular file is called a torrent. 
- Peers in a torrent download equal-size chunks of the file from one another, with a typical chunk size of 256 kbytes. 
- When a peer first joins a torrent, it has no chunks. Over time it accumulates more and more chunks. 
- While it downloads chunks it also uploads chunks to other peers. Once a peer has acquired the entire file, it may (selfishly) leave the torrent, or (altruistically) remain in the torrent and continue to upload chunks to other peers.
![](https://i.imgur.com/X7tgf7i.png)
The tracker keeps track of the peers that are participating in the torrent. A user will receive a subset of peers from the tracker of which they will establish concurrent TCP connections. These are the *neighbouring peers*.
## Requesting Chunks
1. Alice issue requests for the list of chunks neighbouring peers have.
2. Alice will use **rarest first** to determine the chunks that are the rarest among the neighbours and then request those chunks first
## Sending Chunks (tit-for-tat)
An incentive based trading algorithm:
1. For each neighbour, continually measure the rate which she receive bits and determine the peers which are sending at the highest rate.
2. Send chunks to these **unchoked peers**.
3. Every 10 seconds, recalculate the rates
4. Every 30 seconds, pick one additional neighbour at random and send it chunks, optimistically unchoking this peer.
> [! Consider Alice and an optimistically unchoked Bob]
> If the rate Alice is sending data to Bob is high enough, she may become one of Bob's top uploaders. In which case, Bob will begin to send data to Alice. If the rate Bob sends data is high enough, he might become Alice's top uploaders. 
> 
> *The effect is, peers capable of uploading at compatible rates tend to find each other.*
> 
> The random neighbour selection allows new peers to get chunks so that they can have something to trade.
> 
> All other peers are choked and do not receive chunks

