---
title: "Functions"
date: 2023-05-23
lastmod: 2023-05-23
---
# Functions
The name of the function being called isn’t actually part of the call syntax. The thing being called, *the callee*, can be any expression that evaluates to a function.
`getCallback()();`
The first pair of parentheses has `getCallback` as its callee. But the second call has the entire `getCallback()` expression as its callee. It is the parentheses following an expression that indicate a function call. You can think of a call as sort of like a postfix operator that starts with `(`.

Updating our grammar:
```
unary          → ( "!" | "-" ) unary | call ;
call           → primary ( "(" arguments? ")" )* ;
arguments      → expression ( "," expression )* ;
```

We can say that a function is one that implements an interface:
```java
interface LoxCallable {
  int arity();
  Object call(Interpreter interpreter, List<Object> arguments);
```
Interpreting function calls:
```java
  @Override
  public Object visitCallExpr(Expr.Call expr) {
    Object callee = evaluate(expr.callee);

    List<Object> arguments = new ArrayList<>();
    for (Expr argument : expr.arguments) { 
      arguments.add(evaluate(argument));
    }

    LoxCallable function = (LoxCallable)callee;
    return function.call(this, arguments);
  }
```
## Currying
Named after Haskell Curry, the rule uses `*` to allow matching a series of calls like `fn(1)(2)(3)`. In this style, defining a function that takes multiple arguments is as a series of nested functions. Each function takes one argument and returns a new function. That function consumes the next argument, returns yet another function, and so on.
## Arity
Arity is the fancy term for the number of arguments a function or operation expects. Unary operators have arity one, binary operators two, etc. With functions, the arity is determined by the number of parameters it declares.
## Native Functions
**Primitives**, **external functions**, or **foreign functions**. They are functions that the interpreter exposes to user code but that are implemented in the host language (in our case Java), not the language being implemented (Lox).

They provide access to the fundamental services that all programs are defined in terms of. If you don’t provide native functions to access the file system, a user’s going to have a hell of a time writing a program that reads and displays a file.

Add a new globals environment which will store all the native methods in fixed reference to the global scope:
```java
class Interpreter implements Expr.Visitor<Object>,
                             Stmt.Visitor<Void> {
  final Environment globals = new Environment();
  private Environment environment = globals;

  Interpreter() {
    globals.define("clock", new LoxCallable() {
      @Override
      public int arity() { return 0; }

      @Override
      public Object call(Interpreter interpreter,
                         List<Object> arguments) {
        return (double)System.currentTimeMillis() / 1000.0;
      }

      @Override
      public String toString() { return "<native fn>"; }
    });
  }
```
## Function declaration
Updated grammar:
```
declaration    → funDecl
               | varDecl
               | statement ;
			   
funDecl        → "fun" function ;
function       → IDENTIFIER "(" parameters? ")" block ;

parameters     → IDENTIFIER ( "," IDENTIFIER )* ;
```
## Function Objects
The implementation of the call method is as follows:
```java
  @Override
  public Object call(Interpreter interpreter,
                     List<Object> arguments) {
    Environment environment = new Environment(interpreter.globals);
    for (int i = 0; i < declaration.params.size(); i++) {
      environment.define(declaration.params.get(i).lexeme,
          arguments.get(i));
    }

    interpreter.executeBlock(declaration.body, environment);
    return null;
  }
```
1. Functions should encapsulate its parameters, meaning no code outside the function should be able to see them. Create a new environment with access to global environment.
2. Bind the params to the values based in as arguments
3. Execute the body of the function in a block
## Return Statements
In Lox, the body of a function is a list of statements which don’t produce values, so we need dedicated syntax for emitting a result.
```
statement      → exprStmt
               | forStmt
               | ifStmt
               | printStmt
               | returnStmt
               | whileStmt
               | block ;

returnStmt     → "return" expression? ";" ;
```
## Closures
Because the interpreter does not keep the environment surrounding a function around, a closure is essentially a data structure which helps to hold onto surrounding variables where the function is declared. 
```java
fun makeCounter() {
  var i = 0;
  fun count() {
    i = i + 1;
    print i;
  }

  return count;
}

var counter = makeCounter();
counter(); // "1".
counter(); // "2".
```
Here we pass in the current state of the interpreter environment in function declaration semantics:
```java
LoxFunction(Stmt.Function declaration, Environment closure) {
    this.closure = closure;

    this.declaration = declaration;
...
public Void visitFunctionStmt(Stmt.Function stmt) {

    LoxFunction function = new LoxFunction(stmt, environment);

    environment.define(stmt.name.lexeme, function);
...
Environment environment = new Environment(closure);

    for (int i = 0; i < declaration.params.size(); i++) {
```
### Static Scoping 
A variable usage refers to the preceding declaration with the same name in the innermost scope that encloses the expression where the variable is used. It is static scoping because running the program should not affect this.

```java
var a = "global";
{
  fun showA() {
    print a;
  }

  showA();
  var a = "block";
  showA();
}
```
"global" should be printed twice, as `a` refers to the outermost `a` which is the preceding declaration in the innermost scope. Code may not always execute in the textual order with the introduction of functions which can defer it.

**A block is not all the same scope**
It’s like each `var` statement splits the block into two separate scopes, the scope before the variable is declared and the one after, which includes the new variable.
#### Persistent Environments
**Persistent data structures**: unlike the squishy data structures you’re familiar with in imperative programming, a persistent data structure can never be directly modified. Instead, any “modification” to an existing structure produces a brand new object that contains all of the original data and the new modification. The original is left unchanged.
![](https://i.imgur.com/lCJlB9t.png)
### Semantic Analysis for variable bindings
Where a parser tells only if a program is grammatically correct (a _syntactic_ analysis), semantic analysis goes farther and starts to figure out what pieces of the program actually mean. In this case, our analysis will resolve variable bindings. We’ll know not just that an expression _is_ a variable, but _which_ variable it is.
#### Variable resolution pass
After the parser produces the syntax tree, but before the interpreter starts executing it, we’ll do a single walk over the tree to resolve all of the variables it contains.

It walks the tree, visiting each node, but a static analysis is different from a dynamic execution:
- **There are no side effects.** When the static analysis visits a print statement, it doesn’t actually print anything. Calls to native functions or other operations that reach out to the outside world are stubbed out and have no effect.
- **There is no control flow.** Loops are visited only once. Both branches are visited in `if` statements. Logic operators are not short-circuited.