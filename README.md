# tsvstat

Generates a table of summary statistics in [TSV](https://en.wikipedia.org/wiki/Tab-separated_values) format
given a table of numerical observations in  [TSV](https://en.wikipedia.org/wiki/Tab-separated_values) format.

```
cat tests/test1.tsv | tsvstat.exe
```

## To build

```
$ cargo build
$ cargo tests
$ cargo install
```

## Summary statistics

* count and sum
* min, max and range
* [Mean](https://en.wikipedia.org/wiki/mean)
* [Standard deviation](https://en.wikipedia.org/wiki/Standard_deviation)
* [Skew](https://en.wikipedia.org/wiki/skew)
* [Kurtosis and excess Kurtosis](https://en.wikipedia.org/wiki/kurtosis)


## Future additions

* Distribution tests such as jarque barra normality test
* Covariance table
* Correlation table
* Multiple regression - choose one table as the independent variable or y and the others as dependent variables or x's

### Median, Quartiles and Percentiles

One cannot always calculates the median, quartiles or percentiles for two reasons, first it would require sorting the data first
and secondly it would require holding the entire dataset in memory rather than just streaming inputs.

* Sorting - the code could easilly test if the data is already sorted and then calculate these statistics.
* Memory - perhaps another program could be used for this.
