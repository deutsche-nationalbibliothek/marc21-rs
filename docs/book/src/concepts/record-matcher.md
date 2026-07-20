# Record Matcher

A matcher is the most important component of both the command-line tool
`marc21` and the Python extension [polars-marc21]. It allows for the
efficient filtering of records (or their components) based on various
criteria. A _record matcher_ consists of either a [leader matcher] or
a [field matcher], which can be combined into more complex statements
through [Boolean connectives] or [grouping].

## Leader Matcher

The leader matcher allows you to check the elements of the [leader]. The
following fields can be checked:

- `base_address` --- Base address of data (position 12-16)
- `encoding` --- Character coding scheme (position 09)
- `length` --- Record length (position 00-04)
- `status ` --- Record status (position 05)
- `type` --- Type of record (position 06)

A leader matcher expression always consists of the prefix `ldr.`
followed by the field to which the matcher refers. This is followed
by a comparison operator (`==`, `!=`, `>=`, `<=`, `>`, or `<`), which
specifies the type of comparison, and a reference value against the
comparison is to be made. The data type of the reference value must
match the data type of the corresponding leader field; i.e., the base
address and record length can only be compared with a 32-bit unsigned
integer value, and the remaining fields can only be compared with a
single character enclosed in either single or double quotes.

### Examples

Suppose we have the following leader field:

```console,ignore
$ marc21 print tests/data/ada.mrc | egrep '^LDR'
LDR 03612nz  a2200589nc 4500
```

We can test this matcher using the [count] command in combination with
the `--where` option. If the command returns the value `1`, the leader
of this record meets the criterion; otherwise, it does not.

```console
$ marc21 count tests/data/ada.mrc --where 'ldr.base_address > 500'
1

$ marc21 count tests/data/ada.mrc --where 'ldr.encoding != "a"'
0

$ marc21 count tests/data/ada.mrc --where 'ldr.length == 3612'
1

$ marc21 count tests/data/ada.mrc --where "ldr.status == 'n'"
1

$ marc21 count tests/data/ada.mrc --where 'ldr.type != "z"'
0

```


## Field Matcher

_tba_


## Boolean Connectives

_tba_


## Grouping

_tba_


[Boolean connectives]: #boolean-connectives
[count]: ../reference/commands/marc21-count.md
[field matcher]: #field-matcher
[grouping]: #grouping
[leader]: https://www.loc.gov/marc/specifications/specrecstruc.html#leader
[leader matcher]: #leader-matcher
[polars-marc21]: https://pypi.org/project/polars-marc21/

