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
records encoded in [MARC 21], which is a popular file format used
to exchange bibliographic data between libraries. In particular, the
command line tool `marc21` allows efficient filtering of records and
extraction of data into a rectangular schema. Since the extracted data
is in tabular form, it can be processed with popular frameworks such as
[Polars] or [Tidyverse].

`marc21-rs` is developed by the Metadata Department of the [German
National Library] (DNB). It is used for data analysis and for automating
metadata workflows (data engineering) as part of automatic content
indexing.

## Contributing

All contributors are required to "sign-off" their commits (using
`git commit -s`) to indicate that they have agreed to the [Developer
Certificate of Origin][DCO].

## License

This project is licensed under the [European Union Public License 1.2].

[DCO]: https://developercertificate.org
[European Union Public License 1.2]: ./LICENSE
[German National Library]: https://dnb.de/
[MARC 21]: https://www.loc.gov/marc
[Polars]: https://pola.rs
[Tidyverse]: https://tidyverse.org
