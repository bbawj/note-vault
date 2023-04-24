---
title: "Scanning"
date: 2023-04-24
---
# Scanning
The first job of an interpreter/compiler is to scan the raw source code as characters and group them into something meaningful.
## Lexemes
A lexeme is smallest sequence of characters which represents something.
`var language = "lox";`
The lexemes here are
- var
- language
- =
- "lox"
- ;
In this grouping process, we can gather other useful information.
### Token type
We can categorize tokens from a raw lexeme by comparing strings, but that is slow. Looking through individual characters should be delegated to the Scanner. The parser on the other hand, just needs to know which *kind* of lexeme it represents. E.g.
```
enum TokenType {
  // Single-character tokens.
  LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE,
  COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,

  // One or two character tokens.
  BANG, BANG_EQUAL,
  EQUAL, EQUAL_EQUAL,
  GREATER, GREATER_EQUAL,
  LESS, LESS_EQUAL,

  // Literals.
  IDENTIFIER, STRING, NUMBER,

  // Keywords.
  AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
  PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,

  EOF
}
```
	