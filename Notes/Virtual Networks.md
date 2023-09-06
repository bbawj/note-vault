---
title: "Virtual Networks"
date: 2023-09-03
lastmod: 2023-09-03
---
# TUN/TAP
Kernel network drivers used for tunneling

TUN and TAP can't be used together because they transmit and receive packets at different layers of the network stack. 
- TUN, namely [network TUNnel](https://en.wikipedia.org/wiki/Tunneling_protocol "Tunneling protocol"), simulates a [network layer](https://en.wikipedia.org/wiki/Network_layer "Network layer") device and operates in layer 3 carrying [[Notes/Internet Protocol.md|IP]]  packets. 
- TAP, namely [network TAP](https://en.wikipedia.org/wiki/Network_tap "Network tap"), simulates a link layer device and operates in layer 2 carrying [Ethernet](Notes/Link%20Layer.md#Ethernet) frames. TAP can be used to create a [user space](https://en.wikipedia.org/wiki/User_space "User space") [network bridge](https://en.wikipedia.org/wiki/Network_bridge "Network bridge")
Packets sent by an operating system via a TUN/TAP device are delivered to a user space program which attaches itself to the device. A user space program may also pass packets into a TUN/TAP device. In this case the TUN/TAP device delivers (or "injects") these packets to the operating-system network stack thus emulating their reception from an external source.