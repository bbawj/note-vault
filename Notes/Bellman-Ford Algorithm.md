---
title: "Bellman-Ford Algorithm"
date: 2023-02-10
---
# Bellman-Ford Algorithm
A single source shortest path algorithm which allows for negative edge weights in the graph unlike [Dijkstra's Algorithm](Notes/Dijkstra's%20Algorithm.md). This algorithm only works if there is no negative cycle in the graph.
<iframe width="560" height="315" src="https://www.youtube.com/embed/obWXjtg0L64" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen></iframe>

```c
function BellmanFord(list vertices, list edges, vertex source) is

    // This implementation takes in a graph, represented as
    // lists of vertices (represented as integers [0..n-1]) and edges,
    // and fills two arrays (distance and predecessor) holding
    // the shortest path from the source to each vertex

    distance := list of size n
    predecessor := list of size n

    // Step 1: initialize graph
    for each vertex v in vertices do

        distance[v] := inf             // Initialize the distance to all vertices to infinity
        predecessor[v] := null         // And having a null predecessor
    
    distance[source] := 0              // The distance from the source to itself is, of course, zero

    // Step 2: relax edges repeatedly
    
    repeat |V|−1 times:
         for each edge (u, v) with weight w in edges do
             if distance[u] + w < distance[v] then
                 distance[v] := distance[u] + w
                 predecessor[v] := u

    // Step 3: check for negative-weight cycles
    for each edge (u, v) with weight w in edges do
        if distance[u] + w < distance[v] then
            // Step 4: find a negative-weight cycle
            negativeloop := [v, u]
            repeat |V|−1 times:
                u := negativeloop[0]
                for each edge (u, v) with weight w in edges do
                    if distance[u] + w < distance[v] then
                        negativeloop := concatenate([v], negativeloop)
            find a cycle in negativeloop, let it be ncycle
            // use any cycle detection algorithm here
            error "Graph contains a negative-weight cycle", ncycle

    return distance, predecessor
```