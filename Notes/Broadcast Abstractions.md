---
title: "Broadcast Abstractions"
date: 2023-02-02
---
# Broadcast Abstractions
## Unreliable Broadcast
Does not guarantee anything. Such events are allowed:
![](https://i.imgur.com/rgh87f2.png)
## Best Effort Broadcast
Guarantees reliability only if sender is correct
- BEB1. Best-effort-Validity: If pi and pj are correct, then any broadcast by pi is eventually delivered by pj  
- BEB2. No duplication: No message delivered more than once  
- BEB3. No creation: No message delivered unless broadcast
![](https://i.imgur.com/LdLrtA0.png)
### Implementation
We can use perfect links:
Upon <beb Broadcast | m>send message m to all processes (for-loop)  
Correctness  
- If sender doesn’t crash, every other correct process receives message by perfect channels (Validity)  
- No creation & No duplication already guaranteed by perfect channels
![](https://i.imgur.com/qLc7YaJ.png)
## Reliable Broadcast
BEB gives no guarantees if sender crashes. Reliable strengthens this by giving guarantees even if sender crashes.
- RB1 = BEB1. Validity  
- RB2 = BEB2. No duplication  
- RB3 = BEB3. No creation  
- RB4. Agreement. If a correct process delivers m, then every correct process delivers m
![](https://i.imgur.com/sqciAfm.png)
### Fail Stop (Lazy) Implementation
![](https://i.imgur.com/96xQG2C.png)
- Perfect failure detector (P): use this to detect when process crash
- BEB: use this to redistribute messages when detect a crash from a process
Case 1: detect crash and redistribute
Case 2: delivered message, detect crash and redistribute
![](https://i.imgur.com/ubwflEO.png)

![](https://i.imgur.com/aye5k1z.png)
#### Performance
Message complexity: best case O(N), worst case O(N^2)
Time complexity: best case 1 round. worst case 2 rounds
### Fail Silent (Eager)
No failure detector necessary. A pessimistic approach that just redistributes any message by assuming that the process has failed.
![](https://i.imgur.com/pzOVFg9.png)
## Uniform Reliable Broadcast
Reliable broadcast creates a problem. If a failed process delivers a message that has a side effect (such as withdrawing some money from an account), the correct processes need not deliver (know of) this side effect.
- URB1 = RB1.  
- URB2 = RB2.  
- URB3 = RB3.  
- URB4. Uniform Agreement: For any message m, if a process delivers m, then every correct process delivers m
### Fail Stop
![](https://i.imgur.com/TzK7KCV.png)
![](https://i.imgur.com/6A4OvxA.png)
![](https://i.imgur.com/omFKzDs.png)
### Fail Silent
Correctness assumption: a majority of processes are always correct. Resilience is N/2 machines can fail
![](https://i.imgur.com/SpK4Poo.png)
## Causal Broadcast
- Causality between broadcast events is preserved by the corresponding delivery events  
- If broadcast(m1) happens-before broadcast(m2), any delivery(m2) cannot happen-before a delivery(m1)  
![](https://i.imgur.com/vJe21OC.png)
### Fail Silent
Each broadcasted messages carries a history which can be used to ensure causality before delivery. The history is an ordered list of casually preceding messages in the past.
![](https://i.imgur.com/exeReIx.png)
![](https://i.imgur.com/QMyVWOR.png)
![](https://i.imgur.com/NRf5PFr.png)
### Fail Silent waiting
![](https://i.imgur.com/S4ldZ4l.png)
![](https://i.imgur.com/9Gs9Gfb.png)

![](https://i.imgur.com/7Vkg3Ka.png)

![](https://i.imgur.com/w6wZvDJ.png)
![](https://i.imgur.com/FtmlotO.png)
