---
title: "Authentication"
date: 2023-11-11
---
Authentication is ensuring the communicated entity is the correct entity.
# Something you know
## Passwords
### Storage
Password security relies on one-way functions (e.g hash functions).
- Pre image resistant: it is computationally infeasible to find $x$ given $y$ such that $H(x) = y$
- Collision resistant: computationally infeasible to find a pair $(x,y)$ such that $x\ne y \land H(x) = H(y)$
#### Salting
Add a random string to the user password before hashing, and store the salt along with the hashed value. An attacker would need to precompute the salt as well as the potential user password.
### Attacks
![](Pics/Pasted%20image%2020231111224116.png)
- Brute force approach can be mitigated with high entropy passwords.  Key loggers can also figure out complex passwords if a system is already compromised. Hence, a good password policy can help ensure most passwords are not weak.
#### Dictionary Attack
Pre-compute a hash table containing pairs of common passwords and their hashes. Weak passwords tend to include common names, dictionary words
# Something you have
One Time Password (OTP) systems create a new password each time a user logs in. 
A SecurID card is such a system. A server knows the algorithm that the card uses, and can verify the password entered by the user. Modern systems integrate OTP into cell phones.
## Smart Cards
Information is stored in the card's memory, only accessible to the on board microprocessor. The microprocessor runs software which can authenticate a user. Tamper resistance prevents unauthorised access.
How it works:
- Smart card issues a challenge to a reader
- User is then required to enter a PIN, allowing the reader to compute a response
- If the card receives the correct response, the user is authenticated
Limitations:
- the smart card reader needs to be trusted
- an attacker can set up a rogue reader, recording user's PIN in the process
- if the attacker obtains the physical card, he will be able to bypass authentication
## ATM Cards
Contains a magnetic stripe that stores user account number data. This data is used as part of the authentication process for anyone using the ATM. The card is not tamper-resistant and anyone with a reader is able to access the information.
# Something you are
Bio metrics
- Palm scan: measures the size of hand and fingers, and curves
- Iris scan
- Retina scan: infrared light is shot into a user's eyes and the pattern of retinal blood vessels is read to create a signature
- Fingerprint
- Voice
- Face
- Signature
Limitations:
- False positives + negatives: impersonation
- Varying social acceptance: personal and private data needs to be stored
- Key management issues: unable to revoke a bio metric key, as a user cannot change it even if it has been compromised by an attacker.

Combining different strategies is more effective.
# Authentication on the Internet
- Client authentication: the server verifies that the client is legitimate.
- Server authentication: the client verifies that the server is legitimate. Ecommerce sites perform server authentication right before the user makes a purchase, to ensure personal details are not sent to a spoofed web server.
- Mutual authentication: both
# Exercises
Is it always a bad idea to write down your password? Is there a way to do it more safely?

Not necessarily. Ensure they are stored safely, not together with the device, and that in return, the passwords used obey a strong password policy. Can use modern password storage software, where a master key is needed to gain access. Easier to remember a singular master key.
![](Pics/Pasted%20image%2020231112111803.png)
2a. $2^{51.7}, 2^{53.6}, 2^{52.6}$. With more combinations, shorter keys offer similar complexities
2bi. 