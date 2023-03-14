---
title: "Information Theory"
date: 2023-03-14
lastmod: 2023-03-14
---
# Information Theory
## Shannon Information Content (Surprise)
Can be interpreted as the *surprise* of an outcome. A lower probability outcome is more surprising!
$$Surprise=log_2\frac{1}{p(x)}$$
## Entropy
A measure of uncertainty. It is the expected surprise of an event.
$$
\begin{aligned}
Entropy&= E(Surprise)\\
&=\sum xP(X=x); \ x=surprise\\
&=\sum log\frac{1}{p(x)}\times p(x)\\
&=\sum p(x)\times (log(1)-log(p(x)))\\
&=\sum-p(x)log(p(x))
\end{aligned}
$$

### Information Gain
Can be interpreted as the reduction in entropy (made things more certain/less surprising).
### Example
Drawing 3 cards out of a standard deck of 52 cards with replacement. Win = all cards are the same colour. Lose = not all the same colour.
$$
\begin{aligned}
&P(Win)=2\times(\frac{1}{2})^3=\frac{1}{4}\\
&P(Lose)=\frac{3}{4} \\
&Entropy_{game}= -\frac{1}{4}log(1/4)-\frac{3}{4}log(3/4)=0.811\\
\end{aligned}
$$
What is the information gain in the event you drew 2 cards (i.e. know what 2 of 3 cards suites are)?
$$
\begin{align}
\text{If both are same color}:\\
P(Win)=1/2\\
P{Lose}=1/2\\
Entropy=1\\
\text{If both are different color}:\\
P(Win)=0\\
P(Lose)=1\\
Entropy = 0\\
E(Entropy)= \frac{1}{2}(1)+\frac{1}{2}(0)=0.5\\
Gain=0.811-0.5=0.311
\end{align}
$$
