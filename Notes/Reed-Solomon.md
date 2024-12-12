---
title: "Reed-Solomon"
date: 2024-11-11
---
# Reed-Solomon
I am not too sure but this is the RS encoder used in 1000 Base T1.

Apparently, "The code encodes 406 information symbols and adds 44 parity symbols, enabling correction of up to 22 symbol errors. The code is systematic, meaning that the information symbols are not disturbed in any way in the encoder and the parity symbols are added separately to each block." , whatever that means.

First generate this thing
![](Pics/Pasted%20image%2020241111164819.png)
Then, create the parity vector for error correction with this
![](Pics/Pasted%20image%2020241111165214.png)