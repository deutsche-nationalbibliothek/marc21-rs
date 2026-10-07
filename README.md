<p align="center">
    <img width="250" height="325" src="https://raw.githubusercontent.com/deutsche-nationalbibliothek/marc21-rs/refs/heads/main/docs/book/src/img/marc21_white_bg.png" />
</p>

<div align="center" markdown="1">

[![Rust](https://github.com/deutsche-nationalbibliothek/marc21-rs/actions/workflows/rust.yaml/badge.svg)](https://github.com/deutsche-nationalbibliothek/marc21-rs/actions/workflows/rust.yaml)
[![docs.rs](https://img.shields.io/docsrs/marc21?label=Documentation)](https://docs.rs/marc21/latest/marc21/)
[![Dependencies](https://deps.rs/repo/github/deutsche-nationalbibliothek/marc21-rs/status.svg)](https://deps.rs/repo/github/deutsche-nationalbibliothek/marc21-rs)
[![crates.io](https://img.shields.io/crates/v/marc21)](https://crates.io/crates/marc21)
[![License](https://img.shields.io/github/license/deutsche-nationalbibliothek/marc21-rs?color=blue)](./LICENSE)

</div>

<hr />

This project provides a toolkit for efficiently processing bibliographic
records encoded in [MARC 21], which is a popular file format used
to exchange bibliographic data between libraries. In particular, the
command line tool `marc21` allows efficient filtering of records and
extraction of data into a rectangular schema. Since the extracted data
is in tabular form, it can be processed with popular frameworks such as
[Polars] or [Tidyverse]. In addition, the Python package [marc21-learn]
provides a [Polars] extension that allows you to use the query syntax to
create a [DataFrame], without using the command line.

`marc21-rs` is developed by the Metadata Department of the [German
National Library] (DNB). It is used for data analysis and for automating
metadata workflows (data engineering) as part of automatic content
indexing.

## Command-Line Interface

The `marc21` tool provides the following commands:

- [check] — Validate records against rule sets
- [concat] — Concatenate records from multiple inputs (alias `cat`)
- [count] — Print the number of records in the input data (alias `cnt`)
- [dedup] — Remove duplicate records from the input
- [describe] —  Creates a frequency table of all subfield codes
- [filter] — Filter records that fulfill a specified condition
- [frequency] — Compute a frequency table of values (alias `freq`)
- [glimpse] — Print a dense preview of a data field
- [grep] — Search for records whose values match a pattern
- [hash] — Compute SHA-256 checksum of records
- [invalid] — Output invalid records that cannot be decoded
- [partition] — Partition records by values
- [print] — Print records in human readable format
- [sample] — Select a random permutation of records
- [select] — Transforms records into CSV or TSV format
- [skosify] — Convert records to SKOS/RDF
- [split] — Split the input into chunks of a given size

Check out the [documentation](https://deutsche-nationalbibliothek.github.io/marc21-rs/marc21-cli/)
to learn more about installing and using the tool.

## Python Integration

The Python package [marc21-learn] uses the query engine to transform MARC21
records into a rectangular [DataFrame]:

```python
from marc21_learn.io import read_marc21

sources =  "authorities-gnd-*.mrc.gz"
query = "001 AS `cn`, 075{ b AS `gndgen` | 2 == 'gndgen' }"
predicate = "ldr.type == 'z'"

df = read_marc21(sources, query, predicate=predicate)
print(df)
```

```default
shape: (10_317_327, 2)
┌────────────┬────────┐
│ cn         ┆ gndgen │
│ ---        ┆ ---    │
│ str        ┆ str    │
╞════════════╪════════╡
│ 040000028  ┆ s      │
│ 040000230  ┆ s      │
│ 040000303  ┆ s      │
│ 040000443  ┆ s      │
│ 040000540  ┆ s      │
│ …          ┆ …      │
│ 1403043981 ┆ u      │
│ 1403044260 ┆ u      │
│ 1403044309 ┆ u      │
│ 1403044899 ┆ u      │
│ 1403044961 ┆ u      │
└────────────┴────────┘
```

Check out the [documentation](https://deutsche-nationalbibliothek.github.io/marc21-rs/marc21-learn/)
to learn more about installing and using the `marc21-learn` toolkit.


## Contributing

All contributors are required to "sign-off" their commits (using
`git commit -s`) to indicate that they have agreed to the [Developer
Certificate of Origin][DCO].

This project uses a strict **no AI** / **no LLM** policy. Please do
not use large language models (LLMs) to create issues, patches, pull
requests, or comments. Although English is the preferred language, you
are welcome to communicate in your native language.

## License

This project is licensed under the [European Union Public License 1.2].

[Bash]: https://www.gnu.org/software/bash/
[DataFrame]: https://docs.pola.rs/user-guide/concepts/data-types-and-structures/#dataframe
[DCO]: https://developercertificate.org
[European Union Public License 1.2]: ./LICENSE
[German National Library]: https://dnb.de/
[MARC 21]: https://www.loc.gov/marc
[Polars]: https://pola.rs
[marc21-learn]: https://pypi.org/project/marc21-learn/
[Tidyverse]: https://tidyverse.org
[ZSH]: https://www.zsh.org

[documentation]: https://deutsche-nationalbibliothek.github.io/marc21-rs/

[check]: https://deutsche-nationalbibliothek.github.io/marc21-rs/reference/commands/marc21-check.html
[concat]: https://deutsche-nationalbibliothek.github.io/marc21-rs/reference/commands/marc21-concat.html
[count]: https://deutsche-nationalbibliothek.github.io/marc21-rs/reference/commands/marc21-count.html
[dedup]: https://deutsche-nationalbibliothek.github.io/marc21-rs/reference/commands/marc21-dedup.html
[describe]: https://deutsche-nationalbibliothek.github.io/marc21-rs/reference/commands/marc21-describe.html
[filter]: https://deutsche-nationalbibliothek.github.io/marc21-rs/reference/commands/marc21-filter.html
[frequency]: https://deutsche-nationalbibliothek.github.io/marc21-rs/reference/commands/marc21-frequency.html
[glimpse]: https://deutsche-nationalbibliothek.github.io/marc21-rs/reference/commands/marc21-glimpse.html
[grep]: https://deutsche-nationalbibliothek.github.io/marc21-rs/reference/commands/marc21-grep.html
[hash]: https://deutsche-nationalbibliothek.github.io/marc21-rs/reference/commands/marc21-hash.html
[invalid]: https://deutsche-nationalbibliothek.github.io/marc21-rs/reference/commands/marc21-invalid.html
[partition]: https://deutsche-nationalbibliothek.github.io/marc21-rs/reference/commands/marc21-partition.html
[print]: https://deutsche-nationalbibliothek.github.io/marc21-rs/reference/commands/marc21-print.html
[sample]: https://deutsche-nationalbibliothek.github.io/marc21-rs/reference/commands/marc21-sample.html
[select]: https://deutsche-nationalbibliothek.github.io/marc21-rs/reference/commands/marc21-select.html
[skosify]: https://deutsche-nationalbibliothek.github.io/marc21-rs/reference/commands/marc21-skosify.html
[split]: https://deutsche-nationalbibliothek.github.io/marc21-rs/reference/commands/marc21-split.html
