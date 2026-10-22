"""Exists Matcher Tests."""

from typing import TYPE_CHECKING

import polars as pl
import pytest
from polars.testing import assert_frame_equal

from marc21_learn.io import read_marc21

if TYPE_CHECKING:
    from pathlib import Path


def test_control_field(data_dir: Path) -> None:
    """Ensures that exists predicate works correctly on control fields."""
    path = data_dir.joinpath("ada.mrc")

    lhs = read_marc21(path, "001 AS `cn`", where="001?")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", where="003?")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", where="00[1-9]?")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    df = read_marc21(path, "001 AS `cn`", where="006?")
    assert df.is_empty()


def test_data_field(data_dir: Path) -> None:
    """Ensures that exists predicate works correctly on data fields."""
    path = data_dir.joinpath("ada.mrc")

    lhs = read_marc21(path, "001 AS `cn`", where="035?")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", where="400/1#?")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", where="06[45]?")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    df = read_marc21(path, "001 AS `cn`", where="036?")
    assert df.is_empty()

    with pytest.raises(ValueError, match=r"position \d:\d"):
        read_marc21(path, "001", where="400/1# ?")
