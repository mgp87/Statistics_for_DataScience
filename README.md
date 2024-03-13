# Statistics_for_DataScience

## What are data?

### Data Types

| Category              | Type     | Description                                                                    | Example                |
| --------------------- | -------- | ------------------------------------------------------------------------------ | ---------------------- |
| Numerical (numbers)   | Interval | Numeric scale with meaningful intervals                                        | Temperature in Celsius |
| Numerical (numbers)   | Ratio    | Numeric scale with a true zero point                                           | Height                 |
| Numerical (numbers)   | Discrete | Countable number of values                                                     | Number of students     |
| Categorical (labeled) | Ordinal  | Categories with a natural order, discrete, no necessarily meaningful intervals | Letter grades          |
| Categorical (labeled) | Nominal  | Categories without a natural order, discrete                                   | Music genres           |

##### Examples:

- **Discrete**: Number of items purchased in January
- **Ordinal**: Favorite cities
- **Nominal**: Last five cities visited
- **Ratio**: Land area of a country
- **Interval**: Temperature in Celsius

### Sample vs Population

- **Population**: The entire group we are interested in
- **Sample**: A subset of the population (ideally representative and randomly selected)

##### Examples:

- **Population**: All students in a school, number of snakes in the zoo
- **Sample**: Average of student grades in a classroom, opinion of visitors about the zoo

###### Population params:

```math
\mu, \beta, \sigma^2
```

###### Sample params:

```math
\hat{\mu}, \hat{\beta}, \hat{\sigma}^2
```

```math
\bar{x}, \beta, s^2
```

#### Data visualization

##### Bar plots

Bar plots can show Categorical (nominal or ordinal) and numerical (discrete) data.
[Rust file](/Data_visualization/bar_plot/src/main.rs)

![Bar Plot Example](/Data_visualization/bar_plot/images/bar-plot.png)

##### Box-and-whisker plots

Box-and-whisker plots can show numerical (interval or ratio) data.
[Rust file](/Data_visualization/box-plot/src/main.rs)

![Box-and-whisker Plot Example](/Data_visualization/box-plot/images/box-plot.png)

##### Histograms

Histograms can show numerical data, they are just like bar plots but categories are binned or bucketed.
[Rust file](/Data_visualization/histogram/src/main.rs)

![Histogram Plot](/Data_visualization/histogram/images/histogram.png)

##### Pie charts

Pie charts can show nominal, ordinal or discrete data representing a percentage of the whole for each category in a visual way since they are contained in a circle.
There is a condition on data to be able to use pie charts, the data must sum up 1 (or 100%).
[Rust file](/Data_visualization/pie-chart/src/main.rs)

![Pie Chart Example](/Data_visualization/pie-chart/images/pie-chart.png)
