---
title: "Control Flow"
date: 2023-05-12
lastmod: 2023-05-14
---
# Control Flow
## Conditional Execution
Adding the if statement to grammar:
```
statement      → exprStmt
               | ifStmt
               | printStmt
               | block ;

ifStmt         → "if" "(" expression ")" statement
               ( "else" statement )? ;
```
In java:
```java
 private Stmt ifStatement() {
    consume(LEFT_PAREN, "Expect '(' after 'if'.");
    Expr condition = expression();
    consume(RIGHT_PAREN, "Expect ')' after if condition."); 

    Stmt thenBranch = statement();
    Stmt elseBranch = null;
    if (match(ELSE)) {
      elseBranch = statement();
    }

    return new Stmt.If(condition, thenBranch, elseBranch);
  }
```
### Dangling Else Problem
The else clause is optional. Most parsers bind the else to the nearest if that precedes it.
![](https://i.imgur.com/tMHaVXZ.png)
## Logical Operators
Updated grammar:
```
expression     → assignment ;
assignment     → IDENTIFIER "=" assignment
               | logic_or ;
logic_or       → logic_and ( "or" logic_and )* ;
logic_and      → equality ( "and" equality )* ;
```
## While loops
Updated grammar:
```
statement      → exprStmt
               | ifStmt
               | printStmt
               | whileStmt
               | block ;

whileStmt      → "while" "(" expression ")" statement ;
```
## For loops
Updated grammar:
```
statement      → exprStmt
               | forStmt
               | ifStmt
               | printStmt
               | whileStmt
               | block ;

forStmt        → "for" "(" ( varDecl | exprStmt | ";" )
                 expression? ";"
                 expression? ")" statement ;
```
The first clause is the initializer. It is executed exactly once, before anything else. It’s usually an expression, but for convenience, we also allow a variable declaration. In that case, the variable is scoped to the rest of the for loop—the other two clauses and the body.

Next is the condition. It’s evaluated once at the beginning of each iteration, including the first. If the result is truthy, it executes the loop body. Otherwise, it bails.

The last clause is the increment. It’s an arbitrary expression that does some work at the end of each loop iteration. The result of the expression is discarded, so it must have a side effect to be useful. In practice, it usually increments a variable.

Any of these clauses can be omitted. Following the closing parenthesis is a statement for the body, which is typically a block
### Desugaring
We don't actually *need* the for loop. It is syntactic sugar for the primitive operations we already have. The for loop can be rewritten to:
```java
{
  var i = 0;
  while (i < 10) {
    print i;
    i = i + 1;
  }
}
```
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
