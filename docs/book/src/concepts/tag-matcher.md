# Tag Matcher

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

