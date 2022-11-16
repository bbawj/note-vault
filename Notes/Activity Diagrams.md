---
title:"Activity Diagrams"
---
# Activity Diagram
Flow chart of activities performed by the system.

![](https://s3.us-west-2.amazonaws.com/secure.notion-static.com/a03efaeb-7696-4a73-9025-52a49b8ae01c/Untitled.png?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Content-Sha256=UNSIGNED-PAYLOAD&X-Amz-Credential=AKIAT73L2G45EIPT3X45%2F20220419%2Fus-west-2%2Fs3%2Faws4_request&X-Amz-Date=20220419T093515Z&X-Amz-Expires=86400&X-Amz-Signature=99360890ade6c5be0eb6927563eed3b3c65cadd576fba9518db5a7b7d0421583&X-Amz-SignedHeaders=host&response-content-disposition=filename%20%3D%22Untitled.png%22&x-id=GetObject)

#### Swimlanes
_Partition_ an activity diagram to show who is doing which action.

![](https://i.imgur.com/AYMzwqw.png)
#### Parallel Paths
**_Fork_ nodes indicate the start of concurrent flows of control.**

**_Join_ nodes indicate the end of parallel paths.**

In a set of parallel paths, execution along **all parallel paths should be complete before the execution can start on the outgoing control flow of the _join_.**

![](https://nus-cs2113-ay2122s2.github.io/website/book/uml/activityDiagrams/basicNotations/parallelPaths/images/notation.png)

> [!EXAMPLE]
>  In this activity diagram (from an online shop website) the actions _User browses products_ and _System records browsing data_ happen in parallel. Both of them need to finish before the _log out_ action can take place.
> 
> ![](https://nus-cs2113-ay2122s2.github.io/website/book/uml/activityDiagrams/basicNotations/parallelPaths/images/example.png)

## Examples
![](https://i.imgur.com/6F88TWQ.png)

![](https://i.imgur.com/edaWTvS.png)


