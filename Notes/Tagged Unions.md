---
title: "Tagged Unions"
date: 2023-05-27
---
# Tagged Unions
A [data structure](https://en.wikipedia.org/wiki/Data_structure "Data structure") used to hold a value that could take on several different, but fixed, types. Only one of the types can be in use at any one time, and a **tag** field explicitly indicates which one is in use. It can be thought of as a type that has several "cases", each of which should be handled correctly when that type is manipulated.

Tagged unions can save storage by overlapping storage areas for each type, since only one is in use at a time. A union looks like a struct except that all of its fields overlap in memory:
![](https://i.imgur.com/xdlDDNu.png)
## Using it for a bytecode VM value representation
A value contains two parts: a type “tag”, and a payload for the actual value. To store the value’s type, we define an enum for each kind of value the VM supports.
```c
typedef struct {
  ValueType type;
  union {
    bool boolean;
    double number;
  } as; 
} Value;
```