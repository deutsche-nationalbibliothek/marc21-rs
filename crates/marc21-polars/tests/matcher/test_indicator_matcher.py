"""Indicator Matcher Test Suite."""

from pathlib import Path

import polars as pl
from polars.testing import assert_frame_equal

from polars_marc21 import read_marc21

__all__ = []


def test_indicator_matcher(data_dir: Path) -> None:
    """Ensures that the indicator matcher works as expected."""
    path = data_dir.joinpath("minna.mrc")

    # EXPLICIT MATCHING
    rhs = pl.DataFrame({"column_1": ["040992918"]})
    lhs = read_marc21(path, "001", where="083/04.a == '832.6'")
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    df = read_marc21(path, "001", where="083/03.a == '832.6'")
    assert isinstance(df, pl.DataFrame)
    assert df.is_empty()

    # PATTERN BASED MATCHING (WILDCARD)
    rhs = pl.DataFrame({"column_1": ["040992918"]})
    lhs = read_marc21(path, "001", where="083/0..a == '832.6'")
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    rhs = pl.DataFrame({"column_1": ["040992918"]})
    lhs = read_marc21(path, "001", where="083/...a == '832.6'")
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    # PATTERN BASED MATCHING (CLASS)
    rhs = pl.DataFrame({"column_1": ["040992918"]})
    lhs = read_marc21(path, "001", where="083/0[34].a == '832.6'")
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    rhs = pl.DataFrame({"column_1": ["040992918"]})
    lhs = read_marc21(path, "001", where="083/[10][34].a == '832.6'")
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    # PATTERN BASED MATCHING (CLASS, RANGE)
    rhs = pl.DataFrame({"column_1": ["040992918"]})
    lhs = read_marc21(path, "001", where="083/0[2-4].a == '832.6'")
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    rhs = pl.DataFrame({"column_1": ["040992918"]})
    lhs = read_marc21(path, "001", where="083/[0-3][2-4].a == '832.6'")
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    # PATTERN BASED MATCHING (CLASS, NEGATION)
    rhs = pl.DataFrame({"column_1": ["040992918"]})
    lhs = read_marc21(path, "001", where="083/0[^123].a == '832.6'")
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    rhs = pl.DataFrame({"column_1": ["040992918"]})
    lhs = read_marc21(path, "001", where="083/0[^1236-9].a == '832.6'")
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    # PATTERN BASED MATCHING (WILDCARD)
    rhs = pl.DataFrame({"column_1": ["040992918"]})
    lhs = read_marc21(path, "001", where="083/*.a == '832.6'")
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    # "NONE" MATCHING
    rhs = pl.DataFrame({"column_1": ["040992918"]})
    lhs = read_marc21(path, "001", where="079.a == 'g'")
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    rhs = pl.DataFrame({"column_1": ["040992918"]})
    lhs = read_marc21(path, "001", where="079/##.a == 'g'")
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)
