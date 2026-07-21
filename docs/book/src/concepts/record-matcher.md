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

- `base_addr` --- Base address of data (position 12-16)
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
$ marc21 count tests/data/ada.mrc --where 'ldr.base_addr > 500'
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

The _field matcher_ allows you to define criteria that must apply to
[variable fields]. There are four different types: The [control field
matcher] operates on control fields, the [data field matcher] on data
fields, the [exists matcher] checks whether a field exists, and the
[count matcher] checks whether a specific number of fields are present.


### Control Field Matcher

A _control field matcher_ consists of four components:

- a [tag matcher] to select the fields
- an optional range to check only substrings
- an comparison operator (`==`, `!=`, `>=`, `>`, `<=`, `<`) or `in`-operator,
- a value (comparison operator) or a list of values (`in`-operator)

In the simplest case, control fields are compared by addressing
the fields using a tag matcher, applying a comparison operator, and
specifying a comparison value.

```console
$ marc21 count tests/data/ada.mrc --where '001 == "119232022"'
1

$ marc21 count tests/data/ada.mrc --where '003 != "DE-101"'
0

```

Some control fields contain fixed-length data elements. To access
individual elements, you can optionally specify a range that defines
the start (inclusive) and end (exclusive). If the value of the field
contains this substring, it is compared to the reference value using the
specified operator.

In the following example, all authority records are filtered where the
date of last transaction (field [005], first 8 characters) is earlier
than January 1, 2025:

```console
$ marc21 count tests/data/ada.mrc --where '001[0:8] < "20260101"'
1

$ marc21 count tests/data/ada.mrc --where '001[:8] < "20260101"'
1

```

Note that, if the start value is omitted (e.g., `004[:4]`), the start
is set to `0`. If the end value is omitted (e.g., `003[3:]`), the end is
automatically set to the length of the corresponding value.

The `in` operator can be used to check whether the value of a control
field comes from a reference list:


```console
$ marc21 count tests/data/ada.mrc --where '003 in ["DE-101", "DE-1979"]'
1

```


### Data Field Matcher

_tba_


### Exists Matcher

_tba_


### Count Matcher

_tba_

## Boolean Connectives

_tba_


## Grouping

_tba_


[leader]: https://www.loc.gov/marc/specifications/specrecstruc.html#leader
[polars-marc21]: https://pypi.org/project/polars-marc21/
[variable fields]: https://www.loc.gov/marc/specifications/specrecstruc.html#varifields
[005]: https://www.loc.gov/marc/authority/ad005.html

[count]: ../reference/commands/marc21-count.md

[Boolean connectives]: #boolean-connectives
[control field matcher]: #control-field-matcher
[count matcher]: #count-matcher
[data field matcher]: #data-field-matcher
[exists matcher]: #exists-matcher
[field matcher]: #field-matcher
[grouping]: #grouping
[leader matcher]: #leader-matcher
[Tag Matcher]: ./tag-matcher.md
