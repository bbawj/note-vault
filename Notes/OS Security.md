---
title: OS Security
date: 2023-09-21
---
![](Pics/Pasted%20image%2020230921121212.png)
# Security Protection Stages
![](Pics/Pasted%20image%2020230925214938.png)
- Identification & Auth: comparing input credentials with stored ones
## Users & Access Control
Principle of least privilege
- users  should only have access to resources needed to perform the desired tasks
Privilege separation
- split a system into different components each with the least privilege for its task
- prevents any attacker from taking over the entire system

A system can have many accounts. 16 bit UIDs are used to identify accounts, e.g. root(0), bin(1). An attacker managing to get root user status effectively takes over the entire system.
### Security Policy
[File permissions](Notes/File%20Systems.md#Protection) are used to control access. They are split into subjects (the user, program, process etc.) , object (file, memory) and operation (read/write/execute).
![](Pics/Pasted%20image%2020230921135811.png)
### Groups
![](Pics/Pasted%20image%2020230921135210.png)
## Process  Controlled Invocation
Processes are assigned 2 types of user and group IDs.
![](Pics/Pasted%20image%2020230921140026.png)
Root user privilege is required to execute certain OS functions, e.g. password changing, which requires R/W access to `/etc/shadow`. The OS overcomes this by giving additional permissions via a SUID:
![](Pics/Pasted%20image%2020230921140143.png)
```
// Example Setting SUID for a file such that it executes with the owner permissions of the file
chmod o+s <FILE>
// Example Setting SUID for a file such that it executes with the group permissions of the file
chmod g+s <FILE>
```
By tricking the SUID program owned by root to do unintended things, an attacker can act as root. Hence, all user input must be processed with extreme care, programs should have SUID status only if it is really necessary.
### RootKit
Malware that obtains root privileges. It originally referred to a set of maliciously modified set of administrative tools which granted root access. Once granted, uses can be as follows:
#### Hijacking sys-call tables
![](Pics/Pasted%20image%2020230921141520.png)

![](Pics/Pasted%20image%2020230921141434.png)
#### Hijacking [Interrupt Stack Table (IST)](Notes/Interrupts.md#Interrupt%20Stack%20Table%20(IST)%20and%20Task%20State%20Segment%20(TSS))
![](Pics/Pasted%20image%2020230921142335.png)
## Logging
![](Pics/Pasted%20image%2020230925215919.png)
# Confinement
The OS isolates applications in the system. If one is malicious, it prevents the rest of the system from being harmed.
## [Virtualization](Notes/Virtualization.md)
![](Pics/Pasted%20image%2020230922000656.png)
However, hypervisors have higher privileges than the OS kernel, and offer a larger attack surface with larger code bases. If compromised, an attacker can take over the entire system more easily.
## Containers
Package code and all dependencies into 1 unit called a container. A container engine is used to manage the different containers, which run isolated from each other.
![](Pics/Pasted%20image%2020230922001007.png)
## Reference Monitors
A mechanism to monitor and mediate/deny requests from protected targets at runtime when a security policy is violated.
![](Pics/Pasted%20image%2020230922002352.png)
### Requirements
Functional:
- Reference validation mechanism must always be invoked
- RM able to observe all requests and deny malicious request
Security:
- Mechanism must be tamper-proof
Assurance:
- Mechanism must be small enough to be analysed and tested

Hardware-based RMs are introduced to monitor the OS
# Integrity Verification
A chain of trust is created by establishing verified systems from bottom to top.
## Software:
![](Pics/Pasted%20image%2020230922002830.png)
## Hardware
After the chip is fabricated, it is hard for the attacker to modify it. The integrity of hardware can be guaranteed. It is also very hard for the attacker to peek into the chip and steal the secret (e.g., encryption key). The confidentiality of hardware can also be guaranteed. Hardware is a perfect component as the start of the chain of trust. 
### Trusted Platform Module (TPM)
![](Pics/Pasted%20image%2020230922003158.png)
#### Platform Integrity
Helps build a chain of trust by establishing a secure boot process from the TPM until the OS has fully booted and applications are running.
#### Full disk encryption
- Encrypt data with hardware key, which is difficult to steal as it never leaves the chip.
- E.g. Windows Bitlocker encrypts disk with FVEK key. This key is further encrypted with the Storage Root Key in the TPM.
#### Remote Attestation
Provide unforgeable evidence about the security of its software to a client for systems to prove that the platform is trustworthy.
![](Pics/Pasted%20image%2020230922004109.png)
Integrity measurement architecture:
- Must be able to provide reliable and trustworthy security report.
- TPM provides a platform integrity measurement service which cannot be compromised by the OS or applications.
- Measure the hash value of the code of each loaded software and store them in the Platform Configuration Register
Remote attestation protocol:
- Report must be transmitted to the client without being modified
- TPM is equipped with keys to encrypt and sign messages
- Key is verified with a trusted third party Privacy Certification Authority
#### Trusted Execution Environment (TEE)
The TPM can guarantee that privileged software (like the OS), is not malicious at load time, but not the security at runtime. The TEE ensures that the OS should support the execution of apps but should not be able to compromise them by reading data, changing their code.
![](Pics/Pasted%20image%2020230922004554.png)
![](Pics/Pasted%20image%2020230922005621.png)
With SEV, the TCB can be reduced to just the hardware. The hypervisor does not need to be trusted.
![](Pics/Pasted%20image%2020230922005643.png)
![](Pics/Pasted%20image%2020230922005743.png)
Use cases:
To provide increased security against malicious actors gaining access to a device’s content, the PIN and fingerprint functionalities used to unlock the Android device must be secured. Privileged and secure access to peripherals (like the fingerprint sensor) is provided by the TEE.