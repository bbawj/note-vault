---
title:"Knowledge Representation"
---
# Knowledge Representation
Representation of knowledge in a computer-tractable form.
$$\begin{aligned}Logic=Representation+Inference\\ Representation=Syntax+Semantics \end{aligned}$$
Entailment: generates sentences that are necessarily true given that existing sentences are true

Inference: process to implement entailment

Soundness: only generate entailed sentences. If we start with valid premises, no invalid conclusions can be drawn and the system is sound. Everything that is provable is in fact true. 

Completeness: Everything that is true has a proof. __Sound inference does not have to be complete__.

![](https://i.imgur.com/2W1p0tF.png)

![](https://i.imgur.com/1NLoTi8.png)

### Resolution Strategies
![](https://i.imgur.com/jAOvdCc.png)

__Resolution by Refutation__

![](https://i.imgur.com/0gquUHg.png)
![](https://i.imgur.com/65qwq8t.png)
