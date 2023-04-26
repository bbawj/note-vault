---
title: "Visitor Pattern"
date: 2023-04-26
lastmod: 2023-04-26
---
# Visitor Pattern
## Problems we want to solve
### Expression Problem
An object-oriented language like Java assumes that all of the code in one row naturally hangs together. It figures all the things you do with a type are likely related to each other, and the language makes it easy to define them together as methods inside the same class.
![](https://i.imgur.com/efHxyd5.png)

![](https://i.imgur.com/hmj9JaO.png)
This makes it easy to extend the table by adding new rows. Simply define a new class. No existing code has to be touched. But imagine if you want to add a new *operation*—a new column. In Java, that means cracking open each of those existing classes and adding a method to it.
### The functional way
![](https://i.imgur.com/oXBkdvm.png)
## General Idea
The Visitor pattern is really about approximating the functional style within an OOP language. It lets us add new columns to that table easily.

Define all the operations for each class in a single separate interface:
```java
interface PastryVisitor {
    void visitBeignet(Beignet beignet); 
    void visitCruller(Cruller cruller);
}
```

The base class defines an abstract `accept()` that subclasses must implement:
```java
  abstract class Pastry {
    abstract void accept(PastryVisitor visitor);
  }
```

Each subclass takes in visitor for the operation we want to execute, and calls the appropriate visit method.
```java
  class Beignet extends Pastry {
    @Override
    void accept(PastryVisitor visitor) {
      visitor.visitBeignet(this);
    }
  }
  class Cruller extends Pastry {
    @Override
    void accept(PastryVisitor visitor) {
      visitor.visitCruller(this);
    }
  }
```

![](https://i.imgur.com/22pynV4.png)

When we want to define new operations on the set of classes, we create a new class that implements the visitor interface:
```java
class Cook implements PantryVisitor {
	@Override
	void visitBeignet(Beignet beignet) {
		// cook beignet
	}
	void visitCruller(Cruller cruller) {
		// cook cruller
	}
}

class Eat implements PantryVisitor {
	@Override
	void visitBeignet(Beignet beignet) {
		// eat beignet
	}
	void visitCruller(Cruller cruller) {
		// eat cruller
	}
}
```
## Pros
1. We added one accept() method to each class, and we can use it for as many visitors as we want without ever having to touch the pastry classes again.
## Cons
1. 
