---
title: "Regression"
date: 2023-02-12
---
# Regression
The purpose of regression is to derive a good approximation for the function f(x) based on a set of measurements.
## Linear Regression
Take the function as a form of linear combination of variables, where d is the number of variables or dimension.
$$f(x)=\sum_{i=0}^dw_ix_i=w^Tx$$
### Least Squares 
Least squares regression attempts to find the best fit line based on the quantity of w which minimizes the MSE.
#### Mean Square Error (MSE)
One method to measure the error of N samples
![](https://i.imgur.com/kM5L91K.png)
We can measure the distance of each predicted value and the actual observed value. The sum of these distances is called the **Residual Sum of Squares (RSS)**. The RSS depends on the slope and intercept of the line. To minimize the error, we can take the derivative of the RSS to find the minimum point.

Problem: high influence of outliers to the error value.
### Ridge regression
When we only have a small number of data points, least squares will give us an estimate that is high in [variance](2421%20Machine%20Learning.md#Bias%20vs%20Variance). Rather than trying to minimize RSS alone, we can introduce some penalty (bias). We can use the slope of the line (the estimated weights) as the penalty.
![](https://i.imgur.com/LezOqyg.png)
As $\lambda$ increases, the penalty caused by the slope becomes greater. When minimizing the quantity, ridged regression creates a line that has a gradient that is asymptotically close to 0.
### Lasso Regression
Similar to ridge regression, but the minimized term is now:
$$RSS+\lambda\sum_{i=1}^d|w_i|$$
which is the sum of the weights rather than the squared sum of weights. This allows some coefficients to be able to shrink to 0 rather than just being asymptotically close to 0 as $\lambda$ increases.

*Variable selection property*: for models which include a lot of "useless" parameters, the ability to shrink them to 0 makes lasso regression better than ridge regression.
#### Link to bias vs variance trade off 
$$\sum_{i=1}^d|w_i|\le s$$
When s = 0, all weights are zero; the model is extremely simple, predicting a constant and has no variance with the prediction being far from actual value, thus with high bias. 

As s increases, all wi increase from zero toward their least square estimate values:
- Steadily decreasing the training error to the ordinary least square RSS
- Bias decreases as the model continues to better fit training data.
- Values of wi then become more dependent on training data, thus increasing the variance. 
- The test error initially decreases as well, but eventually start increasing due to overfitting to the training data.
## k-NN Regression
Rather than trying to find a best fit line (equation), we use the training data as sample points. For new data, consider the k closest points to a its given value of X and take the average of the values as the prediction
$$f(x)=\frac1k\sum_{x_i\in N_i}y_i$$
![500](https://i.imgur.com/FPskD0S.png)


![](https://i.imgur.com/ObXEPPj.png)
- Non parametric approach may be better if its the true form of the data
- Parametric methods are worse when there are smaller number of observations per predictor and are also less interpretable
## Random Sampling Consensus (RANSAC)
RANSAC allows for a robust model fit to a dataset which contains outliers.
1. Randomly select a sample of points from the data and build the model with this subset
2. Determine the set of data points which are within a distance threshold of the model. This is called the inliers.
3. If the number of inliers is greater than some threshold, re-estimate using all points in this threshold and stop
4. Else select a new random subset and repeat
![300x250](https://i.imgur.com/ILd9fXR.png)![300x250](https://i.imgur.com/Xihs4iv.png)



