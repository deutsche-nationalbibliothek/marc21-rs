# marc21-print(1)

## NAME

*marc21-print* --- Print records in human readable format

## SYNOPSIS

`marc21 print` [_options_] [_path_]...

## DESCRIPTION

This command print records in human readable format.


## OPTIONS

`--translit <form>`
: Transliterate the output into the specified Unicode normal form.
Possible values: `nfd`, `nfkd`, `nfc`, `nfkc`.

### FILTER OPTIONS

{{ #include filter-opts.md }}

### COMMON OPTIONS

{{ #include common-opts.md }}


## EXIT STATUS

{{ #include exit-status.md }}


## EXAMPLES

The following command prints the record from the file `ada.mrc` to the
console:

```console
$ marc21 print tests/data/ada.mrc
LDR 03612nz  a2200589nc 4500
001 119232022
003 DE-101
005 20250720173911.0
008 950316n||azznnaabn           | aaa    |c
024/7# $a 119232022 $0 http://d-nb.info/gnd/119232022 $2 gnd
035 $a (DE-101)119232022
035 $a (DE-588)119232022
035 $z (DE-588)172642531
035 $z (DE-588a)172642531 $9 v:zg
...
```

