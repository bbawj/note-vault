---
title: "Modal Checking"
date: 2023-04-11
---
# Modal Checking
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
