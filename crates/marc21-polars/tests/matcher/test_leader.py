"""Leader Matcher Test Suite."""

from pathlib import Path

import polars as pl
from polars.testing import assert_frame_equal

from polars_marc21 import read_marc21

__all__ = []


def test_leader_matcher(data_dir: Path) -> None:
    """Ensures that the leader matcher works as expected."""
    path = data_dir.joinpath("minna.mrc")

    # BASE ADDRESS (POSITION 12-16)
    rhs = pl.DataFrame({"column_1": ["040992918"]})
    lhs = read_marc21(path, "001", where="ldr.base_addr == 673")
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001", where="ldr.base_addr == 234")
    assert isinstance(lhs, pl.DataFrame)
    assert lhs.is_empty()

    # ENCODING (POSITION 09)
    rhs = pl.DataFrame({"column_1": ["040992918"]})
    lhs = read_marc21(path, "001", where="ldr.encoding == 'a'")
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001", where="ldr.encoding != 'a'")
    assert isinstance(lhs, pl.DataFrame)
    assert lhs.is_empty()

    # LENGTH (POSITION 00-04)
    rhs = pl.DataFrame({"column_1": ["040992918"]})
    lhs = read_marc21(path, "001", where="ldr.length >= 6091")
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001", where="ldr.length > 6091")
    assert isinstance(lhs, pl.DataFrame)
    assert lhs.is_empty()

    rhs = pl.DataFrame({"column_1": ["040992918"]})
    lhs = read_marc21(path, "001", where="ldr.length <= 6091")
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001", where="ldr.length < 6091")
    assert isinstance(lhs, pl.DataFrame)
    assert lhs.is_empty()

    # STATUS (POSITION 05)
    rhs = pl.DataFrame({"column_1": ["040992918"]})
    lhs = read_marc21(path, "001", where="ldr.status == 'n'")
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001", where="ldr.status != 'n'")
    assert isinstance(lhs, pl.DataFrame)
    assert lhs.is_empty()

    # TYPE (POSITION 06)
    rhs = pl.DataFrame({"column_1": ["040992918"]})
    lhs = read_marc21(path, "001", where="ldr.type == 'z'")
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001", where="ldr.type != 'z'")
    assert isinstance(lhs, pl.DataFrame)
    assert lhs.is_empty()
