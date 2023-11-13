---
title: "Cryptography"
date: 2023-11-12
lastmod: 2023-11-13
---
Terminology
- Cryptology: art and science of making and breaking secret codes
- Cryptography: making secret codes
- Crpytanalysis: breaking secret codes
# Classical Ciphers
## Caesar Cipher
A *mono-alphabetic* encryption cipher, where each letter uniquely maps to some other letter.
![](Pics/Pasted%20image%2020231112110505.png)
However, this means that there are only a possible of 25 keys (since there are only 26 letters), which is very easy to crack.
### Hardened versions
Instead of shifting by a fixed amount, let each letter represent a different amount to be shifted by (i.e. "A" shifts by 1 etc.). This means that there will be 26! permutations. This is called a *substitution cipher*.,
![](Pics/Pasted%20image%2020231112110847.png)
However, a shortcut can be discovered without having to go through all combinations. Since each letter uniquely maps to another letter, we can perform frequency analysis on the ciphertext. In English, letters used are unevenly distributed ("E" is most common). From the frequency of letters, we can determine which letter maps which one easily.
>    a. Show that the complexity of mono-alphabetic substitution (26!) is roughly 2^88. 
>    $26! = 2^k$
>    $log_2(26!) = klog_22 \therefore k\approx88.3$
>    b. How do you make a mono-alphabetic substitution cipher harder to break?
>    Use plain text that does not obey the conventional frequency distribution of the language.
>    c. A 3 GHz PC can crack approximately 2^34 work in 1 day.  Calculate the time taken (in years) to crack mono-alphabetic substitution by brute force using 1 PC. What about cracking time of 1 billion PCs of same specs?
>    $2^{88}/2^{34}=2^{54} days = 2^{45}years$
>    With 1 billion PCs: $2^{45}/2^{30} = 2^{15}years$
## Vigenere Cipher
Poly-alphabetic substitution. Similar to the Caesar cipher, except that letters aren’t shifted by three places but rather by values defined by a key, a collection of letters that represent numbers based on their position in the alphabet.

![](Pics/Pasted%20image%2020231112111325.png)
For example, if the key is DUH, letters in the plaintext are shifted using the values 3, 20, 7
- because D is 3 letters after A,
- U is 20 letters after A, and
- H is 7 letters after A.
- The 3, 20, 7 pattern repeats until you’ve encrypted the entire plaintext.

> Why do long keywords, shorter message implies stronger Vigenere cipher?
>Longer keywords allow for more permutations of possible shifts. Shorter messages prevent attackers from detecting patterns in the cipher text which can provide clues to the original key used. 
>
>NSA has intercepted a Vignere ciphertext: {Y W W L F F D A Q B H L W B G V G R G S N Z D V U}, and Ethan Hunt has obtained the key-- CODE. Decrypt this ciphertext.
>WITHDRAWONEHUNDREDDOLLARS
# One Time Pad
The most secure cipher that guarantees perfect secrecy, *even if an attacker has unlimited computing power, it's impossible to learn anything about the plaintext accept for its length*.

Given a plain text $P$ and random key $K$, the one time pad produces a cipher text $C$ given as $C=P\oplus K$, where $\oplus$ is the XOR operator.
To decrpyt, simply XOR with the original key $K$. Hence, both encryption and decryption is instantaneous.

>General Douglas sends the message ATTACK to his soldiers using a one time pad {GZAMCQ} through email. Suppose attacker sniffed out such a ciphertext. Explain why he/she is not able to decrypt this cipher with 100% certainty, assuming attacker knows it’s from a one-time pad.
>The one time pad produces the cipher text "HTUNFB". The attacker can determine that one potential one-time pad value is "GZAMCQ" which produces the plain text ATTACK. However, another potential value is "DO..." produces another plain text DEFEND. Hence, there is no way to figure out which is the actual correct answer.

