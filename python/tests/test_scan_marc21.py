"""Checks of `scan_marc21` function."""

import polars as pl
from polars.testing import assert_frame_equal

from polars_marc21 import scan_marc21

__all__ = []


def test_scan_marc21() -> None:
    """Simple `scan_marc21` smoke test."""
    lhs = scan_marc21(
        "../tests/data/DUMP.mrc.gz",
        query="001, 075{ b | 2 == 'gndgen' }",
        header="ppn,gndgen",
    )

    rhs = pl.DataFrame(
        {
            "ppn": [
                "118540238",
                "118572121",
                "118607626",
                "118632477",
                "040992020",
                "040992918",
                "040993396",
            ],
            "gndgen": list("ppppuuu"),
        },
    ).lazy()

    assert_frame_equal(lhs, rhs)
