"""Control Field Projection Tests."""

from typing import TYPE_CHECKING

import polars as pl
from polars.testing import assert_frame_equal

from marc21_learn.io import read_marc21

if TYPE_CHECKING:
    from pathlib import Path


def test_explicitly_tag_ref(data_dir: Path) -> None:
    """Ensures that explicitly specifying a tag works."""
    path = data_dir.joinpath("ada.mrc")

    lhs = read_marc21(path, "001")
    rhs = pl.DataFrame({"column_1": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "005")
    rhs = pl.DataFrame({"column_1": ["20250720173911.0"]})
    assert_frame_equal(lhs, rhs)


def test_tag_matcher(data_dir: Path) -> None:
    """Ensure that processing multiple fields works properly."""
    path = data_dir.joinpath("ada.mrc")

    lhs = read_marc21(path, "00[13]")
    rhs = pl.DataFrame({"column_1": ["119232022", "DE-101"]})
    assert_frame_equal(lhs, rhs)


def test_ranges(data_dir: Path) -> None:
    """Ensure that processing of ranges works properly."""
    path = data_dir.joinpath("ada.mrc")

    lhs = read_marc21(path, "005[0:4]")
    rhs = pl.DataFrame({"column_1": ["2025"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "005[4:8]")
    rhs = pl.DataFrame({"column_1": ["0720"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "005[8:]")
    rhs = pl.DataFrame({"column_1": ["173911.0"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "005[:8]")
    rhs = pl.DataFrame({"column_1": ["20250720"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001, 005[3:42]")
    rhs = pl.DataFrame({"column_1": ["119232022"], "column_2": [""]})
    assert_frame_equal(lhs, rhs)

    assert read_marc21(path, "005[3:42]").is_empty()
    assert read_marc21(path, "005[2:1]").is_empty()
    assert read_marc21(path, "005[2:2]").is_empty()
