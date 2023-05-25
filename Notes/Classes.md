---
title: "Classes"
date: 2023-05-24
---
# Classes
Updating our grammar:
```
declaration    → classDecl
               | funDecl
               | varDecl
               | statement ;

classDecl      → "class" IDENTIFIER "{" function* "}" ;
```
In plain English, a class declaration is the class keyword, followed by the class’s name, then a curly-braced body. Inside that body is a list of method declarations. Unlike function declarations, methods don’t have a leading fun keyword. Each method is a name, parameter list, and body.

Parser:
```java
private Stmt classDeclaration() {
    Token name = consume(IDENTIFIER, "Expect class name.");
    consume(LEFT_BRACE, "Expect '{' before class body.");

    List<Stmt.Function> methods = new ArrayList<>();
    while (!check(RIGHT_BRACE) && !isAtEnd()) {
      methods.add(function("method"));
    }

    consume(RIGHT_BRACE, "Expect '}' after class body.");

    return new Stmt.Class(name, methods);
  }
```
Interpreter:
```java
@Override
  public Void visitClassStmt(Stmt.Class stmt) {
    environment.define(stmt.name.lexeme, null);
    LoxClass klass = new LoxClass(stmt.name.lexeme);
    environment.assign(stmt.name, klass);
    return null;
  }
```
## Instances
We create instances by *calling* a class name:
```java
class LoxClass implements LoxCallable {
  @Override
  public Object call(Interpreter interpreter,
                     List<Object> arguments) {
    LoxInstance instance = new LoxInstance(this);
    return instance;
  }
```
## Properties
Properties are accessed using a `.` syntax. `someObject.someProperty`
Updating grammar:
```
call           → primary ( "(" arguments? ")" | "." IDENTIFIER )* ;
```
After a primary expression, we allow a series of any mixture of parenthesized calls and dotted property accesses (i.e. get expressions).
### Get Expressions
A get expression stores the name and the expression: `"Get: Expr object, Token name",`

The get expression will call the `get` method on a LoxInstance, returning named fields in the class:
```java
@Override
  public Object visitGetExpr(Expr.Get expr) {
    Object object = evaluate(expr.object);
    if (object instanceof LoxInstance) {
      return ((LoxInstance) object).get(expr.name);
    }

    throw new RuntimeError(expr.name,
        "Only instances have properties.");
  }

  Object get(Token name) {
    if (fields.containsKey(name.lexeme)) {
      return fields.get(name.lexeme);
    }

    throw new RuntimeError(name, 
        "Undefined property '" + name.lexeme + "'.");
  }
```
### Set Expressions
Assignment now supports dotted identifiers on the left hand:
```
assignment     → ( call "." )? IDENTIFIER "=" assignment
               | logic_or ;
```
However, the reference to `call` allows any high-precedence expression before the last dot, including any number of _getters_:
![](https://i.imgur.com/MGfMz4m.png)
## Methods
For each method, we create a new LoxFunction and add that to the class via a hashmap.
```java
    Map<String, LoxFunction> methods = new HashMap<>();
    for (Stmt.Function method : stmt.methods) {
      LoxFunction function = new LoxFunction(method, environment);
      methods.put(method.name.lexeme, function);
    }

    LoxClass klass = new LoxClass(stmt.name.lexeme, methods);
```
## This
```java
class Person {
  sayName() {
    print this.name;
  }
}

var jane = Person();
jane.name = "Jane";

var bill = Person();
bill.name = "Bill";

bill.sayName = jane.sayName;
bill.sayName(); // ?
```
Does that last line print “Bill” because that’s the instance that we _called_ the method through, or “Jane” because it’s the instance where we first grabbed the method?

*Bound methods*: if you take a reference to a method on some object so you can use it as a callback later, you want to remember the instance it belonged to, even if that callback happens to be stored in a field on some other object.
![](https://i.imgur.com/konesbH.png)
We need to take `this` at the point that the method is accessed and attach it to the function through a closure.

Put this into the current scope in resolver:
```java
beginScope();
    scopes.peek().put("this", true);
```
For each method, bind this into its closure environment:
```java
LoxFunction bind(LoxInstance instance) {
    Environment environment = new Environment(closure);
    environment.define("this", instance);
    return new LoxFunction(declaration, environment);
  }
```
## Constructors
Lox uses `init` as a constructor.

Store whether a LoxFunction is an initializer or not
```java
 private final boolean isInitializer;

  LoxFunction(Stmt.Function declaration, Environment closure,
              boolean isInitializer) {
    this.isInitializer = isInitializer;
```
If it is, we get the bound instance in the function closure:
```java
if (isInitializer) return closure.getAt(0, "this");
```
## Inheritance
Lox uses the `<` to define an *extends* relationship:
```
classDecl      → "class" IDENTIFIER ( "<" IDENTIFIER )?
                 "{" function* "}" ;
```
The class expression must now capture the superclass relationship:
```
      "Class      : Token name, Expr.Variable superclass," +
                  " List<Stmt.Function> methods",
```
Look for the method in the current class before walking up the superclass chain:
```java
    if (superclass != null) {
      return superclass.findMethod(name);
    }
```
### Super
With `this`, the keyword works sort of like a magic variable, and the expression is that one lone token. But with `super`, the subsequent `.` and property name are inseparable parts of the `super` expression. You can’t have a bare `super` token all by itself.
```
primary        → "true" | "false" | "nil" | "this"
               | NUMBER | STRING | IDENTIFIER | "(" expression ")"
               | "super" "." IDENTIFIER ;
```
The super expression contains the keyword and its method access:
```
"Super    : Token keyword, Token method",
```
a super expression starts the method lookup from “the superclass”, but which superclass? The naïve answer is the superclass of this, the object the surrounding method was called on. That coincidentally produces the right behavior in a lot of cases, but that’s not actually correct.
```java
class A {
  method() {
    print "A method";
  }
}

class B < A {
  method() {
    print "B method";
  }

  test() {
    super.method();
  }
}

class C < B {}

C().test(); // A method
```
Instead, lookup should start on the superclass of _the class containing the `super` expression_. In this case, since `test()` is defined inside B, the `super` expression inside it should start the lookup on _B_’s superclass—A.

One important difference is that we bound `this` when the method was _accessed_. The same method can be called on different instances and each needs its own `this`. With `super` expressions, the superclass is a fixed property of the _class declaration itself_. Every time you evaluate some `super` expression, the superclass is always the same.

That means we can create the environment for the superclass once, when the class definition is executed. Immediately before we define the methods, we make a new environment to bind the class’s superclass to the name `super`