Since the key is truly random, the cipher text gives *no* useful information about the plain text, except for the length. This is because all plain texts are equally likely. An attacker can produce all possible combinations of the plaintext, but will not be able to distinguish which one is the original.
- Pad must be random. Else, not every key has an equal chance of being used as part of the 
- Pad used only once.  Else, an attacker can use previous cipher text as a way to distinguish the correct key.
- Bits used must be in sync
- Pad only known to sender and receiver
# Modern Ciphers
## Stream Ciphers
Stretch the key into an infinite **period** keystream $KS$ and use that to encrypt each bit deterministically.
$KS = SC(K,N) \therefore C =P\oplus KS \land P=C\oplus KS$
### Feedback shift registers
Hardware stream cipher which made it efficient.
## Block Ciphers
**Deterministic** algorithms that operate on a fixed-length group of bits called blocks.
### AES
Advanced Encryption Standard (AES) is the most widely used block cipher today.

### Modes of operation
What if we have a payload that is larger than the fixed size of the block?
#### Electronic Code Book
![](Pics/Pasted%20image%2020231113124429.png)

![](Pics/Pasted%20image%2020231113124448.png)
#### Cipher Block Chaining
An initialisation vector (IV) is used to XOR with the plain text before starting the encryption. A randomly generated IV will ensure that even if P1 and P2 are the same, the resulting cipher text is different.
![](Pics/Pasted%20image%2020231113124548.png)
Despite having a fixed key $k$, changing IV will generate a new cipher text. However, the *pair of  $(k, IV)$ must not be repeated* else security can be compromised.
#### Counter Mode
A counter is used to generate a key stream by passing it into the encryption engine. The resulting block cipher is used as a stream cipher of bits to XOR with the plain text.
![](Pics/Pasted%20image%2020231113143056.png)
Decrypt using $P_i = C_i \oplus E_k(CTR_i)$
- If the key k remains fixed, the counter must change for every block.
- The counter must not repeat for any block encrypted with same key.
- The pair (k, CTRi) must not repeat for the lifetime of the mechanism.

Counter options:
- Deterministic Counter : CTR1 = 0, CTR2 = 1, CTR3 = 2, ...
- Random Counter (IV) : CTR1 = IV, CTR2 = IV + 1, CTR3 = IV + 2, ...
- Random Counter (Nonce) : Choose IV = (x-bit Nonce || y-bit Counter)
- Galois Counter (GCM) : 96-bit Nonce fixed, 32-bit Counter increments

>Explain why for CTR mode in block cipher encryption, your counter values must all be distinct to be secure.
>With a fixed key, the same counter will encrypt into the same block cipher key stream. Hence, the cipher text produced will be the same given the same input bits. This can generate patterns (frequency) that can be studied.
## Key Management
Cryptographic algorithms need keys to work. A crypto-secure random number generater (RNG) is needed.
>In early IPOD days, some listeners complained hearing the same song within 2 hours although they have 400 songs on their ipod. Assuming 4 min songs on average. Question: Is the IPOD shuffling random?
>2 hours plays $2*60 / 4 = 30$ songs
>P(no clash) = $\frac{400*399*398*...*371}{400^{30}}=0.327$
>P(clash) = $1-0.327=0.673$ relatively high chance

The strength of strong algorithms depend on:
- Algorithm design
- Keys generated (quality of RNG)
> Suppose that a certain email system uses hash of time when document is encrypted & emailed together. Example: time 20220203073000 means (3 February 2022 7:30:00)
> a) Explore if hacker has a chance to read this document. 
> Brute force attack will take $10^{14}$ options. Since $2^{35}$ can be cracked in a day, it is very feasible
> b) What are the key lessons we can learn from this implementation of strong algorithm AES? 
> Strong algorithm alone is not enough. Keys generated must be of as high entropy as possible and cryptographically random.
> 
> Keystream of {0,1} generated by pseudo-random number generators will be periodic. Secure keystream must necessary have long period. Show that even with extremely long period, some keystreams may not be suitable for use to generate encryption keys.
> Consider a keystream of 0000.....11111111.... (100 0's followed by 100 1's). This keystream is predictable and not suitable.
- Key length
> [!Birthday Problem]
> A 128 bit algorithm should make use of 256 bit hash key. This is because it takes approx $2^128$ for a hash output to repeat, causing a collision.
- Secure implementation

