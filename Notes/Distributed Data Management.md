---
title: "Distributed Data Management"
date: 2023-03-09
---
# Distributed Data Management
## Distributed Transactions
[Transaction Management](Notes/Transaction%20Management.md) for distributed systems. All shards should either commit/abort the same transaction.
### Atomic Commit
The de-facto protocol for atomic commit is *two-phase commit (2PC)*

Approach: use 1 process as the *coordinator* (leader). Given a proposed transaction T, commit if all followers agree to commit. Abort if at least one follower aborts/fails.
![](https://i.imgur.com/BwHNWh3.png)
![](https://i.imgur.com/3SFOabX.png)
Problem: if the process was to fail after the decision was made by the coordinator it will be unable to apply the changes locally in the shard.
- Build a more reliable system by building a replicated state machine within the shard. Replicated log will allow fault tolerance
![](https://i.imgur.com/ng0Qo3J.png)
Problem: replicated cluster failure will cause loss of entire log
- Perform replication across different clusters. With a replica of each shard in each data centre
![](https://i.imgur.com/XVsQGMx.png)
Problem:  coordinator was a single point of failure
- Replicate coordinator in the same way for fault tolerance. Second phase of 2PC is no longer needed as each shard can access the local log for decision (abort/commit) without additional broadcast.
## Distributed Snapshotting
Capturing the global state of a distributed system.
![](https://i.imgur.com/RZ5fBA9.png)
### Consistent Cuts
![](https://i.imgur.com/iRHWINo.png)
Properties:
1. Termination: eventually every process records its state
2. Validity: all recorded states correspond to a consistent cut
### Chandy Lamport Algorithm
Approach: disseminate a special marker to mark events during the cut.
![500](https://i.imgur.com/sLAUnrL.png)
- Channels and processes record state (add to snapshot) until the marker has been received. E.g. channel incoming to p2 continuously records messages until a marker is passed through it
- Snapshot is complete once everyone has seen the marker.
### Epoch-based Snapshotting
For continuous data stream processing, it is difficult to log individual task executions.

Approach: divide computations into epochs, such as stages, and treat them as 1 transaction.
![](https://i.imgur.com/fHmY8bu.png)
The Chandy Lamport algorithm is not enough, as it will capture a lot of in-flight messages. We want to capture just the states which would in itself reflect the effect of these messages.
![](https://i.imgur.com/wOJyL7c.png)
This is done by *epoch alignment*: 
1. Allow all messages to go through until an epoch change marker is introduced
2. On receiving the marker, log the state
3. When a process receiving the marker has multiple channels, prioritise inputs from channels which have not seen the marker until they all see the marker.
4. Terminate once all processes seen the marker.
![500](https://i.imgur.com/tZkXLIj.png)
