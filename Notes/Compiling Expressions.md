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
## Pratt Parser
We also know we need a table that, given a token type, lets us find
- the function to compile a prefix expression starting with a token of that type,
- the function to compile an infix expression whose left operand is followed by a token of that type, and
- the precedence of an infix expression that uses that token as an operator.
