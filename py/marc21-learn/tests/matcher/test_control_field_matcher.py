"""Control Field Matcher Tests."""

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

    lhs = read_marc21(path, "001 AS `cn`", predicate="001 == '119232022'")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    df = read_marc21(path, "001 AS `cn`", predicate="001 == '119232023'")
    assert df.is_empty()

    with pytest.raises(ValueError, match=r"position \d:\d"):
        read_marc21(path, "001", predicate="001=='119232022'")


def test_ne_operator(data_dir: Path) -> None:
    """Ensures that the not equal operator works correctly."""
    path = data_dir.joinpath("ada.mrc")

    lhs = read_marc21(path, "001 AS `cn`", predicate="003 != 'DE-588'")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    df = read_marc21(path, "001 AS `cn`", predicate="003 != 'DE-101'")
    assert df.is_empty()

    with pytest.raises(ValueError, match=r"position \d:\d"):
        read_marc21(path, "001", predicate="001!='DE-101'")


def test_ge_operator(data_dir: Path) -> None:
    """Ensures that the greater than or equal operator works correctly."""
    path = data_dir.joinpath("ada.mrc")

    lhs = read_marc21(path, "001 AS `cn`", predicate="003 >= 'DE-100'")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", predicate="003 >= 'DE-101'")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    df = read_marc21(path, "001 AS `cn`", predicate="003 >= 'DE-102'")
    assert df.is_empty()

    with pytest.raises(ValueError, match=r"position \d:\d"):
        read_marc21(path, "001", predicate="001>='DE-101'")


def test_gt_operator(data_dir: Path) -> None:
    """Ensures that the greater than operator works correctly."""
    path = data_dir.joinpath("ada.mrc")

    lhs = read_marc21(path, "001 AS `cn`", predicate="003 > 'DE-100'")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    df = read_marc21(path, "001 AS `cn`", predicate="003 > 'DE-101'")
    assert df.is_empty()

    with pytest.raises(ValueError, match=r"position \d:\d"):
        read_marc21(path, "001", predicate="001>'DE-101'")


def test_le_operator(data_dir: Path) -> None:
    """Ensures that the less than or equal operator works correctly."""
    path = data_dir.joinpath("ada.mrc")

    lhs = read_marc21(path, "001 AS `cn`", predicate="003 <= 'DE-102'")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", predicate="003 <= 'DE-101'")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    df = read_marc21(path, "001 AS `cn`", predicate="003 <= 'DE-100'")
    assert df.is_empty()

    with pytest.raises(ValueError, match=r"position \d:\d"):
        read_marc21(path, "001", predicate="001<='DE-101'")


def test_lt_operator(data_dir: Path) -> None:
    """Ensures that the less than operator works correctly."""
    path = data_dir.joinpath("ada.mrc")

    lhs = read_marc21(path, "001 AS `cn`", predicate="003 < 'DE-102'")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    df = read_marc21(path, "001 AS `cn`", predicate="003 < 'DE-101'")
    assert df.is_empty()

    with pytest.raises(ValueError, match=r"position \d:\d"):
        read_marc21(path, "001", predicate="001<'DE-102'")


def test_in_operator(data_dir: Path) -> None:
    """Ensures that the in operator works correctly."""
    path = data_dir.joinpath("ada.mrc")

    predicate = "003 in ['DE-102', 'DE-101']"
    lhs = read_marc21(path, "001 AS `cn`", predicate=predicate)
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    predicate = "003 in ['DE-102', 'DE-103']"
    df = read_marc21(path, "001 AS `cn`", predicate=predicate)
    assert df.is_empty()


def test_not_in_operator(data_dir: Path) -> None:
    """Ensures that the not in operator works correctly."""
    path = data_dir.joinpath("ada.mrc")

    predicate = "003 not in ['DE-102', 'DE-103']"
    lhs = read_marc21(path, "001 AS `cn`", predicate=predicate)
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    predicate = "003 not in ['DE-101', 'DE-102']"
    df = read_marc21(path, "001 AS `cn`", predicate=predicate)
    assert df.is_empty()


def test_range_expressions(data_dir: Path) -> None:
    """Ensures that range expressions works correctly."""
    path = data_dir.joinpath("ada.mrc")

    lhs = read_marc21(path, "001 AS `cn`", predicate="005[0:4] == '2025'")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", predicate="005[:4] == '2025'")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", predicate="005[4:6] == '07'")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    predicate = "005[8:16] == '173911.0'"
    lhs = read_marc21(path, "001 AS `cn`", predicate=predicate)
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    predicate = "005[8:] == '173911.0'"
    lhs = read_marc21(path, "001 AS `cn`", predicate=predicate)
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    predicate = "005[8:17] == '173911.0'"
    df = read_marc21(path, "001 AS `cn`", predicate=predicate)
    assert df.is_empty()

    with pytest.raises(ValueError, match=r"position \d:\d"):
        read_marc21(path, "001", predicate="005[:] == '20250720173911.0'")
