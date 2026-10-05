"""Tag Matcher Tests."""

from typing import TYPE_CHECKING

import polars as pl
import pytest
from polars.testing import assert_frame_equal

from marc21_learn.io import read_marc21

if TYPE_CHECKING:
    from pathlib import Path


def test_explicit(data_dir: Path) -> None:
    """Ensures that the indicator matcher works in the explicit form."""
    path = data_dir.joinpath("ada.mrc")

    lhs = read_marc21(path, "001 AS `cn`", predicate="400/1#?")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    df = read_marc21(path, "001 AS `cn`", predicate="400/2#?")
    assert df.is_empty()


def test_wildcard(data_dir: Path) -> None:
    """Ensures that the wildcard matcher works correctly."""
    path = data_dir.joinpath("ada.mrc")

    lhs = read_marc21(path, "001 AS `cn`", predicate="400/..?")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", predicate="400/1.?")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", predicate="400/.#?")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", predicate="400/*?")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    df = read_marc21(path, "001 AS `cn`", predicate="300/*?")
    assert df.is_empty()


def test_classes(data_dir: Path) -> None:
    """Ensures that the class matcher works correctly."""
    path = data_dir.joinpath("ada.mrc")

    lhs = read_marc21(path, "001 AS `cn`", predicate="400/[1-2]#?")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", predicate="400/[1-2][3-5#]?")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    df = read_marc21(path, "001 AS `cn`", predicate="400/[3-9]#?")
    assert df.is_empty()


def test_negated_class(data_dir: Path) -> None:
    """Ensures that the class negation works correctly."""
    path = data_dir.joinpath("ada.mrc")

    lhs = read_marc21(path, "001 AS `cn`", predicate="400/[^7-9]#?")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", predicate="400/[1-2][^23456789]?")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    df = read_marc21(path, "001 AS `cn`", predicate="400/[^12345]#?")
    assert df.is_empty()


def test_class_ranges(data_dir: Path) -> None:
    """Ensures that the class ranges works correctly."""
    path = data_dir.joinpath("ada.mrc")

    lhs = read_marc21(path, "001 AS `cn`", predicate="400/[1-5]#?")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", predicate="400/1[^2-8]?")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    df = read_marc21(path, "001 AS `cn`", predicate="400/[2-5]#?")
    assert df.is_empty()

    with pytest.raises(ValueError, match=r"position \d:\d"):
        read_marc21(path, "001", predicate="400/[1-1]#?")

    with pytest.raises(ValueError, match=r"position \d:\d"):
        read_marc21(path, "001", predicate="400/[3-1]#?")
