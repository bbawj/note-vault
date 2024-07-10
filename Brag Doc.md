---
title: "Brag Doc"
date: 2022-11-07
lastmod: 2024-07-10
---
# Brag Doc
As learnt from https://jvns.ca/blog/brag-documents/
## Shopee
Project: API auto failure detection, triaging and reporting internal tool
- Roll out for LATAM regions
- Build new service-specific email reporting flow to increase workflow efficiency and standardization for *all* service teams
- Helped to increase success rates from 60% to 90% in the UAT environment
![](https://i.imgur.com/tqHMsB9.png)

## Beep
Project: Lead development of Android EV charging payment application

- Stablisation of the application in the new Spectra POS hardware. Achieved 40% reduction in production-reported defects.
- Helped integrate a new payment platform "Soepay" with our application. 
 - Implemented abstracted Event Bus on top of event bus external library which could only cache a single item, in order to support multiple charging sessions 
 - Implemented and designed a power loss recovery flow. Used on device payment logs as a source of truth which then synchronised with the backend charging service to recover ongoing transactions. This allowed the team to be more confident in deploying for larger scale customers: ![](Pics/Pasted%20image%2020240710204343.png)
 - Implemented a session expiry flow to capture payment when a user's charging session has gone on for too long: ![](Pics/Pasted%20image%2020240710204746.png)
 - Goal: reduce overhead required to support multiple releases. Added github backport actions to help us push critical changes to past releases.
 - Implemented a way to register and log desired event types to be sent to our backend servers. Goal: improve early visibility of indicators that could point to failures.
 - Implemented a way to detect the current network status in order to perform logic such as retries.
 - Debugged and fixed an OOM issue in a recursive network call:![](Pics/Pasted%20image%2020240710205831.png)
 - Goal: easier debugging of events. Refactored our charging session logic into a state machine:![](Pics/Pasted%20image%2020240710210030.png)
- Debugged and fixed an issue with external HTTP library, increasing stability in poor network conditions (such as in basement carparks):![](Pics/Pasted%20image%2020240710203008.png)


