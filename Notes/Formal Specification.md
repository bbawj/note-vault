---
title: "Formal Specification"
date: 2023-03-28
lastmod: 2023-03-28
---
# Formal Specification
A formal specification is the expression, in some formal language and at the some level of abstraction, of a collection of properties the system should satisfy through its behaviour.
## Motivation
*Boehm’s First Law: Errors are more frequent during requirements and design activities and are more expensive the later they are removed.*

Formality helps us to obtain higher quality specifications which are able to detect serious problems in original informal specifications. It also enables automated analysis of the specification.
## Problem Abstraction
Process of simplifying the problem at hand and facilitating our understanding of a system.
- focus on intended purpose
- ignore details of how the purpose is achieved
## Systems
![](https://i.imgur.com/1OwlwHL.png)
- Application: A physical entity whose function and operation is being monitored and controlled
- Controller: Hardware and software monitoring and controlling the application in real time
- Actuator (effector): A device that converts an electrical signal from the output of the computer to a physical quantity, which affects the function of the application.
- Sensor: A device that converts an application’s physical quantity into an electric signal for input into the computer
### Example on cold vaccine storage
A system that stores vaccine at a temperature that *should not exceed -70 degrees*
- Application: storage chamber  
- Sensor: temperature sensor  
- Actuator: cooling engine  
- Controller (software):  
	- checks measurements  
	- sets the cooling engine  
	Might also:  
	- output information on a display  
	- Write to log file and send it over network
 
Safety property: $temp+\delta\le-70$
[Fault Tree Analysis](Notes/Risk%20Analysis.md#Fault%20Tree%20Analysis):
![](https://i.imgur.com/AaVRrhk.png)
Safety invariants (things that should *always* hold) that need to be verified:
1. Always after controller has reacted, if sensor is not OK then alarm is raised and actuator is in decr
2. Always after controller reacted, if sensor is OK and temp + Δ ≥ -70 then cooler is in decr
## Formal Specification Frameworks
[[Event-B]]