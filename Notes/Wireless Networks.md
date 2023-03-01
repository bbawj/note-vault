---
title: "Wireless Networks"
date: 2022-12-03
---
# Wireless Networks
## Network characteristics and its effects
- Decreasing signal strength as signal passes through matter
- Interference from other sources
- Multipath propagation as portions of the signal reflect off surfaces, taking different paths between the sender and receiver causing the signal to be blurred
$$C=BW\times log_2(1+\frac{S}{N})$$
- C is channel capacity, which is the maximum information rate
- BW is bandwith in *Hz*
- S is signal and N is noise in *watts*
### Bandwidth
Wireless communications run based on electromagnetic waves, and the bandwidth is the frequency range over which this communication can occur. *e.g. the 802.11b and 802.11g standards use the 2.4-2.5GHz band across all WiFi devices*.

Higher frequencies can transfer more information (see equation above), but come at the cost of lower range as signals cannot travel as far.
### Signal Power to Noise Power Ratio
The larger the amount of background noise, the stronger the signal has to be to carry the information. Increasing SNR can be done in 2 ways, increasing the transmission power, or reducing the distance between receiver and transmitter
#### Problems
- Near-far problem: louder signals crowd out weaker signals
- Cell-breathing: more signals result in greater interference and shrinks the effective range of a signal
### Bit Error Rate
Bit error rate varies according to the bit transmission rate and SNR. This means that as SNR changes, it would be good to support dynamic selection of the modulation technique (bit transmission rate) to adapt to the channel conditions.
![400](https://i.imgur.com/qQgP6rZ.png)
### Hidden terminal problem and fading
![](https://i.imgur.com/0xUre1l.png)
Signals are not strong enough to be detected at each individual source, but end up interfering at each other at the destination (B).
## Wireless LANs: WiFi 802.11
Infrastructure (use of access points) wireless LAN architecture, connecting basic service sets:
![500](https://i.imgur.com/VlS0YqC.png)
Each AP is given a unique MAC address for its interface, similar to [Ethernet](Notes/Link%20Layer.md#Ethernet).
### Channels and Association
- Each access point is assigned by the network admin a Service Set Identifier (SSID) (which also shows up as the names of the nearby APs on your device).
- 802.11 operates in a 85 Mhz band with 11 partially overlapping channels. Each AP is assigned by the network admin a channel number. 
To obtain information about an AP, the AP periodically sends beacon frames, each with the AP's SSID and MAC address
![](https://i.imgur.com/I3RRCWb.png)
### CSMA/CA Multiple Access Control Protocol
Similar to Ethernet's [Carrier Sense Multiple Access Collision Detection (CSMA/CD)](Notes/Link%20Layer.md#Carrier%20Sense%20Multiple%20Access%20Collision%20Detection%20(CSMA/CD)) but using collision avoidance instead.
- Not collision detection: [Hidden terminal problem](#Hidden%20terminal%20problem) makes it impossible to detect all collisions
- Collision avoidance: transmit the frame in entirety, but in a manner as to avoid collisions
![500](https://i.imgur.com/ztHgcnp.png)
#### Link-layer acknowledgements:
1. If idle, transmits frame after a short period of time known as the Distributed Inter-frame Space
2. Else, choose a random value using [Binary Exponential Backoff](Notes/Binary%20Exponential%20Backoff.md) and count down this value after DIFS while the channel is sensed idle. While it is busy, the counter is frozen. In 2 competing senders, they will hopefully choose a different backoff value, causing the "winning" sender to transmit first. The "loser" will hear the winner's signal, freeze its counter and refrain from transmitting until the winner has completed.
3. When counter = 0, transmit entirely and wait for ACK
4. If ACK not received, retransmit from step 2 with increased value
#### Clear to Send (CTS) / Request To Send (RTS):
*If 2 senders are out of range of each other, they would not be able to freeze their counter value.*
1. Sender sends a RTS frame to the AP with the total time required to transmit the data and acknowledgement frame
2. AP responds by broadcasting a CTS, allowing sender permission to transmit while instructing others not to.
### 802.11 Frame
![](https://i.imgur.com/AGgKYok.png)
Note 4 address fields compared to just source/receiver:
- Address 1: MAC address of receiver
- Address 2: MAC address of transmitting station
- Address 3: MAC address of router interface for the router which connects the subnet in the BSS to the internet
	- Router encapsulates data frame into an Ethernet frame with source/destination and sends it to an AP
	- AP converts the ethernet frame to 802.11 frame with source/destination, but adds the router MAC address to the address 3
	- Receiver can now use address 3 as the destination MAC address for sending data (and the router can then deconstruct it and forward the packet to the destination in the IP datagram)
## Wireless Personal Area Networks (WPANs)
Low power, short range low rate technology
### Bluetooth
A network established without network infrastructure in a master-slave configuration. Channel is partitioned using [TDM](Notes/Link%20Layer.md#Channel%20Partitioning%20Protocols) and the channel is changed in a pseudo random manner from time slot to slot, known as frequency hopping spread spectrum (FHSS)
### Zigbee
For even lower powered, lower data rate applications than Bluetooth such as home temperature and light sensors.
## Cellular Internet Access
WiFi 802.11 access points have a small coverage area. To combat this, cellular networks extended beyond voice communication to allow wireless internet connection.
### 2G: providing voice communication
Service area is partitioned into cells:
![](https://i.imgur.com/CxObifd.png)
BSC: allocates BTS radio channels to subscribers, finds the cell which a mobile user is in and perform handoffs.
MSC: performs user authorisation and accounting, call establishment, teardown and handoff.
### 3G: extending to support data
![](https://i.imgur.com/pVMox49.png)
New cellular network operates in parallel with the voice network.
### 4G: all-IP core network
Both voice and data are carried in IP datagrams rather than on separate networks
![](https://i.imgur.com/22jLkMt.png)
