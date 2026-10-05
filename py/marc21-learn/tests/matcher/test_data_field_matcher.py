"""Data Field Matcher Tests."""

from typing import TYPE_CHECKING

import polars as pl
import pytest
from polars.testing import assert_frame_equal

from marc21_learn.io import read_marc21

if TYPE_CHECKING:
    from pathlib import Path


def test_exists_matcher(data_dir: Path) -> None:
    """Ensures that the exists matcher works correctly."""
    path = data_dir.joinpath("ada.mrc")

    # long from
    lhs = read_marc21(path, "001 AS `cn`", predicate="375{ 2? }")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", predicate="375{ !x? }")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", predicate="375{ [a2]? }")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    df = read_marc21(path, "001 AS `cn`", predicate="375{ x? }")
    assert df.is_empty()

    # short from
    lhs = read_marc21(path, "001 AS `cn`", predicate="375.2?")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "001 AS `cn`", predicate="!375.x?")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    df = read_marc21(path, "001 AS `cn`", predicate="375.x?")
    assert df.is_empty()

    with pytest.raises(ValueError, match=r"position \d:\d"):
        read_marc21(path, "001", predicate="375{ a ? }'")


def test_count_matcher(data_dir: Path) -> None:
    """Ensures that the count matcher works correctly."""
    path = data_dir.joinpath("ada.mrc")

    lhs = read_marc21(path, "001 AS `cn`", predicate="079{ #q == 3 }")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    df = read_marc21(path, "001 AS `cn`", predicate="079{ #q == 4 }")
    assert df.is_empty()

    lhs = read_marc21(path, "001 AS `cn`", predicate="079{ #q != 2 }")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    df = read_marc21(path, "001 AS `cn`", predicate="079{ #q != 3 }")
    assert df.is_empty()

    lhs = read_marc21(path, "001 AS `cn`", predicate="079{ #q >= 3 }")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    df = read_marc21(path, "001 AS `cn`", predicate="079{ #q >= 4 }")
    assert df.is_empty()

    lhs = read_marc21(path, "001 AS `cn`", predicate="079{ #q > 2 }")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    df = read_marc21(path, "001 AS `cn`", predicate="079{ #q > 3 }")
    assert df.is_empty()

    lhs = read_marc21(path, "001 AS `cn`", predicate="079{ #q <= 3 }")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    df = read_marc21(path, "001 AS `cn`", predicate="079{ #q <= 2 }")
    assert df.is_empty()

    lhs = read_marc21(path, "001 AS `cn`", predicate="079{ #q < 4 }")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    df = read_marc21(path, "001 AS `cn`", predicate="079{ #q < 2 }")
    assert df.is_empty()

    with pytest.raises(ValueError, match=r"position \d:\d"):
        read_marc21(path, "001", predicate="079{ # q == 3 }'")


