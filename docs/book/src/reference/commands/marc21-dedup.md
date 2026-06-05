# marc21-dedup(1)

## NAME

*marc21-dedup* --- Remove duplicate records from the input

## SYNOPSIS

`marc21 count` [_OPTIONS_] [_PATH_]...

## DESCRIPTION

This command deduplicates records that occur multiple times. Duplicates
are identified by comparing the control number (field [001]) of a
record.

## OPTIONS

### FILTER OPTIONS

{{ #include filter-opts.md }}

### COMMON OPTIONS

{{ #include common-opts.md }}

## EXIT STATUS

{{ #include exit-status.md }}

## EXAMPLES

In the following example, all duplicate records found in the input
files `s1.mrc` and `s2.mrc` are removed and written to the output file
`out.mrc`:

```console
$ marc21 dedup s1.mrc s2.mrc -o out.mrc
```

[001]: https://www.loc.gov/marc/authority/ad001.html
