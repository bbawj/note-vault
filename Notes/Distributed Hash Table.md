---
title: "Distributed Hash Table"
date: 2023-01-19
---
# Distributed Hash Table
A DHT is a distributed P2P database. 
- Each entry is a key-value pair (host, IP address).
- A peer queries the DHT with key, and it returns the value matching that key
- Peers can also insert pairs
For example, it is used in [BitTorrent's distributed tracker](Notes/BitTorrent.md), where the key is a torrent identifier and the value is the set of IP addresses in the torrent.
## Hash Table
![500](https://i.imgur.com/nQtZPyc.png)
## Distributed (assigning keys to peers)
Pairs are evenly distributed among peers, with each peer only knowing a small number of other peers. To resolve a query, a small number of messages are exchanged to obtain the value.
![500](https://i.imgur.com/CZvwsKq.png)
## Circular DHT
Each peer is only aware of its immediate successor and predecessor.
![](https://i.imgur.com/9ayZnUh.png)
Average of $N/2$ messages needed.
### Shortcuts
![](https://i.imgur.com/wXqTBNe.png)
### Peer churn
![](https://i.imgur.com/V1nQ9SS.png)