def test_comparison_matcher(data_dir: Path) -> None:
    """Ensures that the comparison matcher works correctly."""
    path = data_dir.joinpath("ada.mrc")

    # long from
    lhs = read_marc21(path, "001 AS `cn`", predicate="079{ q == 'z' }")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    df = read_marc21(path, "001 AS `cn`", predicate="079{ q == 'x' }")
    assert df.is_empty()

    lhs = read_marc21(path, "001 AS `cn`", predicate="079{ q != 'z' }")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    df = read_marc21(path, "001 AS `cn`", predicate="079{ a != 'g' }")
    assert df.is_empty()

    lhs = read_marc21(path, "001 AS `cn`", predicate="079{ u >= 'w' }")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    df = read_marc21(path, "001 AS `cn`", predicate="079{ u >= 'x' }")
    assert df.is_empty()

    lhs = read_marc21(path, "001 AS `cn`", predicate="079{ u > 'v' }")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    df = read_marc21(path, "001 AS `cn`", predicate="079{ u > 'w' }")
    assert df.is_empty()

    lhs = read_marc21(path, "001 AS `cn`", predicate="079{ u <= 'w' }")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    df = read_marc21(path, "001 AS `cn`", predicate="079{ u <= 'j' }")
    assert df.is_empty()

    lhs = read_marc21(path, "001 AS `cn`", predicate="079{ u < 'w' }")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    df = read_marc21(path, "001 AS `cn`", predicate="079{ u < 'k' }")
    assert df.is_empty()

    with pytest.raises(ValueError, match=r"position \d:\d"):
        read_marc21(path, "001", predicate="079{ u<'w' }")

    # short form
    lhs = read_marc21(path, "001 AS `cn`", predicate="079.q == 'z'")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    df = read_marc21(path, "001 AS `cn`", predicate="079.q == 'x'")
    assert df.is_empty()

    lhs = read_marc21(path, "001 AS `cn`", predicate="079.q != 'z'")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    df = read_marc21(path, "001 AS `cn`", predicate="079.a != 'g'")
    assert df.is_empty()

    lhs = read_marc21(path, "001 AS `cn`", predicate="079.u >= 'w'")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    df = read_marc21(path, "001 AS `cn`", predicate="079.u >= 'x'")
    assert df.is_empty()

    lhs = read_marc21(path, "001 AS `cn`", predicate="079.u > 'v'")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    df = read_marc21(path, "001 AS `cn`", predicate="079.u > 'w'")
    assert df.is_empty()

    lhs = read_marc21(path, "001 AS `cn`", predicate="079.u <= 'w'")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    df = read_marc21(path, "001 AS `cn`", predicate="079.u <= 'j'")
    assert df.is_empty()

    lhs = read_marc21(path, "001 AS `cn`", predicate="079.u < 'w'")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    df = read_marc21(path, "001 AS `cn`", predicate="079.u < 'k'")
    assert df.is_empty()

    with pytest.raises(ValueError, match=r"position \d:\d"):
        read_marc21(path, "001", predicate="079.u<'w'")


def test_substr_matcher(data_dir: Path) -> None:
    """Ensures that the substring matcher works correctly."""
    path = data_dir.joinpath("ada.mrc")

    # long from
    lhs = read_marc21(path, "001 AS `cn`", predicate="400/1#{ a =? 'ove' }")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    df = read_marc21(path, "001 AS `cn`", predicate="400/1#{ a =? 'foo' }")
    assert df.is_empty()

    predicate = "400/1#{ a !? 'Lovelace' }"
    lhs = read_marc21(path, "001 AS `cn`", predicate=predicate)
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    predicate = "400/1#{ a !? 'Ada' }"
    df = read_marc21(path, "001 AS `cn`", predicate=predicate)
    assert df.is_empty()

    predicate = "400/1#{ a =? ['foo', 'ove'] }"
    lhs = read_marc21(path, "001 AS `cn`", predicate=predicate)
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    predicate = "400/1#{ a =? ['foo', 'bar'] }"
    df = read_marc21(path, "001 AS `cn`", predicate=predicate)
    assert df.is_empty()

    predicate = "400/1#{ ANY a =? 'Ada' }"
    lhs = read_marc21(path, "001 AS `cn`", predicate=predicate)
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    predicate = "400/1#{ ALL a =? 'Ada' }"
    lhs = read_marc21(path, "001 AS `cn`", predicate=predicate)
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    with pytest.raises(ValueError, match=r"position \d:\d"):
        read_marc21(path, "001", predicate="400/1#{ a=?'Ada' }")

    # short from
    lhs = read_marc21(path, "001 AS `cn`", predicate="400/1#.a =? 'ove'")
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    df = read_marc21(path, "001 AS `cn`", predicate="400/1#.a =? 'foo'")
    assert df.is_empty()

    predicate = "400/1#.a !? 'Lovelace'"
    lhs = read_marc21(path, "001 AS `cn`", predicate=predicate)
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    predicate = "400/1#.a !? 'Ada'"
    df = read_marc21(path, "001 AS `cn`", predicate=predicate)
    assert df.is_empty()

    predicate = "400/1#.a =? ['foo', 'ove']"
    lhs = read_marc21(path, "001 AS `cn`", predicate=predicate)
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    predicate = "400/1#.a =? ['foo', 'bar']"
    df = read_marc21(path, "001 AS `cn`", predicate=predicate)
    assert df.is_empty()

    with pytest.raises(ValueError, match=r"position \d:\d"):
        read_marc21(path, "001", predicate="400/1#.a=?'Ada'")


