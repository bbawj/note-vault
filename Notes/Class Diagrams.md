---
title: "Class Diagrams"
---
# Class Diagrams
## Basic Notation
![](https://i.imgur.com/wP2mGKP.png)

![](https://i.imgur.com/epJI95o.png)

## Visibility Modifiers
\+ : public
\- : private 
\# : protected
~ : package private
## Associations
![](https://i.imgur.com/6joWlEo.png)

![](https://i.imgur.com/98E6kbI.png)
## Stereotypes
![](https://i.imgur.com/pNajniM.png)

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

