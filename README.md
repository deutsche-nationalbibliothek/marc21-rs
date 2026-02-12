# marc21

[![CI](https://github.com/deutsche-nationalbibliothek/marc21-rs/actions/workflows/ci.yaml/badge.svg?branch=main)](https://github.com/deutsche-nationalbibliothek/marc21-rs/actions/workflows/ci.yaml)
[![crates.io](https://img.shields.io/crates/v/marc21)](https://crates.io/crates/marc21)
[![docs.rs](https://img.shields.io/docsrs/marc21)](https://docs.rs/marc21/latest/marc21/)
[![License](https://img.shields.io/github/license/deutsche-nationalbibliothek/marc21-rs?color=blue)](./LICENSE)

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

```
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

## Tour

The `marc21` program provides various commands for processing MARC-21
data (see `marc21 --help` for a complete list of available commands).
For example, the `concat` command can be used to combine multiple files
into a single output. In the following example the authority data files
from the Integrated Authority Files ([GND]) are concatenated into the
single file `GND.mrc.gz`.

```console
$ marc21 concat -o GND.mrc.gz \
    authorities-gnd-geografikum_dnbmarc.mrc.gz \
    authorities-gnd-koerperschaft_dnbmarc.mrc.gz \
    authorities-gnd-kongress_dnbmarc.mrc.gz \
    authorities-gnd-person_dnbmarc.mrc.gz \
    authorities-gnd-sachbegriff_dnbmarc.mrc.gz \
    authorities-gnd-werk_dnbmarc.mrc.gz
```

The number of records contained in the input can be determined using the
`count` command:

```console
$ marc21 count GND.mrc.gz
10122437
```

The `filter` command extracts those records that fulfill a specified
condition. For example, all records with status `z` and at least one
field `100` with indicators `1` and `#` (space) can be filtered as
follows:

```console
$ marc21 filter 'ldr.status == 'z' && 100/1#?' DUMP.mrc.gz -o out.mrc
```

The `print` command output records in a human-readable format. The
leader and fields are written on a separate line. Consecutive records
are divided by a blank line. The output of the command can be used in
combination with standard UNIX tools such as `grep`, `cut` or `sed`. In
the following example, a single data record is printed on the console:

```console
$ marc12 print tests/data/ada.mrc
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

```console
$ marc21 sample 10 GND.mrc.gz -o SAMPLES.mrc.gz
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
[MARC-21]: https://www.loc.gov/marc
[Polars]: https://pola.rs
[PowerShell]: https://en.wikipedia.org/wiki/PowerShell
[Python]: https://www.python.org
[R]: https://www.r-project.org
[Tidyverse]: https://tidyverse.org
[ZSH]: https://www.zsh.org
