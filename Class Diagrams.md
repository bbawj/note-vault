# Class Diagrams
## Conceptual Model
![](https://s3.us-west-2.amazonaws.com/secure.notion-static.com/56ef3854-7a1b-4eb4-a66a-bdf5e2db2a88/Untitled.png?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Content-Sha256=UNSIGNED-PAYLOAD&X-Amz-Credential=AKIAT73L2G45EIPT3X45%2F20220418%2Fus-west-2%2Fs3%2Faws4_request&X-Amz-Date=20220418T063747Z&X-Amz-Expires=86400&X-Amz-Signature=22f200d77b729f674f44724517f6428df265388033770ccdcdd385bcb5eaf808&X-Amz-SignedHeaders=host&response-content-disposition=filename%20%3D%22Untitled.png%22&x-id=GetObject)

> [!NOTE] Heuristics for identifying entity objects 
> - Terms that developers or users need to clarify in order to understand the use case • Recurring nouns in the use cases (e.g., Incident) 
> - Real-world entities that the system needs to track (e.g., FieldOfficer, Dispatcher, Resource) 
> - Real-world activities that the system needs to track (e.g., EmergencyOperationsPlan) 
> - Data sources or sinks (e.g., Printer).

> [!NOTE] Heuristics for identifying boundary objects 
> - Identify user interface controls that the user needs to initiate the use case (e.g., ReportEmergencyButton). 
> - Identify forms the users needs to enter data into the system (e.g., EmergencyReportForm).
> - Identify notices and messages the system uses to respond to the user (e.g., AcknowledgmentNotice). 
> - When multiple actors are involved in a use case, identify actor terminals (e.g., DispatcherStation) to refer to the user interface under consideration. 

> [!NOTE] Heuristics for identifying control objects 
>- Identify one control object per use case. 
>- Identify one control object per actor in the use case. 
>- The life span of a control object should cover the extent of the use case or the extent of a user session. If it is difficult to identify the beginning and the end of a control object activation, the corresponding use case probably does not have well-defined entry and exit conditions.

![](https://i.imgur.com/30K0QRU.png)


## Associations
![](https://i.imgur.com/6joWlEo.png)

![](https://i.imgur.com/98E6kbI.png)
