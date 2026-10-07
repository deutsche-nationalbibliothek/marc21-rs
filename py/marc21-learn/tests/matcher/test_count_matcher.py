"""Count Matcher Tests."""

from typing import TYPE_CHECKING

import polars as pl
import pytest
from polars.testing import assert_frame_equal

from marc21_learn.io import read_marc21

if TYPE_CHECKING:
    from pathlib import Path


def test_eq_operator(data_dir: Path) -> None:
    """Ensures that the equal operator works correctly."""
    path = data_dir.joinpath("ada.mrc")

    lhs = read_marc21(path, "001 AS `cn`", predicate="#001 == 1")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", predicate="#002 == 0")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", predicate="#035 == 6")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", predicate="#035{ 9? } == 3")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", predicate="#100/1# == 1")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", predicate="#100/* == 1")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", predicate="#099 == 0")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    df = read_marc21(path, "001 AS `cn`", predicate="#001 == 2")
    assert df.is_empty()

    with pytest.raises(ValueError, match=r"position \d:\d"):
        read_marc21(path, "001", predicate="#001==1")


def test_ne_operator(data_dir: Path) -> None:
    """Ensures that the not equal operator works correctly."""
    path = data_dir.joinpath("ada.mrc")

    lhs = read_marc21(path, "001 AS `cn`", predicate="#001 != 2")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", predicate="#002 != 1")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", predicate="#035 != 5")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", predicate="#035{ 9? } != 0")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", predicate="#100/1# != 2")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", predicate="#100/* != 2")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", predicate="#099 != 1")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    df = read_marc21(path, "001 AS `cn`", predicate="#001 != 1")
    assert df.is_empty()

    with pytest.raises(ValueError, match=r"position \d:\d"):
        read_marc21(path, "001", predicate="#001!=1")


def test_ge_operator(data_dir: Path) -> None:
    """Ensures that the greater than or equal operator works correctly."""
    path = data_dir.joinpath("ada.mrc")

    lhs = read_marc21(path, "001 AS `cn`", predicate="#001 >= 0")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", predicate="#001 >= 1")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", predicate="#002 >= 0")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", predicate="#035 >= 5")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", predicate="#035{ 9? } >= 2")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", predicate="#100/1# >= 1")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", predicate="#100/* >= 0")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", predicate="#099 >= 0")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    df = read_marc21(path, "001 AS `cn`", predicate="#001 >= 2")
    assert df.is_empty()

    with pytest.raises(ValueError, match=r"position \d:\d"):
        read_marc21(path, "001", predicate="#001>=1")


def test_gt_operator(data_dir: Path) -> None:
    """Ensures that the greater than operator works correctly."""
    path = data_dir.joinpath("ada.mrc")

    lhs = read_marc21(path, "001 AS `cn`", predicate="#001 > 0")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", predicate="#035 > 5")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", predicate="#035{ 9? } > 2")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", predicate="#100/1# > 0")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", predicate="#100/* > 0")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    df = read_marc21(path, "001 AS `cn`", predicate="#001 > 1")
    assert df.is_empty()

    df = read_marc21(path, "001 AS `cn`", predicate="#099 > 0")
    assert df.is_empty()

    with pytest.raises(ValueError, match=r"position \d:\d"):
        read_marc21(path, "001", predicate="#001>0")


def test_le_operator(data_dir: Path) -> None:
    """Ensures that the less than or equal operator works correctly."""
    path = data_dir.joinpath("ada.mrc")

    lhs = read_marc21(path, "001 AS `cn`", predicate="#001 <= 1")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", predicate="#001 <= 2")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", predicate="#002 <= 1")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", predicate="#035 <= 7")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", predicate="#035{ 9? } <= 3")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", predicate="#100/1# <= 1")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", predicate="#100/* <= 3")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", predicate="#099 <= 0")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    df = read_marc21(path, "001 AS `cn`", predicate="#001 <= 0")
    assert df.is_empty()

    with pytest.raises(ValueError, match=r"position \d:\d"):
        read_marc21(path, "001", predicate="#001<=1")


def test_lt_operator(data_dir: Path) -> None:
    """Ensures that the less than operator works correctly."""
    path = data_dir.joinpath("ada.mrc")

    lhs = read_marc21(path, "001 AS `cn`", predicate="#001 < 2")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", predicate="#035 < 7")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", predicate="#035{ 9? } < 4")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", predicate="#100/1# < 2")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", predicate="#100/* < 3")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    df = read_marc21(path, "001 AS `cn`", predicate="#001 < 1")
    assert df.is_empty()

    df = read_marc21(path, "001 AS `cn`", predicate="#035 < 6")
    assert df.is_empty()

    df = read_marc21(path, "001 AS `cn`", predicate="#099 > 0")
    assert df.is_empty()

    with pytest.raises(ValueError, match=r"position \d:\d"):
        read_marc21(path, "001", predicate="#001<2")
