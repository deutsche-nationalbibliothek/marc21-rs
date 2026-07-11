"""Predicate Test Suite."""

from pathlib import Path

import polars as pl
from polars.testing import assert_frame_equal

from polars_marc21 import read_marc21

__all__ = []


def test_subfield_group_matcher(data_dir: Path) -> None:
    """Ensures that the subfield group matcher works as expected."""
    path = data_dir.joinpath("ada.mrc")

    # EXISTS
    rhs = pl.DataFrame({"column_1": ["119232022"]})
    lhs = read_marc21(path, "001", where="079{ (a?) }")
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001", where="079{ (b?) }")
    assert isinstance(lhs, pl.DataFrame)
    assert lhs.is_empty()

    # COMPARISON
    rhs = pl.DataFrame({"column_1": ["119232022"]})
    lhs = read_marc21(path, "001", where="079{ (a == 'g') }")
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001", where="079{ (b == 'g') }")
    assert isinstance(lhs, pl.DataFrame)
    assert lhs.is_empty()

    # STARTS WITH
    rhs = pl.DataFrame({"column_1": ["119232022"]})
    lhs = read_marc21(path, "001", where="079{ (a =^ 'g') }")
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001", where="079{ (a =^ 'x') }")
    assert isinstance(lhs, pl.DataFrame)
    assert lhs.is_empty()

    # ENDS WITH
    rhs = pl.DataFrame({"column_1": ["119232022"]})
    lhs = read_marc21(path, "001", where="079{ (a =$ 'g') }")
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001", where="079{ (a =$ 'x') }")
    assert isinstance(lhs, pl.DataFrame)
    assert lhs.is_empty()

    # IN
    rhs = pl.DataFrame({"column_1": ["119232022"]})
    lhs = read_marc21(path, "001", where="079{ (a in ['g', 'k']) }")
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001", where="079{ (a in ['x', 'y']) }")
    assert isinstance(lhs, pl.DataFrame)
    assert lhs.is_empty()

    # REGEX
    rhs = pl.DataFrame({"column_1": ["119232022"]})
    lhs = read_marc21(path, "001", where="079{ (a =~ 'g') }")
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001", where="079{ (a =~ '(?i)^[xyz]') }")
    assert isinstance(lhs, pl.DataFrame)
    assert lhs.is_empty()

    # STRSIM
    rhs = pl.DataFrame({"column_1": ["119232022"]})
    lhs = read_marc21(path, "001", where="079{ (a =* 'g') }")
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001", where="079{ (a =* 'x') }")
    assert isinstance(lhs, pl.DataFrame)
    assert lhs.is_empty()

    # COUNT
    rhs = pl.DataFrame({"column_1": ["119232022"]})
    lhs = read_marc21(path, "001", where="079{ (#q == 3) }")
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001", where="079{ (#a > 1) }")
    assert isinstance(lhs, pl.DataFrame)
    assert lhs.is_empty()

    # GROUP
    rhs = pl.DataFrame({"column_1": ["119232022"]})
    lhs = read_marc21(path, "001", where="079{ ((a?)) }")
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001", where="079{ ((b?)) }")
    assert isinstance(lhs, pl.DataFrame)
    assert lhs.is_empty()

    # NOT
    rhs = pl.DataFrame({"column_1": ["119232022"]})
    lhs = read_marc21(path, "001", where="079{ (!(b?)) }")
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001", where="079{ (!(a?)) }")
    assert isinstance(lhs, pl.DataFrame)
    assert lhs.is_empty()

    # AND
    rhs = pl.DataFrame({"column_1": ["119232022"]})
    lhs = read_marc21(path, "001", where="079{ (a? && q?) }")
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001", where="079{ (q? && b?) }")
    assert isinstance(lhs, pl.DataFrame)
    assert lhs.is_empty()

    # OR
    rhs = pl.DataFrame({"column_1": ["119232022"]})
    lhs = read_marc21(path, "001", where="079{ (a? || q?) }")
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001", where="079{ (b? || x?) }")
    assert isinstance(lhs, pl.DataFrame)
    assert lhs.is_empty()

def test_subfield_count_matcher(data_dir: Path) -> None:
    """Tests the subfield count matcher for correctness."""
    path = data_dir.joinpath("ada.mrc")

    # equal
    lhs = read_marc21(path, "001", where="079{ #q == 3 }")
    rhs = pl.DataFrame({"column_1": ["119232022"]})
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001", where="079{ #a == 3 }")
    assert isinstance(lhs, pl.DataFrame)
    assert lhs.is_empty()

    # not equal
    lhs = read_marc21(path, "001", where="079{ #a != 3 }")
    rhs = pl.DataFrame({"column_1": ["119232022"]})
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001", where="079{ #q != 3 }")
    assert isinstance(lhs, pl.DataFrame)
    assert lhs.is_empty()

    # greater than or equal
    lhs = read_marc21(path, "001", where="079{ #q >= 3 }")
    rhs = pl.DataFrame({"column_1": ["119232022"]})
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001", where="079{ #a >= 3 }")
    assert isinstance(lhs, pl.DataFrame)
    assert lhs.is_empty()

    # greater than
    lhs = read_marc21(path, "001", where="079{ #q > 2 }")
    rhs = pl.DataFrame({"column_1": ["119232022"]})
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001", where="079{ #q > 3 }")
    assert isinstance(lhs, pl.DataFrame)
    assert lhs.is_empty()

    # less than or equal
    lhs = read_marc21(path, "001", where="079{ #q <= 3 }")
    rhs = pl.DataFrame({"column_1": ["119232022"]})
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001", where="079{ #q <= 2 }")
    assert isinstance(lhs, pl.DataFrame)
    assert lhs.is_empty()

    # less than
    lhs = read_marc21(path, "001", where="079{ #q < 4 }")
    rhs = pl.DataFrame({"column_1": ["119232022"]})
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001", where="079{ #q < 3 }")
    assert isinstance(lhs, pl.DataFrame)
    assert lhs.is_empty()

    # code groups
    lhs = read_marc21(path, "001", where="079{ #[qu] == 6 }")
    rhs = pl.DataFrame({"column_1": ["119232022"]})
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001", where="079{ #[aq] == 6 }")
    assert isinstance(lhs, pl.DataFrame)
    assert lhs.is_empty()

    # non-existing subfield code
    lhs = read_marc21(path, "001", where="079{ #x == 0 }")
    rhs = pl.DataFrame({"column_1": ["119232022"]})
    assert isinstance(lhs, pl.DataFrame)
    assert_frame_equal(lhs, rhs)
