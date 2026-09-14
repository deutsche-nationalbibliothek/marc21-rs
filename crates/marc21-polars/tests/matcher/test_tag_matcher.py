"""Tag Matcher Test Suite."""

from pathlib import Path

import polars as pl
from polars.testing import assert_frame_equal

from polars_marc21 import read_marc21

__all__ = []


def test_tag_matcher(data_dir: Path) -> None:
    """Ensures that the tag matcher works as expected."""
    path = data_dir.joinpath("minna.mrc")

    # EXPLICITLY FROM
    rhs = pl.DataFrame({"column_1": ["040992918"]})
    lhs = read_marc21(path, "001", where="001 == '040992918'")
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    # WILDCARD
    rhs = pl.DataFrame({"column_1": ["040992918"]})
    lhs = read_marc21(path, "001", where="06.?")
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    rhs = pl.DataFrame({"column_1": ["040992918"]})
    lhs = read_marc21(path, "001", where="0..?")
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    df = read_marc21(path, "001", where="05.?")
    assert isinstance(df, pl.DataFrame)
    assert df.is_empty()

    # DIGIT CLASS (EXPLICITLY)
    rhs = pl.DataFrame({"column_1": ["040992918"]})
    lhs = read_marc21(path, "001", where="0[45]2.a == 'gnd1'")
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    # DIGIT CLASS (RANGE)
    rhs = pl.DataFrame({"column_1": ["040992918"]})
    lhs = read_marc21(path, "001", where="0[3-5]2.a == 'gnd1'")
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    # DIGIT CLASS (NEGATED)
    rhs = pl.DataFrame({"column_1": ["040992918"]})
    lhs = read_marc21(path, "001", where="0[^5678]2.a == 'gnd1'")
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    # COMPLEX
    rhs = pl.DataFrame({"column_1": ["040992918"]})
    lhs = read_marc21(path, "001", where=".[^5678][1-4].a == 'gnd1'")
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    # EXTREME CASES
    rhs = pl.DataFrame({"column_1": ["040992918"]})
    lhs = read_marc21(path, "001", where="...?")
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    rhs = pl.DataFrame({"column_1": ["040992918"]})
    lhs = read_marc21(path, "001", where="[0-9][0-9][0-9]?")
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)
