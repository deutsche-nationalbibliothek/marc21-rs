"""Test Polars Integration."""

from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from pathlib import Path

import polars as pl
from polars.testing import assert_frame_equal

from marc21_learn.io import read_marc21


def test_read_marc21(data_dir: Path) -> None:
    """Test read_marc21 function."""
    lhs = read_marc21(
        data_dir.joinpath("DUMP.mrc.gz"),
        "001, 075{ b | 2 == 'gndgen' }",
    )
    assert isinstance(lhs, pl.DataFrame)

    rhs = pl.from_repr("""
shape: (7, 2)
┌───────────┬──────────┐
│ column_1  ┆ column_2 │
│ ---       ┆ ---      │
│ str       ┆ str      │
╞═══════════╪══════════╡
│ 118540238 ┆ p        │
│ 118572121 ┆ p        │
│ 118607626 ┆ p        │
│ 118632477 ┆ p        │
│ 040992020 ┆ u        │
│ 040992918 ┆ u        │
│ 040993396 ┆ u        │
└───────────┴──────────┘
    """)
    assert isinstance(rhs, pl.DataFrame)

    assert_frame_equal(lhs, rhs)
