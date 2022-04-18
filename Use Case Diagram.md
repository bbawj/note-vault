# Use Case Diagram
## Associations
### extend
The **extending use case is dependent on the base use case**; it literally extends the behaviour described by the base use case. The base use case should be a fully functional use case in its own right without the extending use case’s additional functionality.

![](https://i.imgur.com/htSCvei.png)

![](https://i.imgur.com/LQfjtAm.png)

### include
A **base use case is dependent on the included use case**; Extract use case fragments that are _duplicated_ in multiple use cases. The included use case cannot stand alone and the original use case is not complete without the included one. This should be used sparingly and only in cases where the duplication is significant and exists by design (rather than by coincidence).