---
title: "Default Logic"
date: 2022-11-08
lastmod: 2022-11-21
---
# Default Logic
## Definitions
![](https://i.imgur.com/m6Vb0UF.png)
![](https://i.imgur.com/6mLLFN7.png)
![](https://i.imgur.com/1SGoodm.png)

## Reiter Extension
![](https://i.imgur.com/awvYPHM.png)
![](https://i.imgur.com/IR3xxOj.png)

## Makinson Approach
![](https://i.imgur.com/KMzHeSh.png)

### Process Tree Algorithm
A __closed__ default is one that has been instantiated
The In-set contains all the consequences from applying a default
The Out-set contains all the negations of the justifications from applying a default: these are the predicates which cannot be proven true by the In-Set for the extension to be consistent

1. Start with the root node: Out is initialized to $\emptyset$ while In is set to the current knowledge base
2. For every node, check for direct applicability of defaults (If no defaults are directly applicable: __we arrived at a closed process)__ direct applicability must satisfy 2 conditions:
	1. Default must be triggered: In-set contains the prerequisite
	2. Default must be justified: negation of justifications cannot be proven True from the current In-set
4. If the new In-set becomes inconsistent ($In \cap Out \neq \emptyset$ or $In\cup \delta .consq \ \vdash Out$) : __process is unsuccessful__

__We arrive at an extension for every closed and successful process__.

![](https://i.imgur.com/ZjJAXQC.png)

![](https://i.imgur.com/QpFdmBM.png)

![](https://i.imgur.com/7UDQGXx.png)
