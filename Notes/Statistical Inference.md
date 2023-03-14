---
title: "Statistical Inference"
date: 2023-02-22
---
# Statistical Inference
Inference is about predicting an answer given an observation.
## Bayes Rule
![](https://i.imgur.com/0Wm1wow.png)
## Maximum Likelihood Estimate
The goal of MLE is to find the optimal way to fit a distribution to the data, i.e. the best distribution (parameters) to maximise the probability of observing our data.
![](https://i.imgur.com/GbzkGDj.png)
### Naïve Bayes Classifier
Take the dimensions of the data as **independent of each other** such that the joint probability of the feature set is broken up into the product of the probabilities of each feature. Naive Bayes is thus naive due to its indiscretion towards these combined probabilities, even though they may in fact be correlated to each other.
![500](https://i.imgur.com/Ty4MF2I.png)
#### Examples
Will I play orienteering given the forecast?
![300](https://i.imgur.com/biubIZ4.png) ![400x300](https://i.imgur.com/jbpZyKV.png)
$$
\begin{align}
x&=(sunny, cool, high, true)\\
P(y=yes|forecast=x)&= \frac{Pr(x|y=yes)P(y=yes)}{P(forecast=x)}\\
argmax_{y\in Y}(P(yes|x))&=argmax_{y\in Y}(P(x|yes)P(yes))\\
&=0.005\\
argmax_{y\in Y}(P(no|x))&=argmax_{y\in Y}(P(x|no)P(yes))\\
&=0.021\\
y_{MAP}&=0.021=NO
\end{align}
$$
## Maximum a Posteriori
Finding the most likely distribution parameter, given the data.
![500](https://i.imgur.com/JguWcO5.png)
- From the equation, we can see that MAP means maximising the product of the likelihood and the **prior** probability (some known information of the distribution).
## Bayesian Estimation
ML and MAP produce point estimates for $\theta$, which assumes that the true distribution follows these parameters. Bayes estimation uses the data to estimate a probability distribution for $\theta$ rather than just 1 point estimate. This gives a posterior estimate given the data rather than given the $\theta$.
![](https://i.imgur.com/pRxhWqJ.png)

## References
- https://towardsdatascience.com/a-gentle-introduction-to-maximum-likelihood-estimation-and-maximum-a-posteriori-estimation-d7c318f9d22d