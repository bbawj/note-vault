---
title: "Model-View-Controller Architecture"
date: 2022-11-08
lastmod: 2022-11-21
---
# MVC Architecture
## Design Problems
1. Tight coupling between UI and application logic
2. [Observer Pattern](Notes/Observer%20Pattern.md): Need for UI to update when state changes
3. [Strategy Pattern](Notes/Strategy%20Pattern.md): Need for UI to support different functionalities depending on the user input

![](https://i.imgur.com/bLAeQ9M.png)

![](https://i.imgur.com/A1cpmcm.png)

View:
- Manage how data is presented
- Observes the Model and updates their graphical representation
Model:
- Provides the operations to register and unregister observers
- Implements a notify method to call observers update
Controller:
- Captures input and passes them to the view and model

## Pros
1. Support for simultaneous development
2. Support for multiple views with just 1 Model
3. High cohesion: grouping of related actions
4. Low coupling

## Cons
1. Code navigability
2. Maintaining multi-artefact consistency: decomposition of features results in scattering
