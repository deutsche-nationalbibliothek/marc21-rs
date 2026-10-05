"""Leader Matcher Tests."""

from typing import TYPE_CHECKING

import polars as pl
from polars.testing import assert_frame_equal

from marc21_learn.io import read_marc21

if TYPE_CHECKING:
    from pathlib import Path


def test_length(data_dir: Path) -> None:
    """Ensure leader matcher (length field)."""
    path = data_dir.joinpath("ada.mrc")
    predicate = "ldr.length == 3612"
    query = "001 AS `cn`"

    lhs = read_marc21(path, query, predicate=predicate)
    rhs = pl.DataFrame({"cn": ["119232022"]})

    assert_frame_equal(lhs, rhs)


def test_status(data_dir: Path) -> None:
    """Ensure leader matcher (status field)."""
    path = data_dir.joinpath("ada.mrc")
    predicate = "ldr.status == 'n'"
    query = "001 AS `cn`"

    lhs = read_marc21(path, query, predicate=predicate)
    rhs = pl.DataFrame({"cn": ["119232022"]})

    assert_frame_equal(lhs, rhs)


def test_type(data_dir: Path) -> None:
    """Ensure leader matcher (type field)."""
    path = data_dir.joinpath("ada.mrc")
    predicate = "ldr.type == 'z'"
    query = "001 AS `cn`"

    lhs = read_marc21(path, query, predicate=predicate)
    rhs = pl.DataFrame({"cn": ["119232022"]})

    assert_frame_equal(lhs, rhs)


def test_encoding(data_dir: Path) -> None:
    """Ensure leader matcher (encoding field)."""
    path = data_dir.joinpath("ada.mrc")
    predicate = "ldr.encoding == 'a'"
    query = "001 AS `cn`"

    lhs = read_marc21(path, query, predicate=predicate)
    rhs = pl.DataFrame({"cn": ["119232022"]})

    assert_frame_equal(lhs, rhs)


def test_base_addr(data_dir: Path) -> None:
    """Ensure leader matcher (encoding field)."""
    path = data_dir.joinpath("ada.mrc")
    predicate = "ldr.base_addr == 589"
    query = "001 AS `cn`"

    lhs = read_marc21(path, query, predicate=predicate)
    rhs = pl.DataFrame({"cn": ["119232022"]})

    assert_frame_equal(lhs, rhs)


def test_eq_operator(data_dir: Path) -> None:
    """Test that the equality operator `==` works correctly."""
    path = data_dir.joinpath("ada.mrc")
    query = "001 AS `cn`"

    lhs = read_marc21(path, query, predicate="ldr.encoding == 'a'")
    assert_frame_equal(lhs, pl.DataFrame({"cn": ["119232022"]}))

    lhs = read_marc21(path, query, predicate="ldr.encoding == 'b'")
    assert lhs.is_empty()

    lhs = read_marc21(path, query, predicate="ldr.base_addr == 589")
    assert_frame_equal(lhs, pl.DataFrame({"cn": ["119232022"]}))

    lhs = read_marc21(path, query, predicate="ldr.base_addr == 123")
    assert lhs.is_empty()


def test_ne_operator(data_dir: Path) -> None:
    """Test that the inequality operator `!=` works correctly."""
    path = data_dir.joinpath("ada.mrc")
    query = "001 AS `cn`"

    lhs = read_marc21(path, query, predicate="ldr.encoding != 'b'")
    assert_frame_equal(lhs, pl.DataFrame({"cn": ["119232022"]}))

    lhs = read_marc21(path, query, predicate="ldr.encoding != 'a'")
    assert lhs.is_empty()

    lhs = read_marc21(path, query, predicate="ldr.base_addr != 123")
    assert_frame_equal(lhs, pl.DataFrame({"cn": ["119232022"]}))

    lhs = read_marc21(path, query, predicate="ldr.base_addr != 589")
    assert lhs.is_empty()


def test_lt_operator(data_dir: Path) -> None:
    """Test that the less than operator `<` works correctly."""
    path = data_dir.joinpath("ada.mrc")
    query = "001 AS `cn`"

    lhs = read_marc21(path, query, predicate="ldr.encoding < 'b'")
    assert_frame_equal(lhs, pl.DataFrame({"cn": ["119232022"]}))

    lhs = read_marc21(path, query, predicate="ldr.encoding < 'a'")
    assert lhs.is_empty()

    lhs = read_marc21(path, query, predicate="ldr.base_addr < 590")
    assert_frame_equal(lhs, pl.DataFrame({"cn": ["119232022"]}))

    lhs = read_marc21(path, query, predicate="ldr.base_addr < 589")
    assert lhs.is_empty()


def test_le_operator(data_dir: Path) -> None:
    """Test that the less than or equal operator `<=` works correctly."""
    path = data_dir.joinpath("ada.mrc")
    query = "001 AS `cn`"

    lhs = read_marc21(path, query, predicate="ldr.status <= 'n'")
    assert_frame_equal(lhs, pl.DataFrame({"cn": ["119232022"]}))

    lhs = read_marc21(path, query, predicate="ldr.status <= 'o'")
    assert_frame_equal(lhs, pl.DataFrame({"cn": ["119232022"]}))

    lhs = read_marc21(path, query, predicate="ldr.status <= 'm'")
    assert lhs.is_empty()

    lhs = read_marc21(path, query, predicate="ldr.base_addr <= 590")
    assert_frame_equal(lhs, pl.DataFrame({"cn": ["119232022"]}))

    lhs = read_marc21(path, query, predicate="ldr.base_addr <= 589")
    assert_frame_equal(lhs, pl.DataFrame({"cn": ["119232022"]}))

    lhs = read_marc21(path, query, predicate="ldr.base_addr <= 588")
    assert lhs.is_empty()


def test_gt_operator(data_dir: Path) -> None:
    """Test that the greater than operator `>` works correctly."""
    path = data_dir.joinpath("ada.mrc")
    query = "001 AS `cn`"

    lhs = read_marc21(path, query, predicate="ldr.status > 'm'")
    assert_frame_equal(lhs, pl.DataFrame({"cn": ["119232022"]}))

    lhs = read_marc21(path, query, predicate="ldr.status > 'n'")
    assert lhs.is_empty()

    lhs = read_marc21(path, query, predicate="ldr.length > 3611")
    assert_frame_equal(lhs, pl.DataFrame({"cn": ["119232022"]}))

    lhs = read_marc21(path, query, predicate="ldr.length > 3612")
    assert lhs.is_empty()


def test_ge_operator(data_dir: Path) -> None:
    """Test that the greater than or equal operator `>=` works correctly."""
    path = data_dir.joinpath("ada.mrc")
    query = "001 AS `cn`"

    lhs = read_marc21(path, query, predicate="ldr.status >= 'm'")
    assert_frame_equal(lhs, pl.DataFrame({"cn": ["119232022"]}))

    lhs = read_marc21(path, query, predicate="ldr.status >= 'n'")
    assert_frame_equal(lhs, pl.DataFrame({"cn": ["119232022"]}))

    lhs = read_marc21(path, query, predicate="ldr.status >= 'o'")
    assert lhs.is_empty()

    lhs = read_marc21(path, query, predicate="ldr.base_addr >= 588")
    assert_frame_equal(lhs, pl.DataFrame({"cn": ["119232022"]}))

    lhs = read_marc21(path, query, predicate="ldr.base_addr >= 589")
    assert_frame_equal(lhs, pl.DataFrame({"cn": ["119232022"]}))

    lhs = read_marc21(path, query, predicate="ldr.base_addr >= 590")
    assert lhs.is_empty()
