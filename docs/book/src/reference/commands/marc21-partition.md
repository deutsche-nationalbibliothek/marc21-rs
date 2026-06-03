# marc21-partition(1)

## NAME

*marc21-partition* --- Partition records by values.

## SYNOPSIS

`marc21 partition` [_OPTIONS_] [_PATH_]...

## DESCRIPTION

The partitions are written to the `<outdir>` directory. The filename can
be changed using the `--template` option. By default, the partitions are
saved with the corresponding value and the `.mrc` file extension.

If a record doesn't have the field/subfield, the record won't be written
to a partition. A record with multiple values will be written to each
partition; thus the partitions may not be disjoint. In order to prevent
duplicate records in a partition , all duplicate values of a record will
be removed automatically.

## ARGUMENTS

`<PATH>`
: A MARC-21 Path expression.

## OPTIONS

`--template <string>`
: A template for naming the individual partitions. The placeholder `{}`
is replaced by the value of the path expression. If the template ends
with the suffix `.gz`, the partitions are compressed in Gzip format.

`-o`, `--output <path>`
: Write output to `<path>`; by default all partitions are written to the
current working directory.

### FILTER OPTIONS

{{ #include filter-opts.md }}

### COMMON OPTIONS

{{ #include common-opts.md }}

## EXIT STATUS

{{ #include exit-status.md }}

## EXAMPLES

In the following example, all authority records are partitioned based on the date
of the last record transaction (field [005]), using only the year (positions 0
through 3) as the values:

```console
$ marc21 partition -ps '005[0:4]' authorities-gnd-sachbegriff_dnbmarc.mrc.gz -o out
207,505 records, 0 invalid | 111,473 records/s, elapsed: 00:00:01

$ tree out
out
├── 2009.mrc
├── 2010.mrc
├── 2011.mrc
├── 2012.mrc
├── 2013.mrc
├── 2014.mrc
├── 2015.mrc
├── 2016.mrc
├── 2017.mrc
├── 2018.mrc
├── 2019.mrc
├── 2020.mrc
├── 2021.mrc
├── 2022.mrc
├── 2023.mrc
├── 2024.mrc
├── 2025.mrc
└── 2026.mrc

1 directory, 18 files
```

[005]: https://www.loc.gov/marc/authority/ad005.html
