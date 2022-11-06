# Signal Chain Subsystem
The signal chain subsystem is concerned about how hardware in computer systems communicate.
## Factors affecting signal transfer
### Signal Skew
When the signal of one or more data lines takes a different amount of time to reach the receiver, resulting in wrong data latched by the receiver.
![](https://i.imgur.com/X6ZZI0B.png)
### Cross Talk
The undesired coupling of signals from one circuit to another, resulting in electrical interference.
## Parallel Data Transfer
Multiple bits of data are transferred simultaneously between 2 devices.
![](https://i.imgur.com/iFX7iUd.png)
- Higher transfer rate
- Prone to signal skew and crosstalk
Synchronous data transfer only, as a strobe signal is needed to inform the receiver of when to latch into the data (e.g. rising edge of the signal).
## Serial Data Transfer
Data is transferred one bit at a time over a single data line.
- Less effects of signal skew and crosstalk due to less wires supports higher frequency clocking
- Lower data transfer rate
### Modes
![](https://i.imgur.com/rlUzNiZ.png)
### Synchronous
A common clock signal between transmitter and receiver is used to synchronise the data transfer.
Master-Slave configuration: master provides the clock signal
![](https://i.imgur.com/n0OTRHT.png)
#### Serial Peripheral Interface (SPI) Communication
![](https://i.imgur.com/wdfafjm.png)
### Asynchronous
- No common clock is provided between transmitter and receiver.
- Receiver must know transmitting clock rate prior to transmission and the number of bits in each data packet
- Uses `SYNC` words within the data packets to indicate START/STOP
#### Universal asynchronous receiver-transmitter (UART)
A hardware device that is configurable to send data between a transmitter and receiver.
The UART protocol:
![](https://i.imgur.com/jD59Cnf.png)
Parity bit workings
- Even parity scheme: there should be even number of "1s" in the data field and parity field combined. If there are odd number of 1s in the data, parity bit transmitted should be 1
- Vice versa for odd parity scheme
![](https://i.imgur.com/F8OKzEo.png)
![](https://i.imgur.com/76fcu12.png)
#### RS232 Transmission standard
Uses UART protocol transmission but
- uses a different electrical standard interface. i.e. Logic 1 is represented by a voltage smaller than Logic 0.
- Logic 1 can be -15V while Logic 0 can be +15V
Invert the receiving signal and we are able to read the data using the UART protocol equivalently.
An excellent tutorial:
https://www.youtube.com/watch?v=AHYNxpqKqwo
## Data Transfer Mechanism
### Polling
![](https://i.imgur.com/O1pP4YN.png)
### Interrupt Driven
[[Notes/Interrupts]]
