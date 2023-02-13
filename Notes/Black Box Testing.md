---
title: "Black Box Testing"
date: 2022-11-08
lastmod: 2023-02-12
---
# Black Box Testing
Testing of requirements and specifications

Assumptions:
1. Verifiable requirements (i.e. hire *juniors* on part time basis compared to hire those below 18 years old part time)
2. Testable code

> [!NOTE] Test case design:
> 1. Formulate the equivalence classes
> 2. Break down ECs into boundary values. Remove boundary values which fall into other ECs
> 	- ![](https://i.imgur.com/fZ35b13.png)
> 3. Create valid test cases using permutation of valid boundary values
> 	- ![](https://i.imgur.com/QiSAbai.png)
> 4. Create invalid test cases using permutation of invalid boundary values; *only one parameter can be invalid at one time*
> 	- ![](https://i.imgur.com/wmxT0zZ.png)
> 

## Equivalence Class Testing
*Equivalence Class*: set of values that produce the same output

*Example: We are testing if an alert is sent*

Valid ECs will produce a positive output according to specification (i.e. will send an alert)
Invalid ECs will produce a negative output (i.e. no alert sent).
Error or exception ECs are invalid data ranges or types not within specification (i.e. exception is thrown).

![|600](https://i.imgur.com/ZQD4J5n.png)

![|600](https://i.imgur.com/LJ37vTq.png)

## Boundary Value Testing
For each EC, there are 3 BVs for the 2 ends of the range:
1. On the value
2. Below the value
3. Above the value

Discrete values have no BV.
