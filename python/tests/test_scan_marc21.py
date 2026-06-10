from polars_marc21 import scan_marc21

import polars as pl
from polars.testing import assert_frame_equal


def test_scan_marc21():
    sources = "../tests/data/DUMP.mrc.gz"
    query = "001, 075{ b | 2 == 'gndgen' }"
    header = ["ppn", "gndgen"]

    lhs = scan_marc21(sources, query, header).collect()
    rhs = pl.from_repr(
        """
shape: (7, 2)
┌───────────┬────────┐
│ ppn       ┆ gndgen │
│ ---       ┆ ---    │
│ str       ┆ str    │
╞═══════════╪════════╡
│ 118540238 ┆ p      │
│ 118572121 ┆ p      │
│ 118607626 ┆ p      │
│ 118632477 ┆ p      │
│ 040992020 ┆ u      │
│ 040992918 ┆ u      │
│ 040993396 ┆ u      │
└───────────┴────────┘
    """
    )

    assert_frame_equal(lhs, rhs)
