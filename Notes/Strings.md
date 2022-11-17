---
title: "Strings"
---
# Strings
## Immutability
String assignment will not create a new copy of the string. Only string concatenation will create a new buffer for the string.
```go
func main() { 
	x := "hello"
    y := x  
    // x and y share the same underlying memory 
    y += "world"  
    // now y uses a different buffer  
    // x still uses the same old buffer
    
}
```
Immutability means that individual characters or bytes cannot be reassigned directly.
```go
func main() {  
	str := "hello"

	fmt.Println(str[1]) //101 (ascii of ‘e’)

	str[0] = 'a' //compile error
}
```
We can modify the string by creating a copy:
![](https://i.imgur.com/SkcZeC1.png)
## Goroutine unsafety
![](https://i.imgur.com/7b9GrGz.png)

## Runes
len(str) returns the number of bytes for the string. len(rune(str)) returns the actual number of characters.
![](https://i.imgur.com/pOya9UR.png)
