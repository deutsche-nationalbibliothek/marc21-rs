<p align="center">
    <img width="250" height="325" src="https://github.com/user-attachments/assets/4220de0e-a5d9-42f1-b590-a6003f71ffce" />
</p>

<div align="center" markdown="1">

[![CI](https://github.com/deutsche-nationalbibliothek/marc21-rs/actions/workflows/ci.yaml/badge.svg?branch=main)](https://github.com/deutsche-nationalbibliothek/marc21-rs/actions/workflows/ci.yaml)
[![crates.io](https://img.shields.io/crates/v/marc21)](https://crates.io/crates/marc21)
[![docs.rs](https://img.shields.io/docsrs/marc21)](https://docs.rs/marc21/latest/marc21/)
[![License](https://img.shields.io/github/license/deutsche-nationalbibliothek/marc21-rs?color=blue)](./LICENSE)

</div>

<hr />

This project provides a toolkit for efficiently processing bibliographic
records encoded in [MARC-21], which is a popular file format used
to exchange bibliographic data between libraries. In particular, the
command line tool `marc` allows efficient filtering of records and
extraction of data into a rectangular schema. Since the extracted data
is in tabular form, it can be processed with popular frameworks such as
[Polars] ([Python]) or [Tidyverse] ([R]).

## Installation

The binary name of the command-line tool is `marc21`. To install the
tool, [archives with a precompiled binary] are available for Windows,
macOS and Linux. If a Rust toolchain is available, `marc21` can also be
installed using `cargo` with the following command:

```shell
$ cargo install marc21-cli  
```

## Commands

The `marc21` tool provides the following commands:

- `completions` — Generate shell completions (e.g. [Bash] or [ZSH])
- `concat` — Concatenate records from multiple inputs (alias `cat`)
- `count` — Print the number of records in the input data (alias `cnt`)
- `filter` — Filters those records that fulfill a specified condition
- `invalid` — Outputs invalid records that cannot be decoded
- `print` — Print records in human readable format
- `sample` — Selects a random permutation of records
- `split` — Splits the input into chunks of a given size

## Tour

> [!NOTE]
> The following documentation is based on the current development
> version. Features may be shown that will not be released until the
> next version.

The `marc21` program provides various commands for processing MARC-21
data (see `marc21 --help` for a complete list of available commands).
For example, the `concat` command can be used to combine multiple files
into a single output. In the following example the authority data files
from the Integrated Authority Files ([GND]) are concatenated into the
single file `GND.mrc.gz`.

```shell
$ marc21 concat -o GND.mrc.gz \
    authorities-gnd-geografikum_dnbmarc.mrc.gz \
    authorities-gnd-koerperschaft_dnbmarc.mrc.gz \
    authorities-gnd-kongress_dnbmarc.mrc.gz \
    authorities-gnd-person_dnbmarc.mrc.gz \
    authorities-gnd-sachbegriff_dnbmarc.mrc.gz \
    authorities-gnd-werk_dnbmarc.mrc.gz
```

The `filter` command extracts those records that fulfill a specified
condition. For example, all records with status `z` and at least one
field `100` with indicators `1` and `#` (space) can be filtered as
follows:

```shell
$ marc21 filter 'ldr.status == "z" && 100/1#?' DUMP.mrc.gz -o out.mrc
```

The number of records contained in the input can be determined using the
`count` command. The `--where` option can be used to count only those
records that match a certain criterion:

```shell
$ marc21 count GND.mrc.gz \
    --where 'ldr.type == "z" && 075{ b == "gik" && 2 == "gndspec" }'
179672
```

The `print` command output records in a human-readable format. The
leader and fields are written on a separate line. Consecutive records
are divided by a blank line. The output of the command can be used in
combination with standard UNIX tools such as `grep`, `cut` or `sed`. In
the following example, a single data record is printed on the console:

```shell
$ marc21 print tests/data/ada.mrc --where '100/*.a =? "Love"'
LDR 03612nz  a2200589nc 4500
001 119232022
003 DE-101
005 20250720173911.0
008 950316n||azznnaabn           | aaa    |c
024/7# $a 119232022 $0 http://d-nb.info/gnd/119232022 $2 gnd
[...]
100/1# $a Lovelace, Ada $d 1815-1852
[...]
```

The `sample` command can be used to take random samples of a specified
size:

```shell
$ marc21 sample 10 GND.mrc.gz -o samples.mrc.gz
```

### Operators

The comparison operators `==`, `!=`, `>=`, `>`, `<=`, and `<` can be
used for values in selected leader fields, values in control fields, and
values in subfields. Here are a few examples

```shell
$ marc21 filter '100/1#.a == "Lovelace, Ada"' DUMP.mrc.gz -o out.mrc.gz
$ marc21 filter '100/*.a != "Curie, Marie"' DUMP.mrc.gz -o out.mrc.gz
$ marc21 filter '001 == "119232022"' DUMP.mrc.gz -o out.mrc.gz
$ marc21 filter 'ldr.length > 3000' DUMP.mrc.gz -o out.mrc.gz
$ marc21 filter 'ldr.status == "z"' DUMP.mrc.gz -o out.mrc.gz
```

The `=?` operator and, in negated form, `!?` perform a substring search
on subfield values. The operator allows simultaneous searching for
multiple patterns (use `[]` notation).

```shell
$ marc21 filter '100/*.a =? ["Hate", "Love"]' DUMP.mrc.gz -o out.mrc.gz
$ marc21 filter '100/1#.a =? "Love"' DUMP.mrc.gz -o out.mrc.gz
```

Subfield values can be checked against one or a set of regular
expressions. The filter expression uses the `=~` operator or the `!~`
operator in negated form. The underlying regex engine does not support
all regex features; please refer to the [specification] to learn
more about the syntax and possible limitations. The following example
searches for all records with a field `533` that contains a subfield `n`
whose value matches the regular expression for an ISBN.

```shell
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
quoted.

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

### Enable tab completion

`marc21` supports generating completion scripts for [Bash], [Elvish],
[Fish], [PowerShell] and [ZSH]. For example, the following code snippet
can be included in the `.zshrc` in order to enable tab completion in
[ZSH]:

```zsh
if type "marc21" > /dev/null ; then
    source <(marc21 completions zsh)
fi
```

## Contributing

All contributors are required to "sign-off" their commits (using
`git commit -s`) to indicate that they have agreed to the [Developer
Certificate of Origin][DCO].

## License

This project is licensed under the [European Union Public License 1.2].

[archives with a precompiled binary]: https://github.com/deutsche-nationalbibliothek/marc21-rs/releases
[Bash]: https://www.gnu.org/software/bash/
[DCO]: https://developercertificate.org
[Elvish]: https://elv.sh
[European Union Public License 1.2]: ./LICENSE
[Fish]: https://fishshell.com
[GND]: https://gnd.network
[Levenshtein distance]: https://en.wikipedia.org/wiki/Levenshtein_distance
[MARC-21]: https://www.loc.gov/marc
[Polars]: https://pola.rs
[PowerShell]: https://en.wikipedia.org/wiki/PowerShell
[Python]: https://www.python.org
[R]: https://www.r-project.org
[specification]: https://docs.rs/regex/latest/regex/#syntax
[Tidyverse]: https://tidyverse.org
[ZSH]: https://www.zsh.org
