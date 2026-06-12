"""
Polars io plugin for reading MARC21 records.

The packages provides `scan_marc21` which project MARC21 records to a
DataFrame. Currently, all columns are returned as strings and may need
to be converted to the required data type afterward
"""

from polars_marc21._scan import HeaderLengthError, scan_marc21

__all__ = (
    "HeaderLengthError",
    "scan_marc21"
)
