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
## Regex as an alternative
Lexical grammar: the rules for how a programming language groups characters into lexemes.

Regular Language: if the lexical grammar can be defined by regular expressions.
## Scanner algorithm
Use 2 offset variables `start` and `current` to index into the string.
Recognising lexemes can be done with simple match statements.
```java
private void scanToken() {
    char c = advance();
    switch (c) {
      case '(': addToken(LEFT_PAREN); break;
      case ')': addToken(RIGHT_PAREN); break;
      case '{': addToken(LEFT_BRACE); break;
      case '}': addToken(RIGHT_BRACE); break;
      case ',': addToken(COMMA); break;
      case '.': addToken(DOT); break;
      case '-': addToken(MINUS); break;
      case '+': addToken(PLUS); break;
      case ';': addToken(SEMICOLON); break;
      case '*': addToken(STAR); break; 
    }
  }
```
- `advance()` consumes the next character in the source file
```java
  private char advance() {
	return source.charAt(current++);
  }
```
- `addToken()` grabs the text representing the current lexeme and creates a new token corresponding to a specific token type.
```java
private void addToken(TokenType type, Object literal) {
    String text = source.substring(start, current);
    tokens.add(new Token(type, text, literal, line));
  }
```
### Longer Lexemes
To handle scanning longer lexemes, we use a lookahead. After detecting the start of a lexeme, we pass control over to some lexeme specific code that consumes characters until it reaches the end.
### Literals
Strategy is similar to longer lexemes. For strings, we start consuming when we see a ", for numbers, we start when we see a digit.
