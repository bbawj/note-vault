---
title: "Dynamic Loading"
date: 2022-11-08
lastmod: 2022-11-21
---
# Dynamic Loading
Mechanism for loading a binary and execute functions from external software.

Allows program to start up in the absence of these libraries, to discover available libraries, and to potentially gain additional functionality.

```java
//java reflection API
Class type = Class.forName(name);
Object obj = type.newInstance();
```
