# Factory Pattern
#design-patterns 

## Problems we want to solve
1. Decouple class selection and object creation from the place where the object is used.
2. Need to instantiate a set of classes but without knowing exactly which one until runtime.
3. Do not want to expose object creation logic to the client.

![](https://i.imgur.com/sHF2nj4.png)

The interface and concrete product classes implement an additional [Strategy Pattern](Notes/Strategy%20Pattern.md) design which allows the algorithms to be instantiated and changed during runtime.

## Pros
1. Encapsulation of object creation
2. Extensibility of classes
3. Can easily change object creation logic without affecting context due to decoupling

## Cons
1. Complexity