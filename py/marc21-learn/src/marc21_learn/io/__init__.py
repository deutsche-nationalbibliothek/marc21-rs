"""Methods for reading and write MARC21 records."""

from ._polars import read_marc21, scan_marc21

__all__ = [
    "read_marc21",
    "scan_marc21",
]
