# marc21

This project provides a toolkit for efficiently processing bibliographic
records encoded in [MARC 21], which is a popular file format used
to exchange bibliographic data between libraries. In particular, the
command line tool `marc` allows efficient filtering of records and
extraction of data into a rectangular schema. Since the extracted data
is in tabular form, it can be processed with popular frameworks such as
[Polars] ([Python]) or [Tidyverse] ([R]).


## Commands

The `marc21` tool provides the following commands:

- `concat` — Concatenate records from multiple inputs (alias `cat`)
- `count` — Print the number of records in the input data (alias `cnt`)
- `invalid` — Outputs invalid records that cannot be decoded
- `print` — Print records in human readable format


## Contributing

All contributors are required to "sign-off" their commits (using
`git commit -s`) to indicate that they have agreed to the [Developer
Certificate of Origin][DCO].

## License

This project is licensed under the [European Union Public License 1.2].

[DCO]: https://developercertificate.org
[European Union Public License 1.2]: ./LICENSE
[MARC 21]: https://www.loc.gov/marc
[Polars]: https://pola.rs
[Python]: https://www.python.org
[R]: https://www.r-project.org
[Tidyverse]: https://tidyverse.org
