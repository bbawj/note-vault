---
title: "Clean Code"
date: 2022-11-08
lastmod: 2022-11-21
---
# Clean Code - Notes
## Comments
### Explain yourself in code
```go
//check to see if employee is elgiible for full benefits
if (employee.flags & HOURLY_FLAG) && employee.age > 65
```
Create a function to describe the comment:
```go
if (employee.isEligibleForFullBenefits())
```
