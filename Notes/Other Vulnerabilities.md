---
title: "Other Vulnerabilities"
date: 2023-09-18
---
# Other Vulnerabilities
## Format Strings
### Leaking the stack
![](Pics/Pasted%20image%2020230918101352.png)
### Crashing the program
![](Pics/Pasted%20image%2020230918101437.png)
### Modifying memory 
The format string `%n` does not print anything, instead it stores the number of characters printed so far and can be used to store this value in an address.
![](Pics/Pasted%20image%2020230918101506.png)
## Integer Overflows
![](Pics/Pasted%20image%2020230918103559.png)

![](Pics/Pasted%20image%2020230918103527.png)
## Scripting Vulnerabilities
Construct commands from predefined code fragments and user input at run time.
![](Pics/Pasted%20image%2020230918103805.png)
### Cross Site Scripting (XSS) attacks
Exploit websites which receive user input to inject executable malicious code into the website. 