---
title: "TypeScript"
---
# TypeScript
TypeScript is a superset of JavaScript, with all the features of JavaScript + a type checking layer.
## Why TypeScript
TypeScript helps to combat common errors:

1. Uncalled Functions:
```typescript
function flipCoin() {
  // Meant to be Math.random()
  return Math.random < 0.5;
  //Operator '<' cannot be applied to types '() => number' and 'number'.
}
```
2. Logic errors
```typescript
const value = Math.random() < 0.5 ? "a" : "b";
if (value !== "a") {
  // ...
} else if (value === "b") {
  //This condition will always return 'false' since the types '"a"' and '"b"' have no         
  //overlap.
  // Oops, unreachable
}
```
## Defining Types
#### Type Inference
```typescript
let helloworld = "Hello World" //typescript uses the value as its type
```

This is in contrast to Java which types have to be explicitly defined:
```java
String name = "John";
```

#### Type Aliases
A name for any type
```typescript
type Point = {
  x: number;
  y: number;
};

type ID = number | string;
```

#### Type Interfaces
```typescript
interface User {
  name: string;
  id: number;
}
```

OOP type annotations:
```typescript
class UserAccount {
  name: string;
  id: number;
 
  constructor(name: string, id: number) {
    this.name = name;
    this.id = id;
  }
}
 
const user: User = new UserAccount("Murphy", 1);
```

__An interface is always extendable but an alias is not open for extension:
![](https://i.imgur.com/ioFZmoM.png)

## Composing Types
#### Unions
```typescript
type WindowStates = "open" | "closed" | "minimized";

type LockStates = "locked" | "unlocked";

type PositiveOddNumbersUnderTen = 1 | 3 | 5 | 7 | 9;

function getLength(obj: string | string[]) {
	return obj.length;
}
```

TypeScript will only allow an operation if it is valid for every member of the union. For example, if you have the union `string | number`, you can’t use methods that are only available on string. Use `typeof <x> === "<type>"` to check for types before calling type specific methods.

#### Types with Generics
```typescript
interface Backpack<Type> {
  add: (obj: Type) => void;
  get: () => Type;
}
 
// This line is a shortcut to tell TypeScript there is a
// constant called `backpack`, and to not worry about where it came from.
declare const backpack: Backpack<string>;
 
// object is a string, because we declared it above as the variable part of Backpack.
const object = backpack.get();
 
// Since the backpack variable is a string, you can't pass a number to the add function.
backpack.add(23);
```
## Structural Type System
_In a structural type system, if two objects have the same shape, they are considered to be of the same type._

```typescript
interface Point {
  x: number;
  y: number;
}

function logPoint(p: Point) {
  console.log(`${p.x}, ${p.y}`);
}

const point3 = { x: 12, y: 26, z: 89 };
logPoint(point3); // logs "12, 26"
 
const rect = { x: 33, y: 3, width: 30, height: 80 };
logPoint(rect); // logs "33, 3"
 
const color = { hex: "#187ABF" };
logPoint(color); //fails as x and y are missing

```

An object need only to match all the type attributes for the code to pass (can have more but not less).



## Type Assertions
Sometimes you will have information about the type of a value that TypeScript can’t know about.

For example, if you’re using `document.getElementById`, TypeScript only knows that this will return _some_ kind of `HTMLElement`, but you might know that your page will always have an `HTMLCanvasElement` with a given ID.

In this situation, you can use a _type assertion_ to specify a more specific type: `const myCanvas = document.getElementById("main_canvas") as HTMLCanvasElement;`

Angle-bracket syntax (except if the code is in a `.tsx` file), which is equivalent: `const myCanvas = <HTMLCanvasElement>document.getElementById("main_canvas");`

> [!REMINDER] Runtime behaviour
> Because type assertions are removed at compile-time, there is no runtime checking associated with a type assertion. There won’t be an exception or `null` generated if the type assertion is wrong.

TypeScript only allows type assertions which convert to a _more specific_ or _less specific_ version of a type. This rule prevents “impossible” coercions like: `const x = "hello" as number;`

This rule can be too conservative and will disallow more complex coercions that might be valid. You can use two assertions, first to `any` or `unknown`,then to the desired type:
`const a = (expr as any) as T;`

## Literal Types
The types used in the union above include some literals e.g. "locked", 1

#### Literal Inference (or lack thereof):
When you initialize a variable with an object, TypeScript assumes that the properties of that object might change values later. 

`req.method` must have the type `string`, not `"GET"`:

```typescript
const req = { url: "https://example.com", method: "GET" };

handleRequest(req.url, req.method); 
//error out as type 'string' is not assignable to parameter of type '"GET" | "POST"' for req.method.

// Change 1:

const req = { url: "https://example.com", method: "GET" as "GET" };

// Change 2

handleRequest(req.url, req.method as "GET");
```
