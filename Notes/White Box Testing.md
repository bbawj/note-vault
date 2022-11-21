---
title: "White Box Testing"
tags: [testing]
date: 2022-11-08
lastmod: 2022-11-21
---
# White Box Testing
#testing 

Testing of implementation details, internal paths and structure. Contrast to [Black Box Testing](Notes/Black%20Box%20Testing.md)

## Control Flow Testing
### The Control Flow Graph
![](https://i.imgur.com/PBUSMPg.png)

![](https://i.imgur.com/T7rt8Ea.png)

#### Cyclomatic Complexity
Measurement of complexity based on the number of decision points.
![](https://i.imgur.com/9aO8asZ.png)

### Test Coverage Levels
#### Statement Coverage
![](https://i.imgur.com/ephQk5f.png)

#### Branch Coverage
![](https://i.imgur.com/fiNmfCC.png)

#### Basis Path Coverage
![](https://i.imgur.com/xFTJNs3.png)

> [!NOTE] Algorithm for creating basis paths
> 1. Select the baseline path: Reasonably "typical" path of execution. If loops exists, take the path which does not enter the loop.
> 2. For every decision point in the baseline path:
> 	1. Change the outcome
> 	2. This is the new basis path

> [!important]
> Infeasible paths may exist due to application logic: minimize them by changing multiple decision points at once.

#### Path Coverage
![](https://s3.us-west-2.amazonaws.com/secure.notion-static.com/79909ad3-336d-4fd4-94e1-2eda5533c461/Untitled.png?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Content-Sha256=UNSIGNED-PAYLOAD&X-Amz-Credential=AKIAT73L2G45EIPT3X45%2F20220417%2Fus-west-2%2Fs3%2Faws4_request&X-Amz-Date=20220417T144444Z&X-Amz-Expires=86400&X-Amz-Signature=a03bc1bb8e8c99a9822d68b1e0956b907cfe8612ad536f2f59e0e7d2c3cc8fa8&X-Amz-SignedHeaders=host&response-content-disposition=filename%20%3D%22Untitled.png%22&x-id=GetObject)
![](https://i.imgur.com/3wkNDjW.png)

