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
