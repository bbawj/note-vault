---
title: "Dependency Injection"
date: 2022-11-08
lastmod: 2023-02-12
---
# Dependency Injection
## Problems we want to solve
1. How can a class be independent from the creation of the objects it depends on?
2. How can an application, and the objects it uses support different configurations?
3. How can the behaviour of a piece of code be changed without editing it directly?
## General Idea
An object receives other objects that it depends on. A form of inversion of control, dependency injection aims to separate the concerns of constructing objects and using them, leading to loosely coupled programs. 

### Constructor injection
The most common form of dependency injection is for a class to request its dependencies through its constructor. This ensures the client is always in a valid state, since it cannot be instantiated without its necessary dependencies.

```java
// This class accepts a service in its constructor.
Client(Service service) {
    
    // The client can verify its dependencies are valid before allowing construction.
    if (service == null) {
        throw new InvalidParameterException("service must not be null");
    }

    // Clients typically save a reference so other methods in the class can access it.
    this.service = service;
}
```
## Pros
1. 
## Cons
1. 
