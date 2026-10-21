"""Test Polars Integration."""

from typing import TYPE_CHECKING

import pytest

if TYPE_CHECKING:
    from pathlib import Path

import polars as pl
from polars.testing import assert_frame_equal

from marc21_learn.io import read_marc21, scan_marc21


def test_scan_marc21_sources(data_dir: Path) -> None:
    """Tests the `sources` argument of the `scan_marc21` function."""
    sources = str(data_dir.joinpath("ada.mrc"))
    df = scan_marc21(sources, "001").collect()
    assert df.height == 1

    sources = str(data_dir.joinpath("DUMP.mrc.gz"))
    df = scan_marc21(sources, "001").collect()
    assert df.height == 7

    with pytest.raises(FileNotFoundError, match=r"No such file"):
        scan_marc21(str(data_dir.joinpath("foo.mrc")), "001")

    with pytest.raises(FileNotFoundError, match=r"No such file"):
        scan_marc21(str(data_dir.joinpath("[ab]da.mrc")), "001")

    sources = str(data_dir.joinpath("[ab]da.mrc"))
    df = scan_marc21(sources, "001", glob=True).collect()
    assert df.height == 1

    sources = str(data_dir.joinpath("ada.mrc*"))
    df = scan_marc21(sources, "001", glob=True).collect()
    assert df.height == 2

    with pytest.raises(FileNotFoundError, match=r"Empty glob pattern"):
        scan_marc21(str(data_dir.joinpath("X*.mrc")), "001", glob=True)

    # path source
    sources = data_dir.joinpath("ada.mrc")
    df = scan_marc21(sources, "001").collect()
    assert df.height == 1

    with pytest.raises(FileNotFoundError, match=r"No such file"):
        scan_marc21(data_dir.joinpath("foo.mrc"), "001")

    sources = data_dir.glob("[ab]da.mrc")
    df = scan_marc21(sources, "001").collect()
    assert df.height == 1

    sources = data_dir.glob("ada.mrc*")
    df = scan_marc21(sources, "001").collect()
    assert df.height == 2

    df = scan_marc21(data_dir.glob("X*.mrc"), "001").collect()
    assert df.is_empty()

    # list path
    sources = [data_dir.joinpath("ada.mrc"), data_dir.joinpath("ada.mrc.gz")]
    df = scan_marc21(sources, "001").collect()
    assert df.height == 2

    with pytest.raises(FileNotFoundError, match=r"No such file"):
        scan_marc21(
            [
                data_dir.joinpath("ada.mrc"),
                data_dir.joinpath("foo.mrc"),
            ],
            "001",
        ).collect()

    sources: list[str] = []
    df = scan_marc21(sources, "001").collect()
    assert df.is_empty()


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
