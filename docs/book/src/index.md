<center>
  <img src="https://github.com/user-attachments/assets/4220de0e-a5d9-42f1-b590-a6003f71ffce" width="250" height="325" />
</center>

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

The source code is licensed under the [European Union Public License 1.2].

[License]: https://github.com/deutsche-nationalbibliothek/marc21-rs/blob/main/LICENSE
[German National Library]: https://dnb.de/
[European Union Public License 1.2]: https://github.com/deutsche-nationalbibliothek/marc21-rs/blob/main/LICENSE
[GND]: https://gnd.network
[MARC 21]: https://www.loc.gov/marc
[Polars]: https://pola.rs
[Tidyverse]: https://tidyverse.org

