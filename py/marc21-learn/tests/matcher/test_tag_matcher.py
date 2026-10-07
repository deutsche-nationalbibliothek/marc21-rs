"""Tag Matcher Tests."""

from typing import TYPE_CHECKING

import polars as pl
import pytest
from polars.testing import assert_frame_equal

from marc21_learn.io import read_marc21

if TYPE_CHECKING:
    from pathlib import Path


def test_explicit(data_dir: Path) -> None:
    """Ensures that the tag matcher works in the explicit form."""
    path = data_dir.joinpath("ada.mrc")

    lhs = read_marc21(path, "001 AS `cn`", predicate="001?")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    df = read_marc21(path, "001 AS `cn`", predicate="004?")
    assert df.is_empty()

    lhs = read_marc21(path, "001 AS `cn`", predicate="075?")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    df = read_marc21(path, "001", predicate="099?")
    assert df.is_empty()


def test_wildcard(data_dir: Path) -> None:
    """Ensures that individual positions can be replaced with `.`."""
    path = data_dir.joinpath("ada.mrc")

    lhs = read_marc21(path, "001 AS `cn`", predicate=".01?")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", predicate="0.1?")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", predicate="00.?")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", predicate="..1?")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", predicate="...?")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", predicate="3.5?")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    df = read_marc21(path, "001", predicate="8..?")
    assert df.is_empty()


def test_classes(data_dir: Path) -> None:
    """Ensures that classes `[]` work correctly."""
    path = data_dir.joinpath("ada.mrc")

    lhs = read_marc21(path, "001 AS `cn`", predicate="00[13]?")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    df = read_marc21(path, "001", predicate="00[46]?")
    assert df.is_empty()

    lhs = read_marc21(path, "001 AS `cn`", predicate="07[45]?")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    df = read_marc21(path, "001", predicate="07[68]?")
    assert df.is_empty()

    lhs = read_marc21(path, "001 AS `cn`", predicate="[10]01?")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", predicate="0[67]5?")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", predicate="06[45]?")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", predicate="[34][10]0/*?")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", predicate="[34]0[10]/*?")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", predicate="[34][67][45]/*?")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)


def test_class_ranges(data_dir: Path) -> None:
    """Ensures that class ranges `[X-Y]` work correctly."""
    path = data_dir.joinpath("ada.mrc")

    lhs = read_marc21(path, "001 AS `cn`", predicate="00[4-6]?")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    df = read_marc21(path, "001", predicate="00[6-7]?")
    assert df.is_empty()

    lhs = read_marc21(path, "001 AS `cn`", predicate="07[2-5]?")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    df = read_marc21(path, "001", predicate="07[2-4]?")
    assert df.is_empty()

    lhs = read_marc21(path, "001 AS `cn`", predicate="[0-9][0-9][0-9]?")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    with pytest.raises(ValueError, match=r"position \d:\d"):
        read_marc21(path, "001", predicate="07[2-2]?")

    with pytest.raises(ValueError, match=r"position \d:\d"):
        read_marc21(path, "001", predicate="07[3-1]?")


def test_negated_class(data_dir: Path) -> None:
    """Ensures that class negation `[^...]` work correctly."""
    path = data_dir.joinpath("ada.mrc")

    lhs = read_marc21(path, "001 AS `cn`", predicate="00[^1-7]?")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    df = read_marc21(path, "001", predicate="00[^1-8]?")
    assert df.is_empty()

    lhs = read_marc21(path, "001 AS `cn`", predicate="06[^1-46-9]?")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    df = read_marc21(path, "001", predicate="06[^1-5]?")
    assert df.is_empty()