def test_member_matcher(data_dir: Path) -> None:
    """Ensures that the member matcher works correctly."""
    path = data_dir.joinpath("ada.mrc")

    # long from
    predicate = "042{ a in ['gnd1', 'gnd2'] }"
    lhs = read_marc21(path, "001 AS `cn`", predicate=predicate)
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    predicate = "042{ a in ['gnd2', 'gnd3'] }"
    df = read_marc21(path, "001 AS `cn`", predicate=predicate)
    assert df.is_empty()

    predicate = "042{ a not in ['gnd2', 'gnd3'] }"
    lhs = read_marc21(path, "001 AS `cn`", predicate=predicate)
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    predicate = "042{ a not in ['gnd1', 'gnd3'] }"
    df = read_marc21(path, "001 AS `cn`", predicate=predicate)
    assert df.is_empty()

    predicate = "042{ a in ['gnd1', 'gnd2', ] }"
    lhs = read_marc21(path, "001 AS `cn`", predicate=predicate)
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    # short form
    predicate = "042.a in ['gnd1', 'gnd2']"
    lhs = read_marc21(path, "001 AS `cn`", predicate=predicate)
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    predicate = "042.a in ['gnd2', 'gnd3']"
    df = read_marc21(path, "001 AS `cn`", predicate=predicate)
    assert df.is_empty()

    predicate = "042.a not in ['gnd2', 'gnd3']"
    lhs = read_marc21(path, "001 AS `cn`", predicate=predicate)
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    predicate = "042.a not in ['gnd1', 'gnd3']"
    df = read_marc21(path, "001 AS `cn`", predicate=predicate)
    assert df.is_empty()

    predicate = "042.a in ['gnd1', 'gnd2', ]"
    lhs = read_marc21(path, "001 AS `cn`", predicate=predicate)
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)


def test_prefix_matcher(data_dir: Path) -> None:
    """Ensures that the prefix matcher works correctly."""
    path = data_dir.joinpath("ada.mrc")

    # long from
    predicate = "400/1#{ a =^ 'Love' }"
    lhs = read_marc21(path, "001 AS `cn`", predicate=predicate)
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    predicate = "400/1#{ a =^ 'Hate' }"
    df = read_marc21(path, "001 AS `cn`", predicate=predicate)
    assert df.is_empty()

    # short from
    predicate = "400/1#.a =^ 'Love'"
    lhs = read_marc21(path, "001 AS `cn`", predicate=predicate)
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    predicate = "400/1#.a =^ 'Hate'"
    df = read_marc21(path, "001 AS `cn`", predicate=predicate)
    assert df.is_empty()


def test_suffix_matcher(data_dir: Path) -> None:
    """Ensures that the suffix matcher works correctly."""
    path = data_dir.joinpath("ada.mrc")

    # long from
    predicate = "100/1#{ a =$ 'Ada' }"
    lhs = read_marc21(path, "001 AS `cn`", predicate=predicate)
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    predicate = "100/1#{ a =$ 'Curie' }"
    df = read_marc21(path, "001 AS `cn`", predicate=predicate)
    assert df.is_empty()

    # short from
    predicate = "100/1#.a =$ 'Ada'"
    lhs = read_marc21(path, "001 AS `cn`", predicate=predicate)
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    predicate = "100/1#.a =^ 'Curie'"
    df = read_marc21(path, "001 AS `cn`", predicate=predicate)
    assert df.is_empty()


