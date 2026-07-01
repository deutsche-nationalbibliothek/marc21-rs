# marc21-frequency(1)

## NAME

*marc21-frequency* --- Compute a frequency table of values


## SYNOPSIS

`marc21 frequency` [_OPTIONS_] `<QUERY>` [_PATH_]...\
`marc21 freq` [_OPTIONS_] `<QUERY>` [_PATH_]...

## DESCRIPTION

This command computes a frequency table over all values (columns) of
the given query expression. The resulting frequency table is sorted
in descending order (the most frequent value is printed first). If the
count of two or more subfield values is equal, these lines are given in
lexicographical order. The set of data fields, which are included in the
result of a record, can be restricted by an optional predicate.

## ARGUMENTS

`<QUERY>`
: A MARC-21 query expression.

## OPTIONS

`-u`, `--unique`
: This flag ensures that all values generated for a record are counted
only once in the frequency table.

`-r`, `--reverse`
:  Sort results in reverse order

`-t <n>`, `--threshold <n>`
: Ignore rows with a frequency less than `<n>`.

`-n <n>`, `--num <n>`
: Limit result to the `<n>` most frequent subfield values. The value 0
means no restriction.

`-H`, `--header <header>`
: Insert a header row before the data. The header should be entered as
a comma-separated list. Leading and trailing spaces in each column are
automatically removed.

`--tsv`
: Write output tab-separated (TSV)

`-o`, `--output <path>`
: Write output to `<path>` instead of `stdout`. If the filename ends in
`.tsv` or `.tsv.gz`, the output is automatically saved in TSV format.
The output is gzip-compressed when the filename ends with `.gz`.

### FILTER OPTIONS

{{ #include filter-opts.md }}

### COMMON OPTIONS

{{ #include common-opts.md }}

## EXIT STATUS

{{ #include exit-status.md }}

## EXAMPLES

The following example creates a frequency table based on the year of the
last update (field [005]/00-04).

```
$ marc21 frequency -s -H 'year,count' '005[0:4]' GND.mrc`
year,count
2025,1193157
2024,1131644
2021,854178
2022,848635
2023,760070
2016,734399
2010,564136
2017,522303
2020,498302
2008,465916
2019,423590
2011,423077
2014,422959
2018,375568
2013,295991
2015,245866
2026,221200
2012,135738
2009,104168
```

[005]: https://www.loc.gov/marc/authority/ad005.html
