# Black Box Testing
#testing

Testing of requirements and specifications

Assumptions:
1. Verifiable requirements (i.e. hire *juniors* on part time basis compared to hire those below 18 years old part time)
2. Testable code

Test case design:
1. Formulate the equivalence classes
2. Break down ECs into boundary values. Remove boundary values which fall into other ECs
	- ![](https://s3.us-west-2.amazonaws.com/secure.notion-static.com/a13ad3fb-3770-475d-86ea-ce4b666052e8/Untitled.png?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Content-Sha256=UNSIGNED-PAYLOAD&X-Amz-Credential=AKIAT73L2G45EIPT3X45%2F20220419%2Fus-west-2%2Fs3%2Faws4_request&X-Amz-Date=20220419T110437Z&X-Amz-Expires=86400&X-Amz-Signature=f9c1fb9a45c0c9c1a0a037bf503940b8267e87db0409d229e60c2505338b1f00&X-Amz-SignedHeaders=host&response-content-disposition=filename%20%3D%22Untitled.png%22&x-id=GetObject)
3. Create valid test cases using permutation of valid boundary values
	1. ![](https://i.imgur.com/QiSAbai.png)
4. Create invalid test cases using permutation of invalid boundary values; *only one parameter can be invalid at one time*
	1. ![](https://i.imgur.com/wmxT0zZ.png)


## Equivalence Class Testing
*Equivalence Class*: set of values that produce the same output

*Example: We are testing if an alert is sent*

Valid ECs will produce a positive output according to specification (i.e. will send an alert)
Invalid ECs will produce a negative output (i.e. no alert sent).
Error or exception ECs are invalid data ranges or types not within specification (i.e. exception is thrown).

![](https://s3.us-west-2.amazonaws.com/secure.notion-static.com/b0fcd2f6-9196-49cf-83d6-c4c886bcda82/Untitled.png?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Content-Sha256=UNSIGNED-PAYLOAD&X-Amz-Credential=AKIAT73L2G45EIPT3X45%2F20220419%2Fus-west-2%2Fs3%2Faws4_request&X-Amz-Date=20220419T110915Z&X-Amz-Expires=86400&X-Amz-Signature=a974de6b79ac98e3d7d1b2a33039282f76c9e2570060b49cbee5bf939a18678f&X-Amz-SignedHeaders=host&response-content-disposition=filename%20%3D%22Untitled.png%22&x-id=GetObject)

![](https://s3.us-west-2.amazonaws.com/secure.notion-static.com/c77bf518-1839-457d-ab54-c85b9a30ba56/Untitled.png?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Content-Sha256=UNSIGNED-PAYLOAD&X-Amz-Credential=AKIAT73L2G45EIPT3X45%2F20220419%2Fus-west-2%2Fs3%2Faws4_request&X-Amz-Date=20220419T110939Z&X-Amz-Expires=86400&X-Amz-Signature=c87347900973470b10039aa6406055c3758f64d7f1ebb3f4629e8f0fa6c693d4&X-Amz-SignedHeaders=host&response-content-disposition=filename%20%3D%22Untitled.png%22&x-id=GetObject)

## Boundary Value Testing
For each EC, there are 3 BVs for the 2 ends of the range:
1. On the value
2. Below the value
3. Above the value

Discrete values have no BV.