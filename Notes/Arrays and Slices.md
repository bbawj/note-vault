---
title: "Arrays and Slices"
---
# Arrays and Slices
## Arrays
![Screenshot 2022-08-24 at 6.33.10 PM](Pics/Screenshot%202022-08-24%20at%206.33.10%20PM.png)
## Slices
![](https://i.imgur.com/pmb83Wv.png)

### Slice reallocation
![](https://i.imgur.com/ORzY1Ky.png)
### Deletion
![](https://i.imgur.com/aXxo47i.png)
### Pitfalls
Since a slice is a pointer to an array, passing a slice into a function will allow the function to modify the original array.

However, if slice reallocation occurs inside this new function, the function will no longer be modifying the original array:
![](https://i.imgur.com/3uoV8dh.png)
To avoid this, we can return the modified array from the func
### Goroutine Unsafety
![](https://i.imgur.com/gSFVh8C.png)
