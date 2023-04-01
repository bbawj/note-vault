---
title: "Event-B"
date: 2023-03-28
---
# Event-B
A formal specification framework based on [Set Theory](Notes/Set%20Theory.md).
## Abstract Machine Notation
![](https://i.imgur.com/7xfv1u5.png)

![](https://i.imgur.com/64BY7T5.png)
## Syntax
### Context
![](https://i.imgur.com/MuA6Aj1.png)
### Machine
![](https://i.imgur.com/SXkthOv.png)
### Events
![](https://i.imgur.com/qaEwcKw.png)
#### Actions
![](https://i.imgur.com/3v567Uc.png)
![](https://i.imgur.com/pjz2k7S.png)
### Set
## Examples
### University Access
A system for controlling access to a university building
- An university has some fixed number of students.
- Students can be inside or outside the university building.
- The system should allow a new student to be registered in order to get the access to the university building.
- To deny the access to the building for a student the system should support deregistration.
- The system should allow only registered students to enter the university building.
![](https://i.imgur.com/VXtNCCN.png)
![](https://i.imgur.com/UgA1Nsh.png)
![](https://i.imgur.com/UqKBITK.png)
![](https://i.imgur.com/zHaLrA2.png)
![](https://i.imgur.com/M9Y5iot.png)
### Coffee Club
![](https://i.imgur.com/jtIHx6p.png)
![](https://i.imgur.com/kYOr7ax.png)
![](https://i.imgur.com/TQ514Bt.png)
![](https://i.imgur.com/O9vIX2G.png)
### Printer Access
- A system should support adding a permission for a student in order to get an access to a particular printer and removing a permission.
- A system should support removing a student’s access to all printers at once.
- A system should support giving the combined permissions of any two students to both of them.
#### Requirements Document
![](https://i.imgur.com/u8f6wNi.png)
#### Modelling
- To keep track of changing permissions, it will make use of a variable access whose type is a relation between STUDENTS and PRINTERS.  
![300](https://i.imgur.com/Qsa5Xxr.png)

![400](https://i.imgur.com/vehZFBv.png)
![400](https://i.imgur.com/YJuqO2y.png)
![400](https://i.imgur.com/kVgu5Zb.png)
#### New requirement: a student can use no more than 3 printers
![](https://i.imgur.com/ODxLte4.png)

![](https://i.imgur.com/U8eh3gW.png)
### Seat Booking System
![](https://i.imgur.com/QZwnvJ0.png)

