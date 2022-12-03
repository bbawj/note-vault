---
title: "Wireless Networks"
date: 2022-12-03
---
# Wireless Networks
## Performance Fundamentals
$$C=BW\times log_2(1+\frac{S}{N})$$
- C is channel capacity, which is the maximum information rate
- BW is bandwith in *Hz*
- S is signal and N is noise in *watts*
### Bandwith
Wireless communications run based on electromagnetic waves, and the bandwith is the frequency range over which this communication can occur. *e.g. the 802.11b and 802.11g standards use the 2.4-2.5GHz band across all WiFi devices*.

Higher frequencies can transfer more information (see equation above), but come at the cost of lower range as signals cannot travel as far.
### Signal Power to Noise Power Ratio
The larger the amount of background noise, the stronger the signal has to be to carry the information. Increasing SNR can be done in 2 ways, increasing the transmission power, or reducing the distance between receiver and transmitter
#### Problems
- Near-far problem: louder signals crowd out weaker signals
- Cell-breathing: more signals result in greater interference and shrinks the effective range of a signal

