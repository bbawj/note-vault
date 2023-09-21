---
title: OS Security
date: 2023-09-21
---
![](Pics/Pasted%20image%2020230921121212.png)
# Users & Access Control
A system can have many accounts. 16 bit UIDs are used to identify accounts, e.g. root(0), bin(1). An attacker managing to get root user status effectively takes over the entire system.
![](Pics/Pasted%20image%2020230921135811.png)
## Groups
![](Pics/Pasted%20image%2020230921135210.png)
## Process  Controlled Invocation
Processes are assigned 2 types of user and group IDs.
![](Pics/Pasted%20image%2020230921140026.png)
Root user privilege is required to execute certain OS functions, e.g. password changing, which requires R/W access to `/etc/shadow`. The OS overcomes this by giving additional permissions via a SUID:
![](Pics/Pasted%20image%2020230921140143.png)
By tricking the SUID program owned by root to do unintended things, an attacker can act as root. Hence, all user input must be processed with extreme care, programs should have SUID status only if it is really necessary.
### RootKit
Malware that obtains root privileges. It originally referred to a set of maliciously modified set of administrative tools which granted root access. Once granted, uses can be as follows:
#### Hijacking sys-call tables
![](Pics/Pasted%20image%2020230921141520.png)

![](Pics/Pasted%20image%2020230921141434.png)
#### Hijacking [Interrupt Stack Table (IST)](Notes/Interrupts.md#Interrupt%20Stack%20Table%20(IST)%20and%20Task%20State%20Segment%20(TSS))
![](Pics/Pasted%20image%2020230921142335.png)