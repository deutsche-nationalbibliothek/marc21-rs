"""Control Field Matcher Test Suite."""

from pathlib import Path

import polars as pl
from polars.testing import assert_frame_equal

from polars_marc21 import read_marc21

__all__ = []


def test_control_field_matcher(data_dir: Path) -> None:
    """Ensures that the control field matcher works as expected."""
    path = data_dir.joinpath("minna.mrc")

    # EQ
    rhs = pl.DataFrame({"column_1": ["040992918"]})
    lhs = read_marc21(path, "001", where="001 == '040992918'")
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    # NE
    rhs = pl.DataFrame({"column_1": ["040992918"]})
    lhs = read_marc21(path, "001", where="001 != '119232022'")
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    # GE
    rhs = pl.DataFrame({"column_1": ["040992918"]})
    lhs = read_marc21(path, "001", where="001 >= '040992918'")
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    # GT
    rhs = pl.DataFrame({"column_1": ["040992918"]})
    lhs = read_marc21(path, "001", where="001 > '040992917'")
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    # LE
    rhs = pl.DataFrame({"column_1": ["040992918"]})
    lhs = read_marc21(path, "001", where="001 <= '040992918'")
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    # GT
    rhs = pl.DataFrame({"column_1": ["040992918"]})
    lhs = read_marc21(path, "001", where="001 < '040992919'")
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    # RANGE (START:END)
    rhs = pl.DataFrame({"column_1": ["040992918"]})
    lhs = read_marc21(path, "001", where="005[0:4] == '2024'")
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    # RANGE (:END)
    rhs = pl.DataFrame({"column_1": ["040992918"]})
    lhs = read_marc21(path, "001", where="005[:4] > '2023'")
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    # RANGE (START:)
    rhs = pl.DataFrame({"column_1": ["040992918"]})
    lhs = read_marc21(path, "001", where="003[3:] == '101'")
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    # IN-OPERATOR
    predicate = "003 in ['DE-101a','DE101b','DE-101']"
    rhs = pl.DataFrame({"column_1": ["040992918"]})
    lhs = read_marc21(path, "001", where=predicate)
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)
