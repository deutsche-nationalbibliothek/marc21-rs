# marc21-select(1)

## NAME

*marc21-select* --- Transforms records into CSV or TSV format

## SYNOPSIS

`marc21 select` [_OPTIONS_] `<QUERY>` [_PATH_]...\

## DESCRIPTION

This command allows you to efficiently transform records into a
rectangular table schema. By default, the output is in CSV format.

## ARGUMENTS

`<QUERY>`
: A MARC-21 query expression.

## OPTIONS

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

This example demonstrates how to create a table in CSV format, where
the first column (`cn`) contains the control number of the record,
the second column (`label`) contains the name of the authority record,
and the third column (`gndsys`) contains the GND classification. Since
multiple notations from the GND classification system can be assigned
to a single data record, the output generates multiple rows for these
data records.

```
$ marc21 select -ps --header 'cn,label,gndsys' \
    '001, 150.a, 065{ a | 2 == "sswd" }' DUMP.mrc.gz -o out.csv
207,505 records, 0 invalid | 102,139 records/s, elapsed: 00:00:01  

$ head -10 out.csv
cn,label,gndsys
040000028,A 302 D,31.9b
040000230,Aargauer,17.1
040000303,Abakus,28
040000443,Abbildung,28
040000540,ABC-Schutz,7.15a
040000540,ABC-Schutz,8.4
040000567,ABC-Waffen,8.4
040000656,Abdichtung,31.3b
040000656,Abdichtung,31.6
```
