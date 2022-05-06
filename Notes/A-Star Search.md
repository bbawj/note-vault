# A* Search
Combines [[Greedy Best First Search]] h(n) with [[Uniform Cost Search]] g(n)

Evaluation function $$f(n)=g(n)+h(n)$$
**Remember to take the full path cost in calculating g(n) for a node**

Optimality: Optimal with a *admissible heuristic*
Time Complexity: Exponential in length of solution
Space Complexity: Exponential in length of solution

[Admissible Heuristic](https://en.wikipedia.org/wiki/Admissible_heuristic) for every node n, it underestimates the cost of getting from n to the closest goal node __there is no path from n to a goal that has path cost less than h(n)__. It prevents A* from skipping the optimal solution.

> [!Example] Suppose you're trying to [drive from Chicago to New York](https://maps.google.co.uk/maps?q=Chicago+to+New+York&saddr=Chicago&daddr=New+York&hl=en&ll=41.294317,-80.81543&spn=11.071941,20.302734&sll=41.656497,-82.155762&sspn=11.010616,20.302734&geocode=FWICfwIdGuDG-inty_TQPCwOiDEAwMAJrabgrw%3BFVA6bQIdS8KW-yk7CD_TpU_CiTFi_nfhBo8LyA&t=h&z=6) and your heuristic is what your friends think about geography. If your first friend says, "Hey, Boston is close to New York" (underestimating), then you'll waste time looking at routes via Boston. Before long, you'll realise that any [sensible route from Chicago to Boston](https://maps.google.co.uk/maps?saddr=Chicago&daddr=Boston&hl=en&ll=42.228517,-79.343262&spn=10.912859,20.302734&sll=40.63063,-73.87207&sspn=11.183158,20.302734&geocode=FWICfwIdGuDG-inty_TQPCwOiDEAwMAJrabgrw%3BFZ9WhgIdw7bD-ykbMT0NLWXjiTGg6GIBJL98eA&t=h&mra=ls&z=6) already gets fairly close to New York before reaching Boston and that actually going via Boston just adds more miles. So you'll stop considering routes via Boston and you'll move on to find the optimal route. Your underestimating friend cost you a bit of planning time but, in the end, you found the right route.

Guaranteed to expand no more nodes than UCS: Heuristics guide the search towards the goal node which prevents expansion of redundant nodes. Where heuristic h(n) = 0, it will expand the same number as UCS.

### Example Graphs
![](https://i.imgur.com/3WhlDkF.png)

![](https://i.imgur.com/K8AbMZD.png) 

![](https://i.imgur.com/lymZP66.png)
