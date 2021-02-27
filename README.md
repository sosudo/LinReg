# LinReg - A Simple Linear Regression Tool Written In Rust
This program simply takes two arrays (```x``` and ```data```) [such that ```x``` is the x-axis and ```data``` is the y-axis with the data]. <br>
To edit the data, you can simply change the array values in ```main.rs``` on lines 8 and 9, respectively for ```x``` and ```data```. <br>
Both the <code>x</code> array and the <code>data</code> array must contain only integer values. Therefore, please scale your data appropriately.<br>
In other words, if you have the coordinates (1.5, 4) and (6, 2.3), multiply all values by 10 in order to make them integers: (15, 40) and (60, 23).<br>
The program will print out the:
<ul>
      <li>Arithmetic mean of the <code>data</code> array</li>
      <li>Standard deviation of the <code>data</code> array</li>
      <li>Correlation coefficient of all of the given data</li>
      <li>Variance of the <code>data</code> array</li>
      <li>The line of best fit of all of the given data (in linear standard form)</li>
</ul>
The program also creates a file entitiled <code>plot.svg</code>. <br>
This file is a scatter plot of all of the given data, and the expected points of the linear regression when the elements of the <code>x</code> array are plugged into the line of best fit. <br>
The black points are the points of the data given, and the blue points are the points of the line of best fit.
