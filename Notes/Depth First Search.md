# Depth First Search

### Graph Traversal
![](https://i.imgur.com/syk9okN.png)

_Assuming ties are handled in alphabetical order_

Expansion Order:
A > B > C > E > F > G
Final Path:
A > B > C > E > F > G 
### Pseudocode
A recursive implementation of DFS:

**procedure** DFS(_G_, _v_) **is**
    label _v_ as discovered
    **for all** directed edges from _v_ to _w that are_ **in** _G_.adjacentEdges(_v_) **do**
        **if** vertex _w_ is not labeled as discovered **then**
            recursively call DFS(_G_, _w_)

A non-recursive implementation of DFS with worst-case space complexity O(E)

**procedure** DFS_iterative(_G_, _v_) **is**
    let _S_ be a stack
    _S_.push(_v_)
    **while** _S_ is not empty **do**
        _v_ = _S_.pop()
        **if** _v_ is not labeled as discovered **then**
            label _v_ as discovered
            **for all** edges from _v_ to _w_ **in** _G_.adjacentEdges(_v_) **do** 
                _S_.push(_w_)