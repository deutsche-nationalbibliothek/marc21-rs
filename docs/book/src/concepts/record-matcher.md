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


### Tag Matcher

In MARC21, variable fields are uniquely identified by tags. A _tag
matcher_ is an expression used to filter those fields that match a
specific tag.

In its simplest form, only the three numerical digits of a tag are
specified. A match with a tag only exists if these digits exactly match
those of the tag:

```console
$ marc21 count tests/data/ada.mrc --where '001 == "119232022"'
1

$ marc21 count tests/data/ada.mrc --where '002 == "xyz"'
0

```

In order to identify more than one field, a pattern-based comparison
must be performed. Each numerical digit of a tag can be specified by one
of the following variants.

First, a digit can be represented by the wildcard character `.` that
accepts all possible values from `0` to `9`. For example, the following
tag matcher accepts all fields that contains at least one field that
begins with `0` and ends with `8`. The middle position can contain any
digit.

```console
$ marc21 count tests/data/ada.mrc --where '0.8?'
1

```

Furthermore, a digit can also be represented by specifying a class of
possible digits. In the following example, all fields that start with
a zero, have either a two, three, or five in the second position, and
end with a 5 are accepted.

```console
$ marc21 count tests/data/ada.mrc --where '0[235]5?'
1

```

Similar to the character classes of a regular expression, several
consecutive digits within a class can be combined into a range. The
range is inclusive, and the upper interval limit must be greater than
the lower limit. Note that a class can consist of more than one range
expression.

```console
$ marc21 count tests/data/ada.mrc --where '0[2-5]5?'
1

```

A class can also be specified in negated form (`^`). In this case, the
matcher checks that the digit in the corresponding position of the tag
does not originate from the class digits:

```console
$ marc21 count tests/data/ada.mrc --where '04[^0-3]?'
0

```

After all, every options can be used in all positions of a tag matcher.
In the most extreme case, the expressions `...` and `[0-9][0-9][0-9]`
accept every field, while the expression `[^0-9][^0-9][^0-9]` accepts no
fields. In the following example, all fields that begin with any digit
not followed by a `0`and end with a `1`, `2`, `3`, `5`, or `6` are taken
into account.

```console
$ marc21 count tests/data/ada.mrc --where '.[^0][1-356].2 == "sswd"'
1

```





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

