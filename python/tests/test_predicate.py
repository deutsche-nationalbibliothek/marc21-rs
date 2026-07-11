"""Query Test Suite."""

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
