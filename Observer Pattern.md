# Observer Pattern
#design-patterns

## Problems we want to solve
1. Tight coupling due to a 1-many dependency
2. We need a number of dependent objects to update automatically when one object changes state
3. We need an object to notify a number of other objects

## Push / Pull Mechanisms
![](https://s3.us-west-2.amazonaws.com/secure.notion-static.com/2261236d-3321-4176-a26e-0ca29be0b983/Untitled.png?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Content-Sha256=UNSIGNED-PAYLOAD&X-Amz-Credential=AKIAT73L2G45EIPT3X45%2F20220416%2Fus-west-2%2Fs3%2Faws4_request&X-Amz-Date=20220416T035113Z&X-Amz-Expires=86400&X-Amz-Signature=20a135b4316fb0ea7b63d3b0b5409860c562dc86a021bca46037d0aeca855b3b&X-Amz-SignedHeaders=host&response-content-disposition=filename%20%3D%22Untitled.png%22&x-id=GetObject)

## Example Class Diagrams
![](https://s3.us-west-2.amazonaws.com/secure.notion-static.com/81ed9add-fb1b-4aa6-a748-031e060b0bdd/Untitled.png?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Content-Sha256=UNSIGNED-PAYLOAD&X-Amz-Credential=AKIAT73L2G45EIPT3X45%2F20220416%2Fus-west-2%2Fs3%2Faws4_request&X-Amz-Date=20220416T035200Z&X-Amz-Expires=86400&X-Amz-Signature=4435f31c44f173ec54f2a95ba9a3d679c3a0cad273b0394ce61a9cad0300da53&X-Amz-SignedHeaders=host&response-content-disposition=filename%20%3D%22Untitled.png%22&x-id=GetObject)
 
 ![](https://s3.us-west-2.amazonaws.com/secure.notion-static.com/3e3ad946-a0f4-4449-a453-6c119db6253f/Untitled.png?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Content-Sha256=UNSIGNED-PAYLOAD&X-Amz-Credential=AKIAT73L2G45EIPT3X45%2F20220416%2Fus-west-2%2Fs3%2Faws4_request&X-Amz-Date=20220416T035235Z&X-Amz-Expires=86400&X-Amz-Signature=426f3049917e84300f5a0fc827d797e977ee8b265203f5d678e5f7f30d0acf53&X-Amz-SignedHeaders=host&response-content-disposition=filename%20%3D%22Untitled.png%22&x-id=GetObject)

![](https://i.imgur.com/fkAbPLG.png)

![](https://i.imgur.com/70E2KU7.png)

![](https://i.imgur.com/6abQdzd.png)

## Pros
1. Abstracts coupling between Subject and Observer
2. Supports broadcast communication
3. Enable reusability of subjects and observers

## Cons
1. Slower performance
2. Possible unnecessary complexity 