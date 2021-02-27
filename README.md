# LinReg - A Simple Linear Regression Tool Written In Rust
This program simply takes two arrays (```x``` and ```data```) [such that ```x``` is the x-axis and ```data``` is the y-axis with the data]. <br>
To edit the data, you can simply change the array values in ```main.rs``` on lines 8 and 9, respectively for ```x``` and ```data```. <br>
The program will print out the:
<ul>
      <li>Arithmetic mean of the ```data``` array</li>
      <li>Standard deviation of the ```data``` array</li>
      <li>Correlation coefficient of all of the given data</li>
      <li>Variance of the ```data``` array</li>
      <li>The line of best fit of all of the given data (in linear standard form)</li>
</ul>
The program also creates a file entitiled ```plot.svg```. <br>
This file is a scatter plot of all of the given data, and the expected points of the linear regression when the elements of the ```x``` array are plugged into the line of best fit.