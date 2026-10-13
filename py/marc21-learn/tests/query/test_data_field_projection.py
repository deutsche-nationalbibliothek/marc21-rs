"""Control Field Projection Tests."""

from typing import TYPE_CHECKING

import polars as pl
from polars.testing import assert_frame_equal

from marc21_learn.io import read_marc21

if TYPE_CHECKING:
    from pathlib import Path


def test_short_form(data_dir: Path) -> None:
    """Ensures that explicitly specifying a tag works."""
    path = data_dir.joinpath("ada.mrc")

    # field NR, subfield NR
    lhs = read_marc21(path, "042.a")
    rhs = pl.DataFrame({"column_1": ["gnd1"]})
    assert_frame_equal(lhs, rhs)

    # field R, subfield NR
    lhs = read_marc21(path, "065.a")
    rhs = pl.DataFrame({"column_1": ["28p", "9.5p"]})
    assert_frame_equal(lhs, rhs)

    # field NR, subfield R
    lhs = read_marc21(path, "079.q")
    rhs = pl.DataFrame({"column_1": ["f", "s", "z"]})
    assert_frame_equal(lhs, rhs)

    # field R, subfield R
    lhs = read_marc21(path, "075.[b2]")
    rhs = pl.DataFrame({"column_1": ["p", "gndgen", "piz", "gndspec"]})
    assert_frame_equal(lhs, rhs)


def test_long_form(data_dir: Path) -> None:
    """Ensures that explicitly specifying a tag works."""
    path = data_dir.joinpath("ada.mrc")

    # field NR, subfield NR
    lhs = read_marc21(path, "042{ a }")
    rhs = pl.DataFrame({"column_1": ["gnd1"]})
    assert_frame_equal(lhs, rhs)

    # field R, subfield NR
    lhs = read_marc21(path, "065{ a }")
    rhs = pl.DataFrame({"column_1": ["28p", "9.5p"]})
    assert_frame_equal(lhs, rhs)

    # field NR, subfield R
    lhs = read_marc21(path, "079{ q }")
    rhs = pl.DataFrame({"column_1": ["f", "s", "z"]})
    assert_frame_equal(lhs, rhs)

    # field R, subfield R
    lhs = read_marc21(path, "075{ [b2] }")
    rhs = pl.DataFrame({"column_1": ["p", "gndgen", "piz", "gndspec"]})
    assert_frame_equal(lhs, rhs)

    # multiple columns
    lhs = read_marc21(path, "075{ b, 2 }")
    rhs = pl.DataFrame(
        {
            "column_1": ["p", "piz"],
            "column_2": ["gndgen", "gndspec"],
        },
    )
    assert_frame_equal(lhs, rhs)

    # constraint
    lhs = read_marc21(path, "075{ b | 2 == 'gndgen' }")
    rhs = pl.DataFrame({"column_1": ["p"]})
    assert_frame_equal(lhs, rhs)

    # discard column
    lhs = read_marc21(path, "075{ _, 2 }")
    rhs = pl.DataFrame({"column_1": ["gndgen", "gndspec"]})
    assert_frame_equal(lhs, rhs)

    lhs = read_marc21(path, "075{ b, _ }")
    rhs = pl.DataFrame({"column_1": ["p", "piz"]})
    assert_frame_equal(lhs, rhs)

    # empty path
    df = read_marc21(path, "075{ _ }")
    assert df.is_empty()
