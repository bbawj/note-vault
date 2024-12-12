---
title: PAM
date: 2024-09-18
---
# PAM (Pulse Amplitude Modulation)
An encoding method using the amplitude of a series of signal pulses to refer to specific bit values.
![](Pics/Fig1-1_1-2_3.png)
## Waveform
![](Pics/Pasted%20image%2020240923102831.png)
PAM2 looks like there is more than 2 levels, but that's because of signal loss, etc. so we just have a threshold. anything above 0 is +1, -1 vice versa
- consecutive H/L signals would stack and form larger peaks (the purpose of the scrambler is to avoid such consecutive values)
- amplitude can also be diminished due to channel resistance
- a square wave is actually bad because it creates higher EMI. 
## 3B2T and friends
This refers to encoding 3 bits into 2 ternary values. Using PAM, we can set a ternary with amplitudes {-1, 0, +1}. We thus have 8 binary values to encode using 9 output waveforms.
![](Pics/Pasted%20image%2020240918112624.png)