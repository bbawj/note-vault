---
title: "Model Checking"
date: 2023-04-11
lastmod: 2023-04-12
---
# Model Checking
Inputs:
- description of the system
- property to verify
The tool either confirms that the property is true in the model or informs the user that it does not hold. In that case, the model checker will also provide a *counter-example*: a run of the system that violates the property. This answer helps find the reason for the failure and has significantly contributed to the success of model checking in practice.
## Transition Systems and Kripke Structures
Transition systems describe the states, initial states and the possible state transition for the system.

Formally: $T=(Q,I,E,\delta)$
- $Q$ set of states
- $I$ set of initial states
- $E$ set of actions
- $\delta$ a transition relation $\delta \subset Q\times E\times Q$
### Kripke Structures
A Kripke structure extends a transition system with the idea of propositions in states.

Formally: $K=(Q,I,E,\delta,\lambda)$
- $\lambda: Q\rightarrow2^V$, where $V$ is a set
![](https://i.imgur.com/K2zPO0W.png)
## Checking Invariants
Invariants are safety properties.

Formally: a system invariant is a property $P$ such that $q\models P$, for all reachable states $q$.
![](https://i.imgur.com/U18MQjB.png)
### Enumeration Algorithm
A simple [Breadth First Search](Notes/Breadth%20First%20Search.md) or [Depth First Search](Notes/Depth%20First%20Search.md) algorithm to enumerate all possible states and check for an invariant violation.
![](https://i.imgur.com/CZ7ZoIA.png)
The search space is large. A few techniques can be employed:
- Reduction based: determine a subset of runs whose exploration guarantees the correctness invariant over the overall system
- Abstraction: construct a significantly smaller model whose correctness guarantees the correctness of the original model.
## Temporal Logic
The properties/invariants we need to check are for example:
- Does (the reachable part of) K contain “bad” states, such as deadlock states where only the τ action is enabled, or states that do not satisfy an invariant?
- Are there executions of K such that, after some time, a “good” state is never reached or a certain action never executed (livelock)?

### Linear Temporal Logic
We can express these by combining [Propositional Logic](Notes/Propositional%20Logic.md) and temporal operators:
![](https://i.imgur.com/48C4UHK.png)
Operators are defined on paths/traces of execution:
![](https://i.imgur.com/0FIVOBR.png)
They can also be nested (only the second trace is true):
![300](https://i.imgur.com/UHhxhC8.png)
### Computation Tree Logic
Apply temporal logic not just on a single path but on many different possible branches which is needed for transition systems.
![](https://i.imgur.com/v0Cwu78.png)
Operators cannot be nested
![](https://i.imgur.com/anXQDlo.png)
## Tools
A core tool used for model checking is [[NuSMV]]
- Symbolic Model Verifier (SMV)
- NuSMV: open source re-implementation
