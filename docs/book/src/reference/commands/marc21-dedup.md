# marc21-dedup(1)

## NAME

*marc21-dedup* --- Remove duplicate records from the input

## SYNOPSIS

`marc21 count` [_OPTIONS_] [_INPUT_]...

## DESCRIPTION

This command deduplicates records that occur multiple times. Duplicates
are identified by comparing the control number (field [001]) of a
record.

## OPTIONS

`--strategy <strategy>`
  : Use the given strategy to determine duplicate records. The `cn`
    strategy (default) is used to distinguish records by the control
    number (field `001`) and `hash` compares the SHA-256 checksums over
    all fields of a record. Note: If a record doesn't contain a control
    number and the `cn` strategy  is selected, the record is ignored and
    won't be written to OUTPUT.

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
