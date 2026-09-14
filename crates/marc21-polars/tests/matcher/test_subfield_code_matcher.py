"""Subfield Code Matcher Test Suite."""

from pathlib import Path

import polars as pl
from polars.testing import assert_frame_equal

from polars_marc21 import read_marc21

__all__ = []


def test_subfield_code_matcher(data_dir: Path) -> None:
    """Ensures that the subfield code matcher works as expected."""
    path = data_dir.joinpath("minna.mrc")

    # EXPLICIT MATCHING (SHORT FORM)
    rhs = pl.DataFrame({"column_1": ["040992918"]})
    lhs = read_marc21(path, "001", where="079.q == 'f'")
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    df = read_marc21(path, "001", where="079.x == 'f'")
    assert isinstance(df, pl.DataFrame)
    assert df.is_empty()

    # EXPLICIT MATCHING (LONG FORM)
    rhs = pl.DataFrame({"column_1": ["040992918"]})
    lhs = read_marc21(path, "001", where="079{ q == 'f' }")
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    df = read_marc21(path, "001", where="079{ x == 'f' }")
    assert isinstance(df, pl.DataFrame)
    assert df.is_empty()

    # WILDCARD MATCHING (SHORT FORM)
    rhs = pl.DataFrame({"column_1": ["040992918"]})
    lhs = read_marc21(path, "001", where="079.* == 'f'")
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    df = read_marc21(path, "001", where="079.* == 'x'")
    assert isinstance(df, pl.DataFrame)
    assert df.is_empty()

    # WILDCARD MATCHING (LONG FORM)
    rhs = pl.DataFrame({"column_1": ["040992918"]})
    lhs = read_marc21(path, "001", where="079{ * == 'f' }")
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    df = read_marc21(path, "001", where="079{ * == 'x' }")
    assert isinstance(df, pl.DataFrame)
    assert df.is_empty()

    # CLASS MATCHING (SHORT FORM)
    rhs = pl.DataFrame({"column_1": ["040992918"]})
    lhs = read_marc21(path, "001", where="079.[aq] == 'f'")
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    df = read_marc21(path, "001", where="079.[aq] == 'x'")
    assert isinstance(df, pl.DataFrame)
    assert df.is_empty()

    # CLASS MATCHING (LONG FORM)
    rhs = pl.DataFrame({"column_1": ["040992918"]})
    lhs = read_marc21(path, "001", where="079{ [aq] == 'f' }")
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    df = read_marc21(path, "001", where="079{ [aq] == 'x' }")
    assert isinstance(df, pl.DataFrame)
    assert df.is_empty()

    # CLASS MATCHING (RANGE, SHORT FORM)
    rhs = pl.DataFrame({"column_1": ["040992918"]})
    lhs = read_marc21(path, "001", where="079.[q-t] == 'f'")
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    df = read_marc21(path, "001", where="079{ [a-c] == 'f' }")
    assert isinstance(df, pl.DataFrame)
    assert df.is_empty()

    # CLASS MATCHING (RANGE, LONG FORM)
    rhs = pl.DataFrame({"column_1": ["040992918"]})
    lhs = read_marc21(path, "001", where="079{ [q-t] == 'f' }")
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)
