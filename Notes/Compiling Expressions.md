---
title: "Compiling Expressions"
date: 2023-05-26
---
# Compiling Expressions
## Prefix expressions
```c
static void unary() {
  TokenType operatorType = parser.previous.type;

  // Compile the operand.
  expression();

  // Emit the operator instruction.
  switch (operatorType) {
    case TOKEN_MINUS: emitByte(OP_NEGATE); break;
    default: return; // Unreachable.
  }
}
```
Ensuring precedence: Here, the operand to `-` should be just the `a.b` expression, not the entire `a.b + c`.
```
-a.b + c;
```
When parsing the operand to unary `-`, we need to compile only expressions at a certain precedence level or higher. In jlox’s recursive descent parser we accomplished that by calling into the parsing method for the lowest-precedence expression we wanted to allow (in this case, `call()`). Each method for parsing a specific expression also parsed any expressions of higher precedence too, so that included the rest of the precedence table.
## Infix expressions
```c
static void binary() {
  TokenType operatorType = parser.previous.type;
  ParseRule* rule = getRule(operatorType);
  parsePrecedence((Precedence)(rule->precedence + 1));

  switch (operatorType) {
    case TOKEN_PLUS:          emitByte(OP_ADD); break;
    case TOKEN_MINUS:         emitByte(OP_SUBTRACT); break;
    case TOKEN_STAR:          emitByte(OP_MULTIPLY); break;
    case TOKEN_SLASH:         emitByte(OP_DIVIDE); break;
    default: return; // Unreachable.
  }
}
```
## Vaughan Pratt Parser
We also know we need a table that, given a token type, lets us find
- the function to compile a prefix expression starting with a token of that type,
- the function to compile an infix expression whose left operand is followed by a token of that type, and
- the precedence of an infix expression that uses that token as an operator.

Here’s how the entire function works: At the beginning of `parsePrecedence()`, we look up a prefix parser for the current token. The first token is _always_ going to belong to some kind of prefix expression, by definition. It may turn out to be nested as an operand inside one or more infix expressions, but as you read the code from left to right, the first token you hit always belongs to a prefix expression.

After parsing that, which may consume more tokens, the prefix expression is done. Now we look for an infix parser for the next token. If we find one, it means the prefix expression we already compiled might be an operand for it. But only if the call to `parsePrecedence()` has a `precedence` that is low enough to permit that infix operator.

If the next token is too low precedence, or isn’t an infix operator at all, we’re done. We’ve parsed as much expression as we can. Otherwise, we consume the operator and hand off control to the infix parser we found. It consumes whatever other tokens it needs (usually the right operand) and returns back to `parsePrecedence()`. Then we loop back around and see if the _next_ token is also a valid infix operator that can take the entire preceding expression as its operand. We keep looping like that, crunching through infix operators and their operands until we hit a token that isn’t an infix operator or is too low precedence and stop.
![](https://i.imgur.com/tco7C4Y.png)
Function calls for (-1 + 2) * 3 - -4:
```
expression
| parsePrecedence(PREC_ASSIGNMENT)
| | grouping
| | | expression
| | | | parsePrecedence(PREC_ASSIGNMENT)
| | | | | unary // for "-"
| | | | | | parsePrecedence(PREC_UNARY)
| | | | | | | number
| | | | | binary // for "+"
| | | | | | parsePrecedence(PREC_FACTOR) // PREC_TERM + 1
| | | | | | | number
| | binary // for "*"
| | | parsePrecedence(PREC_UNARY) // PREC_FACTOR + 1
| | | | number
| | binary // for "-"
| | | parsePrecedence(PREC_FACTOR) // PREC_TERM + 1
| | | | unary // for "-"
| | | | | parsePrecedence(PREC_UNARY)
| | | | | | number
```