<p align="center">
    <img width="250" height="320" src="https://raw.githubusercontent.com/deutsche-nationalbibliothek/marc21-rs/refs/heads/main/py/marc21-learn/doc/_static/img/logo_text.png" />
</p>

<div align="center" markdown="1">

[![Python](https://github.com/deutsche-nationalbibliothek/marc21-rs/actions/workflows/marc21-learn-ci.yaml/badge.svg)](https://github.com/deutsche-nationalbibliothek/marc21-rs/actions/workflows/marc21-learn-ci.yaml)
[![PyPI Version](https://img.shields.io/pypi/v/marc21-learn)](https://pypi.org/project/marc21-learn/#history)
[![PyPI Status](https://img.shields.io/pypi/status/marc21-learn)](https://pypi.org/project/marc21-learn/)
[![PyPI License](https://img.shields.io/pypi/l/marc21-learn)](https://pypi.org/project/marc21-learn)

</div>

<hr />

`marc21-learn` is a toolkit that bridges the gap between the [MARC21] data
format and the machine learning library [scikit-learn].

This project is developed by the Metadata Department of the [German National
Library] (DNB). It is used for data analysis and for automating metadata
workflows (data engineering) as part of automatic content indexing.

Check out the [documentation](https://deutsche-nationalbibliothek.github.io/marc21-rs/marc21-learn/)
to learn more about installing and using the `marc21-learn` toolkit.

## Example

The [marc21_learn.io] module provides a [Polars] extension that uses the query
engine of [marc21](https://crates.io/crates/marc21) to transform records into
a rectangular [DataFrame]:

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

## Contributing

All contributors are required to "sign-off" their commits (using `git commit
-s`) to indicate that they have agreed to the [Developer Certificate of
Origin][DCO].

This project uses a strict **no AI** / **no LLM** policy. Please do
not use large language models (LLMs) to create issues, patches, pull
requests, or comments. Although English is the preferred language, you
are welcome to communicate in your native language.

## License

This project is licensed under the [European Union Public License 1.2].


[DataFrame]: https://docs.pola.rs/user-guide/concepts/data-types-and-structures/#dataframe
[DCO]: https://developercertificate.org
[documentation]: https://deutsche-nationalbibliothek.github.io/marc21-rs/marc21-learn/
[European Union Public License 1.2]: ../../LICENSE
[German National Library]: https://dnb.de/
[MARC21]: https://www.loc.gov/marc
[marc21_learn.io]: https://deutsche-nationalbibliothek.github.io/marc21-rs/marc21-learn/reference/io.html#module-marc21_learn.io
[Polars]: https://pola.rs
[scikit-learn]: https://scikit-learn.org
