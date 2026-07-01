"""
Polars io plugin for reading MARC21 records.

The packages provides `scan_marc21` which project MARC21 records to a
LazyFrame. Currently, all columns are returned as strings and may need
to be converted to the required data type afterward. In addition, the
`read_marc21` function is provided, which returns a DataFrame instead of
a LazyFrame.
"""

from polars_marc21._scan import (
    HeaderLengthError,
    read_marc21,
    scan_marc21,
)

__all__ = (
    "HeaderLengthError",
    "read_marc21",
    "scan_marc21",
)
