# First Steps

This section provides an overview of working with the `marc21` command
line tool. It demonstrates important commands using simple use cases.
An in-depth explanation of the concepts, in particular the structure of
filter expressions, has been omitted for brevity.

The `marc21` tool provides various [commands] for processing _MARC 21_
records (see `marc21 --help` for a complete list of available commands).

## Concatenate Multiple Files

The [concat] command can be used to combine multiple files into a
single output. In the following example, the authority data files from
the Integrated Authority Files ([GND]) are concatenated into the single
file `GND.mrc.gz`.

```console
$ marc21 concat -ps authorities-gnd-*.mrc.gz -o GND.mrc.gz
10,122,437 records, 0 invalid | 49,035 records/s, elapsed: 00:03:19
```

The `--skip-invalid` (`-s`) option is used to skip invalid  records that
could not be decoded. If the option is not specified, processing will
abort at the first invalid record. In addition, the processing progress
can be displayed with the `--progress` (`-p`) option.

## Filtering Records

The [filter] command extracts those records that fulfill a specified
condition. For example, all records with status `z` _and_ at least
one field `100` with indicators `1` and `#` (space) can be filtered
as follows:

```console
$ marc21 filter -s 'ldr.status == "z" && 100/1#?' DUMP.mrc.gz -o out.mrc
```

### Operators

The comparison operators `==`, `!=`, `>=`, `>`, `<=`, and `<` can be
used for values in selected leader fields, values in control fields, and
values in subfields. Here are a few examples

```console
$ marc21 filter -s '100/1#.a == "Lovelace, Ada"' DUMP.mrc.gz -o out.mrc
$ marc21 filter -s '100/*.a != "Curie, Marie"' DUMP.mrc.gz -o out.mrc
$ marc21 filter -s '001 == "119232022"' DUMP.mrc.gz -o out.mrc
$ marc21 filter -s 'ldr.length > 3000' DUMP.mrc.gz -o out.mrc
$ marc21 filter -s 'ldr.status == "z"' DUMP.mrc.gz -o out.mrc
```

To check whether a value (control field or data field) comes from a
specified list, the `in` operator is used. In contrast, the `not in`
operator checks whether a value is not contained in the list. The
following example tests whether a field `100` exists that has a subfield
`a` with the value _"Curie, Marie"_ **or** _"Lovelace, Ada"_:

```console
$ marc21 filter -s '100/*.a in ["Lovelace, Ada", "Curie, Marie"]' \
    DUMP.mrc.gz -o out.mrc
```

The `=?` operator and, in negated form, `!?` perform a substring search
on subfield values. These operators allow simultaneous searching for
multiple patterns by using the `[]`-notation:

```console
$ marc21 filter -s '100/*.a =? ["Hate", "Love"]' DUMP.mrc.gz -o out.mrc.gz
$ marc21 filter -s '100/1#.a =? "Love"' DUMP.mrc.gz -o out.mrc.gz
```

Subfield values can be checked against one or a set of regular
expressions. The filter expression uses the `=~` operator or the `!~`
operator in negated form. The underlying regex engine does not support
all regex features; please refer to the [specification] to learn
more about the syntax and possible limitations. The following example
searches for all records with a field `533` that contains a subfield `n`
whose value matches the regular expression for an ISBN.

```console
$ marc21 filter -s \
    '533.n =~ "(?i)ISBN(?:-1[03])?(?::?\\s*)?\\s(?:97[89][-\ ]?)?\\d{1,5}[-\\ ]?(?:\\d+[-\\ ]?){2}(?:\\d|X)"' \
    DUMP.mrc.gz -o out.mrc.gz
```

To test whether a subfield value begins with a prefix or not, the `=^`
operator or, in its negated form, the `!^` operator is used:

```console
$ marc21 filter -s '400/1#.a =^ "Love"' DUMP.mrc.gz -o out.mrc.gz
$ marc21 filter -s '400/1#.a =^ ["Hate", "Love"]' DUMP.mrc.gz -o out.mrc.gz
$ marc21 filter -s '400/1#{ [ac] =^ "Count" }' DUMP.mrc.gz -o out.mrc.gz
```

In contrast, the `=$` operator can be used to check whether a subfield
value ends with a specific suffix. Keep in mind that the `$` character
often has a special meaning on the command line and may need to be
escaped.

```console
$ marc21 filter -s '548.4 =$ "/gnd#dateOfBirthAndDeath"' DUMP.mrc.gz -o out.mrc.gz
$ marc21 filter -s '401/1#.a !$ "Ada"' DUMP.mrc.gz -o out.mrc.gz
```

Similarity comparisons between character strings are performed using
the `=*` operator (in negated form `!*`). The normalized [Levenshtein
distance] is calculated between the subfield value and the comparison
value. If this is greater than the specified threshold value, the
comparison is considered a match. The default threshold value is `0.8`
and can be changed using the command line option `--strsim-threshold`:

```console
$ marc21 filter -s --strsim-threshold 0.9 '100/1#.a =* "Lovelace, Bda"' \
    DUMP.mrc.gz -o out.mrc.gz
```

## Counting Records

The number of records contained in the input can be determined using the
[count] command:

```console
$ marc21 count GND.mrc.gz
10329438
```

The `--where` option can be used to count only those records that match
a certain criterion:

```console
$ marc21 count GND.mrc.gz --where 'ldr.type == "z" && 075{ b == "gik" && 2 == "gndspec" }'
179672
```

## Print Records

The [print] command output records in a human-readable format. The
leader, control and data fields are written on a separate line.
Consecutive records are divided by a blank line. The output of the
command can be used in combination with standard UNIX tools such as
`grep`, `cut` or `sed`. In the following example, a single data record
is printed on the console:

```console
$ marc21 print tests/data/ada.mrc --where '100/*.a =? "Love"'
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
035 $z (DE-588a)119232022 $9 v:zg
035 $z (DE-588c)4370325-2 $9 v:zg
040 $a DE-386 $c DE-386 $9 r:DE-576 $b ger $d 1841
042 $a gnd1
043 $c XA-GB
065 $a 28p $2 sswd
065 $a 9.5p $2 sswd
075 $b p $2 gndgen
075 $b piz $2 gndspec
079 $a g $q f $q s $q z $u w $u k $u v
100/1# $a Lovelace, Ada $d 1815-1852
375 $a 2 $2 iso5218
400/1# $a Lovelace, Augusta Ada of $d 1815-1852
400/1# $a Lovelace, Ada Augusta of $d 1815-1852
400/1# $a Byron, Ada $d 1815-1852
400/1# $a Byron King, Augusta Ada $d 1815-1852
400/1# $a King, Augusta Ada $d 1815-1852
400/1# $a King, Ada $d 1815-1852
400/1# $a King-Noel, Augusta Ada $c Countess of Lovelace $d 1815-1852
...
```


[commands]: ../reference/commands/index.md
[concat]: ../reference/commands/marc21-concat.md
[count]: ../reference/commands/marc21-count.md
[filter]: ../reference/commands/marc21-filter.md
[GND]: https://gnd.network
[Levenshtein distance]: https://en.wikipedia.org/wiki/Levenshtein_distance
[print]: ../reference/commands/marc21-print.md
[specification]: https://docs.rs/regex/latest/regex/#syntax


