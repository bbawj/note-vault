# White Box Testing
#testing 

Testing of implementation details, internal paths and structure. Contrast to [[Black Box Testing]]

## Control Flow Testing
### The Control Flow Graph
![](https://s3.us-west-2.amazonaws.com/secure.notion-static.com/1e31d566-47fb-4c1a-aba4-719ee37e7eae/Untitled.png?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Content-Sha256=UNSIGNED-PAYLOAD&X-Amz-Credential=AKIAT73L2G45EIPT3X45%2F20220417%2Fus-west-2%2Fs3%2Faws4_request&X-Amz-Date=20220417T143904Z&X-Amz-Expires=86400&X-Amz-Signature=7acaa44bb254592aa6b9a4b38bffa3b5a1ef4fcae0d7c814f0d39d66ed2934cd&X-Amz-SignedHeaders=host&response-content-disposition=filename%20%3D%22Untitled.png%22&x-id=GetObject)

![](https://s3.us-west-2.amazonaws.com/secure.notion-static.com/99052640-4389-467e-b383-8194ae99c301/Untitled.png?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Content-Sha256=UNSIGNED-PAYLOAD&X-Amz-Credential=AKIAT73L2G45EIPT3X45%2F20220417%2Fus-west-2%2Fs3%2Faws4_request&X-Amz-Date=20220417T143906Z&X-Amz-Expires=86400&X-Amz-Signature=d4ca735e4193856bf7ef0577b2f15fb34990b83eda46b2228062307237c20e0d&X-Amz-SignedHeaders=host&response-content-disposition=filename%20%3D%22Untitled.png%22&x-id=GetObject)

![](https://s3.us-west-2.amazonaws.com/secure.notion-static.com/463d1b7f-0b4a-4daf-8cb8-5b4da5609302/Untitled.png?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Content-Sha256=UNSIGNED-PAYLOAD&X-Amz-Credential=AKIAT73L2G45EIPT3X45%2F20220417%2Fus-west-2%2Fs3%2Faws4_request&X-Amz-Date=20220417T143908Z&X-Amz-Expires=86400&X-Amz-Signature=5079ec79df45ce860480834f1e2737cb1f08b314221cc4aa50ee168d5a4525ff&X-Amz-SignedHeaders=host&response-content-disposition=filename%20%3D%22Untitled.png%22&x-id=GetObject)

#### Cyclomatic Complexity
![](https://s3.us-west-2.amazonaws.com/secure.notion-static.com/5713528d-1857-4e99-8f92-38cce88cdab5/Untitled.png?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Content-Sha256=UNSIGNED-PAYLOAD&X-Amz-Credential=AKIAT73L2G45EIPT3X45%2F20220417%2Fus-west-2%2Fs3%2Faws4_request&X-Amz-Date=20220417T145007Z&X-Amz-Expires=86400&X-Amz-Signature=b8bdff3ddf98f37b93744af951d8b982b9c7e46f5352796b70dee184df1eb16e&X-Amz-SignedHeaders=host&response-content-disposition=filename%20%3D%22Untitled.png%22&x-id=GetObject)

![](https://s3.us-west-2.amazonaws.com/secure.notion-static.com/f891081c-f9ee-45f6-a995-ba72af2e5a21/Untitled.png?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Content-Sha256=UNSIGNED-PAYLOAD&X-Amz-Credential=AKIAT73L2G45EIPT3X45%2F20220417%2Fus-west-2%2Fs3%2Faws4_request&X-Amz-Date=20220417T145009Z&X-Amz-Expires=86400&X-Amz-Signature=42e2a1fc60ab52bfbd31335dcd2e0fca1781cf69bee6c548581b83b63548e7fb&X-Amz-SignedHeaders=host&response-content-disposition=filename%20%3D%22Untitled.png%22&x-id=GetObject)

### Test Coverage Levels
#### Statement Coverage
![](https://s3.us-west-2.amazonaws.com/secure.notion-static.com/c12b233e-1466-4af4-944a-ff45da4aba1c/Untitled.png?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Content-Sha256=UNSIGNED-PAYLOAD&X-Amz-Credential=AKIAT73L2G45EIPT3X45%2F20220417%2Fus-west-2%2Fs3%2Faws4_request&X-Amz-Date=20220417T144439Z&X-Amz-Expires=86400&X-Amz-Signature=452aaa44dbc2796eebb92bcee682615c533bded111fe8dd8cbc2b6bdc35d7871&X-Amz-SignedHeaders=host&response-content-disposition=filename%20%3D%22Untitled.png%22&x-id=GetObject)

#### Branch Coverage
![Level 2](https://s3.us-west-2.amazonaws.com/secure.notion-static.com/aa0cf98e-4ffd-4b7e-a608-d5ff6e62f5fb/Untitled.png?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Content-Sha256=UNSIGNED-PAYLOAD&X-Amz-Credential=AKIAT73L2G45EIPT3X45%2F20220417%2Fus-west-2%2Fs3%2Faws4_request&X-Amz-Date=20220417T144441Z&X-Amz-Expires=86400&X-Amz-Signature=8d27eb8625a50da77fa014b3a57aa620187f42240f50d9003288a499f7fc7c7f&X-Amz-SignedHeaders=host&response-content-disposition=filename%20%3D%22Untitled.png%22&x-id=GetObject)

#### Basis Path Coverage
![](https://s3.us-west-2.amazonaws.com/secure.notion-static.com/6a3eaa97-3834-49c0-bf00-f40e6861ca89/Untitled.png?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Content-Sha256=UNSIGNED-PAYLOAD&X-Amz-Credential=AKIAT73L2G45EIPT3X45%2F20220417%2Fus-west-2%2Fs3%2Faws4_request&X-Amz-Date=20220417T144903Z&X-Amz-Expires=86400&X-Amz-Signature=f499cf2ac4af36528aefc6e7f5b7174e8f1168e9ae5602325b0b463e181bf0a8&X-Amz-SignedHeaders=host&response-content-disposition=filename%20%3D%22Untitled.png%22&x-id=GetObject)

> [!NOTE] Algorithm for creating basis paths
> 1. Select the baseline path: Reasonably "typical" path of execution. If loops exists, take the path which does not enter the loop.
> 2. For every decision point in the baseline path:
> 	1. Change the outcome
> 	2. This is the new basis path

> [!important]
> Infeasible paths may exist due to application logic: minimize them by changing multiple decision points at once.

#### Path Coverage
![](https://s3.us-west-2.amazonaws.com/secure.notion-static.com/79909ad3-336d-4fd4-94e1-2eda5533c461/Untitled.png?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Content-Sha256=UNSIGNED-PAYLOAD&X-Amz-Credential=AKIAT73L2G45EIPT3X45%2F20220417%2Fus-west-2%2Fs3%2Faws4_request&X-Amz-Date=20220417T144444Z&X-Amz-Expires=86400&X-Amz-Signature=a03bc1bb8e8c99a9822d68b1e0956b907cfe8612ad536f2f59e0e7d2c3cc8fa8&X-Amz-SignedHeaders=host&response-content-disposition=filename%20%3D%22Untitled.png%22&x-id=GetObject)

![](https://s3.us-west-2.amazonaws.com/secure.notion-static.com/83f0760e-8a0f-4850-8383-8702aaa0b142/Untitled.png?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Content-Sha256=UNSIGNED-PAYLOAD&X-Amz-Credential=AKIAT73L2G45EIPT3X45%2F20220417%2Fus-west-2%2Fs3%2Faws4_request&X-Amz-Date=20220417T144454Z&X-Amz-Expires=86400&X-Amz-Signature=df18d40fd6947942849fecca7abab37f66e6e17efc361bfba5d93d9ddb707884&X-Amz-SignedHeaders=host&response-content-disposition=filename%20%3D%22Untitled.png%22&x-id=GetObject)