def test_strsim_matcher(data_dir: Path) -> None:
    """Ensures that the string similarity matcher works correctly."""
    path = data_dir.joinpath("ada.mrc")

    # long from
    predicate = "100/1#{ a =* 'Lovelace, Ada' }"
    lhs = read_marc21(path, "001 AS `cn`", predicate=predicate)
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    predicate = "100/1#{ a =* 'Lavelace, Ada' }"
    lhs = read_marc21(path, "001 AS `cn`", predicate=predicate)
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    predicate = "100/1#{ a =* 'Hateless, Ada' }"
    df = read_marc21(path, "001 AS `cn`", predicate=predicate)
    assert df.is_empty()

    predicate = "100/1#{ ALL a =* 'Lavelace, Ada' }"
    lhs = read_marc21(path, "001 AS `cn`", predicate=predicate)
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    predicate = "100/1#{ ANY a =* 'Lavelace, Ada' }"
    lhs = read_marc21(path, "001 AS `cn`", predicate=predicate)
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    # short from
    predicate = "100/1#.a =* 'Lovelace, Ada'"
    lhs = read_marc21(path, "001 AS `cn`", predicate=predicate)
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    predicate = "100/1#.a =* 'Lavelace, Ada'"
    lhs = read_marc21(path, "001 AS `cn`", predicate=predicate)
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    predicate = "100/1#.a =* 'Hateless, Ada'"
    df = read_marc21(path, "001 AS `cn`", predicate=predicate)
    assert df.is_empty()


def test_regex_matcher(data_dir: Path) -> None:
    """Ensures that the regex matcher works correctly."""
    path = data_dir.joinpath("ada.mrc")

    # long from
    predicate = "042{ a =~ '^gnd[1-7z]$' }"
    lhs = read_marc21(path, "001 AS `cn`", predicate=predicate)
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    predicate = "100/1#{ a =* 'Hateless, Ada' }"
    df = read_marc21(path, "001 AS `cn`", predicate=predicate)
    assert df.is_empty()

    predicate = "042{ ALL a =~ '^gnd[1-7z]$' }"
    lhs = read_marc21(path, "001 AS `cn`", predicate=predicate)
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    predicate = "042{ ANY a =~ '^gnd[1-7z]$' }"
    lhs = read_marc21(path, "001 AS `cn`", predicate=predicate)
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    predicate = r"100/1#{ a =~ 'A[bd]{2,}a' }"
    df = read_marc21(path, "001", predicate=predicate)
    assert df.is_empty()

    # short from
    predicate = "042.a =~ '^gnd[1-7z]$'"
    lhs = read_marc21(path, "001 AS `cn`", predicate=predicate)
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    predicate = "100/1#.a =* 'Hateless, Ada'"
    df = read_marc21(path, "001 AS `cn`", predicate=predicate)
    assert df.is_empty()

    predicate = r"100/1#.a =~ 'A[bd]{2,}a'"
    df = read_marc21(path, "001", predicate=predicate)
    assert df.is_empty()