Keeping the keys secret is much harder.
- Cryptanalysts often attack both symmetric and public-key cryptosystems through their key management. 
- Why should Eve bother going through all the trouble of trying to break the cryptographic algorithm if she can recover the key because of sloppy key storage procedures?
- Why should she spend $10 million building a cryptanalysis machine if she can spend $1000 bribing a clerk?
# Side Note on Hashing
Data transmitted over an insecure channel might have been modified in transit or spoofed even if they were encrypted. The purpose of encryption is to ensure that messages are authentic and confidential. **The purpose of hashing is to ensure data integrity**.
## Hash Functions
A hash function (e.g. SHA-256/512, KECCAK) takes in bits of ANY length and hashes them into a fixed size, typically 256 or 512-bits in modern context. This many-to-one function means it is possible that *different sets of bits hash to the same value*.
![](Pics/Pasted%20image%2020231113180754.png)
Properties:
1. No one is able to reverse the hash function given the output to obtain the input
2. Given an input and hashed value, it is computationally infeasible to find another input that hashes to the same value
3. Given a hashed output, find any 2 inputs that hash to the same value
>If the hash reflected on the web page of download coincides with your computed hash, does it always mean the files have not been tampered with, assuming the hash used is a real secured hash? 
>The hash on the web page may have been tampered with to reflect the incorrect hash
### Message Authentication Code (MAC)
Symmetric key system that consists of three algorithms:
- A key generation algorithm selects a key from the key space uniformly at random.
- A signing algorithm efficiently returns a tag given the key and the message.
- A verifying algorithm efficiently verifies the authenticity of the message given the *same* key and the tag. That is, return *accepted* when the message and tag are not tampered with or forged, and otherwise return *rejected*.
Limitation: cannot provide [non-repudiation](Notes/2460%20Software%20Safety%20and%20Security.md#Security%20properties). Since the private key is shared, anyone holding on to that key can generate MACs for other messages.
### Digital Signatures
Uses asymmetric keys, private key to sign a message. Decrypted with the public key. This instead offers non-repudiation as the private key generated signature is not shared.

A fast one way hash function (e.g. SHA-256) is used to create a fixed length message digest that can be verified with a shared public key. This is used to verify integrity of software downloaded etc.
# Public Key Cryptography
Symmetric key cryptography suffers from big drawbacks:
- How to agree on a new key change
- How to manage keys (storage, expiry)
- How to send encrypted messages to someone you don't know

*Asymmetric key cryptography* makes use of a pair of public and private keys, related to each other mathematically but unable to be determined with just either side of the key.
![](Pics/Pasted%20image%2020231113152552.png)
The public key is shared, while the private key is held secret. A message encrypted with the public key can only ever be decrypted by the intended recipient with the private key (confidentiality). An encrypted message received, able to be decrypted by the sender's public key can tell us that the message was truly sent by the sender (authenticity).
## RSA
How do we generate the private and public key pairs? RSA-2048 is a standard algorithm for doing so.
A public parameter N is generated as the product of 2 k-bit prime numbers.

Possible prime generation algorithm:
1. RNG needs to generate a large *random* odd number
2. Robust prime generation used to find the next prime up from this random integer

RSA algorithm:
1. Generate **random** prime $p$
2. Generate **random** prime $q$ **independently**
3. $N=p*q$
4. Generate a private decryption key exponent $d$  
### Weak RSA parameters
- Size Primes p & q ≤ 512 bits.  
- p & q are not independently generated  
- p & q are not randomly generated by crypto-secure RNG  
- Decryption key d ≤ (N^(0.25))/3  (“short d”)  
- p-1 is a product of only “small” prime factors  
- Common use of same N for users in same company, although they use  different encryption and decryption exponents.
<iframe width="560" height="315" src="https://www.youtube.com/embed/-ShwJqAalOk?si=QNvJQk_Prn2PkPxP" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen></iframe>