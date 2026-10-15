"""Vocabularies used at the German National Library (DNB)."""

from importlib.resources import open_binary
from typing import TYPE_CHECKING, Literal

import polars as pl
import polars.selectors as cs

if TYPE_CHECKING:
    from marc21_learn._typing import UnicodeForm


def dnb_subject_categories(
    *,
    language: Literal["de", "en"] | None = None,
    normalize: UnicodeForm | None = None,
    include_note: bool = False,
) -> pl.DataFrame:
    """
    Subject categories of the German National Library (DNB).

    This function returns the subject categories of the `German National
    Library`_ (DNB). The categories are derived from the 23rd edition of the
    Dewey Decimal Classification. They are used since 2004 to structure the
    `German National Bibliography`_, Series A, B, C, H, and O, and the New
    Release Service. The subject categories are taken from the DNB's
    `Linked Data Service`_ and are returned in NFC normal form by default.

    .. versionadded:: 0.2.0

    .. _`German National Library`: https://dnb.de
    .. _`German National Bibliography`: https://www.dnb.de/EN/nationalbibliografie
    .. _`Linked Data Service`: https://d-nb.info/standards/classification/ddc-sc

    Notes
    -----
    Since the metadata typically links to the notation rather than the
    URI, the `notation` column has been converted to an enum type to
    achieve the best possible performance compared to using strings. This
    may result in conversions (cast) in other datasets. The enum data type
    can then be easily taken from the `notation` column.

    Parameters
    ----------
    language : {'de', 'en'}
        Reduces the vocabulary to the columns relevant to the selected
        language.

    normalize : {'NFC', 'NFKC', 'NFD', 'NFKD'}
        Transliterates all strings in the vocabulary into the specified
        Unicode normal form.

    include_note : bool
        Whether the DataFrame should contain notes or not.

    Returns
    -------
    DataFrame
        The subject categories as a Polars DataFrame

    Raises
    ------
    ValueError
        If an unknown language code or Unicode normal form is given.

    Examples
    --------
    By default, the labels for the concepts are returned in German (de) and
    English (en):

    >>> from marc21_learn.vocabularies import dnb_subject_categories
    >>>
    >>> df = dnb_subject_categories()
    >>> assert df.height == 103
    >>>
    >>> df.head(2) # doctest: +SKIP
    shape: (2, 4)
    ┌─────────────────────────────────┬──────────┬───────────────────────────┬───────────────────────┐
    │ uri                             ┆ notation ┆ label_de                  ┆ label_en              │
    │ ---                             ┆ ---      ┆ ---                       ┆ ---                   │
    │ str                             ┆ enum     ┆ str                       ┆ str                   │
    ╞═════════════════════════════════╪══════════╪═══════════════════════════╪═══════════════════════╡
    │ https://d-nb.info/standards/cl… ┆ 000      ┆ Allgemeines, Wissenschaft ┆ Generalities, science │
    │ https://d-nb.info/standards/cl… ┆ 004      ┆ Informatik                ┆ Computer science      │
    └─────────────────────────────────┴──────────┴───────────────────────────┴───────────────────────┘

    To restrict the columns to one of the languages, you can use the
    `language` parameter. Note that the column name changes from `label_de` to
    `label`.

    >>> from marc21_learn.vocabularies import dnb_subject_categories
    >>>
    >>> df = dnb_subject_categories(language="de")
    >>> df.head()
    shape: (5, 3)
    ┌─────────────────────────────────┬──────────┬─────────────────────────────────┐
    │ uri                             ┆ notation ┆ label                           │
    │ ---                             ┆ ---      ┆ ---                             │
    │ str                             ┆ enum     ┆ str                             │
    ╞═════════════════════════════════╪══════════╪═════════════════════════════════╡
    │ https://d-nb.info/standards/cl… ┆ 000      ┆ Allgemeines, Wissenschaft       │
    │ https://d-nb.info/standards/cl… ┆ 004      ┆ Informatik                      │
    │ https://d-nb.info/standards/cl… ┆ 010      ┆ Bibliografien                   │
    │ https://d-nb.info/standards/cl… ┆ 020      ┆ Bibliotheks- und Informationsw… │
    │ https://d-nb.info/standards/cl… ┆ 030      ┆ Enzyklopädien                   │
    └─────────────────────────────────┴──────────┴─────────────────────────────────┘

    To include the notes for each concept, you must set the `include_note`
    flag:

    >>> from marc21_learn.vocabularies import dnb_subject_categories
    >>>
    >>> df = (
    ...     dnb_subject_categories(language="en", include_note=True)
    ...       .select("notation", "note")
    ...       .head(3)
    ... )
    >>> df
    shape: (3, 2)
    ┌──────────┬──────────────────────────────┐
    │ notation ┆ note                         │
    │ ---      ┆ ---                          │
    │ enum     ┆ str                          │
    ╞══════════╪══════════════════════════════╡
    │ 000      ┆ Includes DDC classes 000-003 │
    │ 004      ┆ Includes DDC classes 004-006 │
    │ 010      ┆ Includes DDC classes 010     │
    └──────────┴──────────────────────────────┘
    """
    # The source file is located in the `data` folder under the `vocab`
    # module. The source file can be updated using the script
    # `scripts/update-dnb-ddc-sc.py`.
    source = open_binary(
        "marc21_learn",
        "vocabularies/data/dnb-ddc-sc.csv.gz",
    )

    df = pl.read_csv(source, schema_overrides={"notation": pl.String})

    # The notations are treated as categorical data to ensure that subsequent
    # queries run as efficiently as possible. Since the list of all notations
    # is known in advance, the data type of the series is set to `Enum`.
    df = df.with_columns(pl.col("notation").cast(pl.Enum(df["notation"])))

    if language:
        if language in ["de", "en"]:
            df = (
                df.drop(cs.starts_with("label") & ~cs.ends_with(language))
                .drop(cs.starts_with("note") & ~cs.ends_with(language))
                .select(pl.all().name.replace(r"_.*$", "$1"))
            )
        else:
            msg = "The vocabulary supports only German (de) and English (en)."
            raise ValueError(msg)

    if not include_note:
        df = df.drop(cs.starts_with("note"))

    if normalize:
        if normalize in ["NFC", "NFKC", "NFD", "NFKD"]:
            df = df.with_columns(cs.string().str.normalize(normalize))
        else:
            msg = (
                f"Invalid Unicode form '{normalize}': "
                "expected 'NFC', 'NFKC', 'NFD', or 'NFKD'."
            )

            raise ValueError(msg)

    return df
