---
title: "Neural Networks"
date: 2023-03-14
---
# Neural Networks
## Perceptron Learning
A method to find separating hyperplanes, for classification:
<iframe width="560" height="315" src="https://www.youtube.com/embed/OFbnpY_k7js" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen></iframe>
- Only limited to linear separable datasets
- Guaranteed to converge
- Initial weights do not affect convergence
A single neuron is limited, we need a multi-layer perceptron network for more than a linear separation:
![](https://i.imgur.com/LmHZ4bu.png)
## Error Backpropagation
A way to iteratively update the weights based on the target output at the final node.
![](https://i.imgur.com/YQyT7EU.png)
- Prone to converge to local minima
- Bad weights can affect convergence