def test_disjunction(data_dir: Path) -> None:
    """Ensures that the Boolean connective `||` works correctly."""
    path = data_dir.joinpath("ada.mrc")

    # FALSE || FALSE = FALSE
    predicate = "042{ a == 'gnd2' || a == 'gnd3' }"
    df = read_marc21(path, "001", predicate=predicate)
    assert df.is_empty()

    # FALSE || TRUE = TRUE
    predicate = "042{ a == 'gnd2' || a == 'gnd1' }"
    lhs = read_marc21(path, "001 AS `cn`", predicate=predicate)
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    # TRUE || FALSE  = TRUE
    predicate = "042{ a == 'gnd1' || a == 'gnd2' }"
    lhs = read_marc21(path, "001 AS `cn`", predicate=predicate)
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    # TRUE || TRUE = TRUE
    predicate = "079{ a == 'g' || q == 'f' }"
    lhs = read_marc21(path, "001 AS `cn`", predicate=predicate)
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    # multiple sub-expressions
    predicate = "042{ a == 'gnd1' || a == 'gnd2' || a == 'gnd3' }"
    lhs = read_marc21(path, "001 AS `cn`", predicate=predicate)
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    # disjunction w/ groups (rhs)
    predicate = "042{ a == 'gnd1' || (a == 'gnd2' || a == 'gnd3') }"
    lhs = read_marc21(path, "001 AS `cn`", predicate=predicate)
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    # disjunction w/ groups (lhs)
    predicate = "042{ (a == 'gnd1' || a == 'gnd2') || a == 'gnd3' }"
    lhs = read_marc21(path, "001 AS `cn`", predicate=predicate)
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    # disjunction inside group stmt
    predicate = "042{ (a == 'gnd1' || a == 'gnd2') }"
    lhs = read_marc21(path, "001 AS `cn`", predicate=predicate)
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)


def test_conjunction(data_dir: Path) -> None:
    """Ensures that the Boolean connective `&&` works correctly."""
    path = data_dir.joinpath("ada.mrc")

    # FALSE && FALSE = FALSE
    predicate = "042{ a == 'gnd2' && a == 'gnd3' }"
    df = read_marc21(path, "001", predicate=predicate)
    assert df.is_empty()

    # TRUE && FALSE = FALSE
    predicate = "042{ a == 'gnd1' && a == 'gnd2' }"
    df = read_marc21(path, "001", predicate=predicate)
    assert df.is_empty()

    # FALSE && TRUE = FALSE
    predicate = "042{ a == 'gnd2' && a == 'gnd1' }"
    df = read_marc21(path, "001", predicate=predicate)
    assert df.is_empty()

    # TRUE && TRUE = TRUE
    predicate = "075{ b == 'p' && 2 == 'gndgen' }"
    lhs = read_marc21(path, "001 AS `cn`", predicate=predicate)
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    # multiple sub-expressions
    predicate = "079{ a == 'g' && q == 'f' && q == 's' }"
    lhs = read_marc21(path, "001 AS `cn`", predicate=predicate)
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    # conjunction w/ groups (lhs)
    predicate = "079{ (a == 'g' || q == 'x') && q == 's' }"
    lhs = read_marc21(path, "001 AS `cn`", predicate=predicate)
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    # conjunction w/ groups (rhs)
    predicate = "079{ a == 'g' && (q == 'x' || q == 's') }"
    lhs = read_marc21(path, "001 AS `cn`", predicate=predicate)
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    # conjunction inside group stmt
    predicate = "075{ (b == 'p' && 2 == 'gndgen') }"
    lhs = read_marc21(path, "001 AS `cn`", predicate=predicate)
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)


def test_operator_precedence(data_dir: Path) -> None:
    """Ensures that the Boolean connective `&&` works correctly."""
    path = data_dir.joinpath("ada.mrc")

    # TRUE || FALSE && FALSE
    predicate = "079{ a == 'g' || q == 'x' && q == 'y' }"
    lhs = read_marc21(path, "001 AS `cn`", predicate=predicate)
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    # (TRUE || FALSE) && FALSE
    predicate = "079{ (a == 'g' || q == 'x') && q == 'y' }"
    df = read_marc21(path, "001 AS `cn`", predicate=predicate)
    assert df.is_empty()

    # TRUE || TRUE && FALSE
    predicate = "079{ a == 'g' || q == 's' && q == 'x' }"
    lhs = read_marc21(path, "001 AS `cn`", predicate=predicate)
    rhs = pl.DataFrame({"cn": ["119232022"]})
    assert_frame_equal(lhs, rhs)

    # (TRUE || TRUE) && FALSE
    predicate = "079{ (a == 'g' || q == 's') && q == 'x' }"
    df = read_marc21(path, "001 AS `cn`", predicate=predicate)
    assert df.is_empty()
