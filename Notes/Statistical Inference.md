---
title: "Statistical Inference"
date: 2023-02-22
---
# Statistical Inference
Inference is about predicting an answer given an observation.
## Bayes Rule
![](https://i.imgur.com/0Wm1wow.png)
## Maximum Likelihood Parameter Estimate 
The goal of MLE is to find the optimal way to fit a distribution to the data, i.e. the best distribution (parameters) to maximise the probability of observing our data.
![](https://i.imgur.com/GbzkGDj.png)
### Naïve Bayes Classifier
Take the dimensions of the data as **independent of each other** such that the joint probability of the feature set is broken up into the product of the probabilities of each feature. Naive Bayes is thus naive due to its indiscretion towards these combined probabilities, even though they may in fact be correlated to each other.
![500](https://i.imgur.com/Ty4MF2I.png)
### MLE for Linear Regression
[Linear Regression](Notes/Regression.md#Linear%20Regression) used the intuition of minimizing the least squared error from the data to the model to find the best fit line. Here we can see that finding the maximum likelihood also leads us to the same conclusion:
![](https://i.imgur.com/rvR9jVs.png)
### MLE for Gaussian
![](https://i.imgur.com/qgRt2hd.png)
### MLE for Bernoulli
![](https://i.imgur.com/CJX9UoG.png)
## Maximum a posteriori Estimation
Finding the most likely distribution parameter, given the data.
![500](https://i.imgur.com/JguWcO5.png)
- From the equation, we can see that MAP means maximising the product of the likelihood and the **prior** probability (some known information of the distribution).
## Classification
With our parameters known, we can make classifications on new data.
Will I play orienteering given the forecast? i.e. yes/no given that the new forecast is rainy.
![300](https://i.imgur.com/biubIZ4.png) ![400x300](https://i.imgur.com/jbpZyKV.png)
### Maximum Likelihood Classification
Classify based on the highest likelihood $P(x|y)\forall y$
$$
\begin{align}
P(rainy|y=yes)=\frac{3}{9}\\
P(rainy|y=no)=\frac{2}{5}\\
y_{ML}=NO
\end{align}
$$
### Maximum a Posteriori Classification
Classify based on the highest posterior probability $P(y|x)\forall y$
$$
\begin{align}
P(yes|rainy)=\frac{P(rainy|yes)P(yes)}{P(rainy)}\\
=\frac{(3/9)(9/14)}{5/14}=0.6\\
P(no|rainy)=\frac{P(rainy|no)P(no)}{P(rainy)}\\
=\frac{(2/5)(5/14)}{5/14}=0.4\\
y_{MAP}=YES
\end{align}
$$

### Naive Bayes Classifier
Take each feature as iid, will I go play orienteering given x?:
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
## Bayesian Estimation
ML and MAP produce point estimates for $\theta$, which assumes that the true distribution follows these parameters. Bayes estimation uses the data to estimate a probability distribution for $\theta$ rather than just 1 point estimate. This gives a posterior estimate given the data rather than given the $\theta$.
![](https://i.imgur.com/pRxhWqJ.png)

## References
- https://towardsdatascience.com/a-gentle-introduction-to-maximum-likelihood-estimation-and-maximum-a-posteriori-estimation-d7c318f9d22d