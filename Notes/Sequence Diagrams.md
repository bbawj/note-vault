---
title:"Sequence Diagrams"
---
# Sequence Diagrams
A sequence diagram **_captures the interactions between multiple objects for a given scenario._**

Notation:

![](https://nus-cs2113-ay2122s2.github.io/website/book/uml/sequenceDiagrams/basic/images/notation.png)

#### Common notation errors 
**Activation bar too long:** The activation bar of a method cannot start before the method call arrives and a method cannot remain active after the method has returned. In the two sequence diagrams below, the one on the left commits this error because the activation bar starts _before_ the method `Foo#xyz()` is called and remains active _after_ the method returns.

![](https://nus-cs2113-ay2122s2.github.io/website/book/uml/sequenceDiagrams/basic/images/commonError-activationBarTooLong.png)

 **Broken activation bar:** The activation bar should remain unbroken from the point the method is called until the method returns. In the two sequence diagrams below, the one on the left commits this error because the activation bar for the method `Foo#abc()` is not contiguous, but appears as two pieces instead.

![](https://nus-cs2113-ay2122s2.github.io/website/book/uml/sequenceDiagrams/basic/images/commonError-brokenActivationBar.png)

#### Object Creation
Notation:

![](https://nus-cs2113-ay2122s2.github.io/website/book/uml/sequenceDiagrams/objectCreation/images/notation.png)

-   The arrow that represents the constructor arrives at the side of the box representing the instance.
-   The activation bar represents the period the constructor is active.

#### Object Deletion
**UML uses an `X` at the end of the lifeline of an object to show its deletion.**

Although object deletion is not that important in languages such as Java that support automatic memory management, you can still show object deletion in UML diagrams to indicate the point at which the object ceases to be used.

Notation:
![](https://nus-cs2113-ay2122s2.github.io/website/book/uml/sequenceDiagrams/objectDeletion/images/notation.png)


#### Loops
Notation:
The `Player` calls the `mark x,y` command or `clear x y` command repeatedly until the game is won or lost.

![|600x700](https://nus-cs2113-ay2122s2.github.io/website/book/uml/sequenceDiagrams/loops/images/playerText.png)

#### Self Invocation
**UML can show a method of an object calling another of its own methods.**

Notation:

![](https://nus-cs2113-ay2122s2.github.io/website/book/uml/sequenceDiagrams/selfInvocation/images/notation.png)
In this variation, the `Book#write()` method is calling the `Chapter#getText()` method which in turn does a _call back_ by calling the `getAuthor()` method of the calling object.

![](https://nus-cs2113-ay2122s2.github.io/website/book/uml/sequenceDiagrams/selfInvocation/images/callBack.png)

#### Alternative Paths
**UML uses `alt` frames to indicate alternative paths.**

Notation:
![](https://nus-cs2113-ay2122s2.github.io/website/book/uml/sequenceDiagrams/alternativePaths/images/notation.png)

#### Calls to Static Methods
Method calls to `static` (i.e., class-level) methods are received by the class itself, not an instance of that class. You can use `<<class>>` to show that a participant is the class itself.

In this example, `m` calls the static method `Person.getMaxAge()` and also the `setAge()` method of a `Person` object `p`.

![](https://nus-cs2113-ay2122s2.github.io/website/book/uml/sequenceDiagrams/staticMethods/images/staticMethodCall.png)
