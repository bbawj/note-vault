---
title: "2460 Software Safety and Security"
date: 2023-03-27
tags: [moc]
lastmod: 2023-04-11
---
#moc 
- [[Risk Analysis]]
- [[X.509 Email Address Vulnerability]]
- [[Formal Specification]]
- [[Notes/Model Checking]]
- [[Software Model Checking]]
- [[Memory Safety]]
- [[Other Vulnerabilities]]

[Safety](Notes/Safety%20and%20Liveliness.md): condition of being protected from harm
Security: degree of protection from harm
# Verification vs Validation
Verification: does the software do things right?
- can be automated by tools to verify specific properties
Validation: does the software do the right thing?
- requires human judgement to think about which are the correct requirements/operations
## Verification
Dynamic analysis: performs at run time analysing the real state of the system
Static analysis: performs at compile time to analyse the simplified state of the system
![](https://i.imgur.com/bSnXtdn.png)
# Trust
Trust is what we expect the entity to do and not to do.
## Trusted Computing Base
A set of system components that need to be trusted to ensure security. 
### Threat Model
A threat model needs to describe:
1. What is trusted
2. Resources and knowledge/actions the untrusted entities can do
3. Security properties we aim to achieve

An example: phishing email – a malicious email with malware as the attachment
- What is trusted: hardware and OS
- What is not trusted: the email attachment.
- Adversarial capabilities: running malicious code in your computer.
- Security properties: protect the computer system such that the malware cannot steal the sensitive data, or tamper with other processes.
### Security properties
- Confidentiality: prevent disclosure of information
- Integrity: prevent modification
- Availability: prevent withholding of information, resources should always be available (DDOS attacks)
- Accountability: actions of an entity can be traced and identified
- Non-repudiation: unforgeable evidence that specific actions occur
- Authenticity: ensure the communicated entity is the correct entity.
### Security Strategies:
- Prevention
- Detection
- Reaction
# Vulnerabilities
- Vulnerability: the weakness of a program that reduces its information assurance  
- Exploit: the technique the attacker takes to compromise the target system  
- Payload: the code the attacker wants the system to run.
![](Pics/Pasted%20image%2020230926224147.png)
## Side channel attack
An attack which is based on extra information that can be gathered because of the fundamental way a computer protocol or algorithm is implemented, rather than flaws in the design of the protocol or algorithm itself.
- Timing attack:  based on measuring how much time various computations (such as, say, comparing an attacker's given password with the victim's unknown one) take to perform.
# Types of malware:
![](Pics/Pasted%20image%2020230922213612.png)
