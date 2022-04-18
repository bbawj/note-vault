# Black Box Testing
#testing

Testing of requirements and specifications

Test case design:
1. Formulate the equivalence classes
2. Break down ECs into boundary values. Remove boundary values which fall into other ECs
	1. ![](https://s3.us-west-2.amazonaws.com/secure.notion-static.com/a13ad3fb-3770-475d-86ea-ce4b666052e8/Untitled.png?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Content-Sha256=UNSIGNED-PAYLOAD&X-Amz-Credential=AKIAT73L2G45EIPT3X45%2F20220417%2Fus-west-2%2Fs3%2Faws4_request&X-Amz-Date=20220417T080432Z&X-Amz-Expires=86400&X-Amz-Signature=986cde67946a7e462986813315ab3eae12aeeca249eca2173019d4cfb1e13f00&X-Amz-SignedHeaders=host&response-content-disposition=filename%20%3D%22Untitled.png%22&x-id=GetObject)
3. Create valid test cases using permutation of valid boundary values
	1. ![](https://i.imgur.com/QiSAbai.png)
4. Create invalid test cases using permutation of invalid boundary values; *only one parameter can be invalid at one time*
	1. ![](https://i.imgur.com/wmxT0zZ.png)


## Equivalence Class Testing
*Equivalence Class*: set of values that produce the same output

Valid ECs will produce an output according to specification
Invalid ECs are invalid data ranges not within specification

## Boundary Value Testing
For each EC, there are 3 BVs for the 2 ends of the range:
1. On the value
2. Below the value
3. Above the value