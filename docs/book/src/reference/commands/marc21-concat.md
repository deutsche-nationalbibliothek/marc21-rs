# marc21-concat(1)

## NAME

*marc21-concat* --- Concatenate records from multiple inputs

## SYNOPSIS

`marc21 concat` [_options_] [_path_]...\
`marc21 cat` [_options_] [_path_]...

## DESCRIPTION

The `concat` command is used to combine records from multiple files into
a single file or output (`stdout`).

## EXIT STATUS

* `0` --- Command succeeded.
* `1` --- Command failed.

## EXAMPLES

In the following example, the five files `dnb_all_dnbmarc.1.mrc.gz`
to `dnb_all_dnbmarc.5.mrc.gz` are concatenated into a single file
`DNB.mrc.gz`. Invalid data records are skipped (option `-s`):

```console
$ marc21 concat -s dnb_all_dnbmarc.*.mrc.gz -o DNB.mrc.gz
```
