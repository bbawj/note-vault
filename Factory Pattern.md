# Factory Pattern
#design-patterns 

## Problems we want to solve
1. Decouple class selection and object creation from the place where the object is used.
2. Need to instantiate a set of classes but without knowing exactly which one until runtime.
3. Do not want to expose object creation logic to the client.

![](https://s3.us-west-2.amazonaws.com/secure.notion-static.com/dc077381-5178-4992-be56-75005d0d92b8/Untitled.png?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Content-Sha256=UNSIGNED-PAYLOAD&X-Amz-Credential=AKIAT73L2G45EIPT3X45%2F20220419%2Fus-west-2%2Fs3%2Faws4_request&X-Amz-Date=20220419T153530Z&X-Amz-Expires=86400&X-Amz-Signature=6b361eea9599200a24bbb5ae893e0e8358dc22f3ab37f972a66f760dd7c15c96&X-Amz-SignedHeaders=host&response-content-disposition=filename%20%3D%22Untitled.png%22&x-id=GetObject)
## Pros
1. Encapsulation of object creation
2. Extensibility of classes
3. Can easily change object creation logic without affecting context due to decoupling

## Cons
1. Complexity