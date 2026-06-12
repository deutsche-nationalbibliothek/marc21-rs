"""Checks of `scan_marc21` function."""
from pathlib import Path

import polars as pl
import pytest
from polars.testing import assert_frame_equal

from polars_marc21 import HeaderLengthError, scan_marc21

__all__ = []


def test_scan_marc21(data_dir: Path) -> None:
    lhs = scan_marc21(
        data_dir.joinpath("DUMP.mrc.gz"),
        "001, 075{ b | 2 == 'gndgen' }",
    ).collect()
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


def test_scan_marc21_sources_str(data_dir: Path) -> None:
    path = str(data_dir.joinpath("DUMP.mrc.gz"))
    df = scan_marc21(path, "001").collect()
    assert isinstance(df, pl.DataFrame)
    assert df.height == 7


def test_scan_marc21_sources_str_glob(data_dir: Path) -> None:
    path = str(data_dir) + "/[am]*.mrc"
    df = scan_marc21(path, "001").collect()
    assert isinstance(df, pl.DataFrame)
    assert df.height == 2


def test_scan_marc21_sources_expand_user(data_dir: Path) -> None:
    user_dir = str(data_dir).replace(str(Path.home()), "~")
    df = scan_marc21(user_dir + "/DUMP.mrc.gz", "001").collect()
    assert isinstance(df, pl.DataFrame)
    assert df.height == 7

    paths = [user_dir + x for x in ["/[am]*.mrc", "/DUMP.mrc.gz"]]
    df = scan_marc21(paths, "001").collect()
    assert isinstance(df, pl.DataFrame)
    assert df.height == 9


def test_scan_marc21_sources_path(data_dir: Path) -> None:
    path = data_dir.joinpath("DUMP.mrc.gz")
    df = scan_marc21(path, "001").collect()
    assert isinstance(df, pl.DataFrame)
    assert df.height == 7


def test_scan_marc21_sources_list_str(data_dir: Path) -> None:
    paths = [
        str(data_dir.joinpath("[am]*.mrc")),
        str(data_dir.joinpath("DUMP.mrc.gz")),
    ]

    df = scan_marc21(paths, "001").collect()
    assert isinstance(df, pl.DataFrame)
    assert df.height == 9


def test_scan_marc21_sources_list_path(data_dir: Path) -> None:
    paths = [
        data_dir.joinpath("DUMP.mrc.gz"),
        data_dir.joinpath("ada.mrc"),
    ]

    df = scan_marc21(paths, "001").collect()
    assert isinstance(df, pl.DataFrame)
    assert df.height == 8


def test_scan_marc21_header(data_dir: Path) -> None:
    """Check the correct usage of the `header` parameter."""
    path = data_dir.joinpath("DUMP.mrc.gz")
    query = "001, 075{ b | 2 == 'gndgen' }"

    # First, the check is made to see if the default column labels
    # are used when no `header` parameter is specified.
    lhs = scan_marc21(path, query).collect()
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

    assert isinstance(lhs, pl.DataFrame)
    assert isinstance(rhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    # The second case checks the processing of a comma-separated
    # list.
    lhs = scan_marc21(path, query, header="ppn, gndgen").collect()
    rhs = pl.from_repr("""
shape: (7, 2)
┌───────────┬──────────┐
│ ppn       ┆ gndgen   │
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

    assert isinstance(lhs, pl.DataFrame)
    assert isinstance(rhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    # The last case checks whether the column names are specified as
    #  a list.
    lhs = scan_marc21(path, query, header=["ppn", "gndgen"]).collect()
    rhs = pl.from_repr("""
shape: (7, 2)
┌───────────┬──────────┐
│ ppn       ┆ gndgen   │
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

    assert isinstance(lhs, pl.DataFrame)
    assert isinstance(rhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    # Check if `HeaderLengthError` is raised when the header length did
    # not match the query width.
    with pytest.raises(HeaderLengthError):
        scan_marc21(path, query, header=["A", "B", "C"])

    with pytest.raises(HeaderLengthError):
        scan_marc21(path, query, header="A")
