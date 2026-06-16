# marc21-concat(1)

## NAME

*marc21-concat* --- Concatenate records from multiple inputs

## SYNOPSIS

`marc21 concat` [_options_] [_path_]...\
`marc21 cat` [_options_] [_path_]...

## DESCRIPTION

The `concat` command is used to combine records from multiple files into
a single file or output (`stdout`).

## OPTIONS

`-a`, `--append`
: Append to the given file, do not overwrite. This option is not
supported when writing to Gzip compressed output. When writing to
`stdout` this flag is ignored.

`--tee <path>`
: Write to the output and the file `<path>` at the same time. This
option can be particularly useful when the output is written to `stdout`
for further processing in a pipeline, but the output is also needed for
following processing step.

### FILTER OPTIONS

{{ #include filter-opts.md }}

### COMMON OPTIONS

{{ #include common-opts.md }}

## EXIT STATUS

{{ #include exit-status.md }}

## EXAMPLES

In the following example, the five files `dnb_all_dnbmarc.1.mrc.gz`
to `dnb_all_dnbmarc.5.mrc.gz` are concatenated into a single file
`DNB.mrc.gz`. Invalid data records are skipped (option `-s`):

```console
$ marc21 concat -s dnb_all_dnbmarc.*.mrc.gz -o DNB.mrc.gz
```
