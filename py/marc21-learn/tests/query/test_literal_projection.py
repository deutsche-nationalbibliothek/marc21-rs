"""Control Field Projection Tests."""

from typing import TYPE_CHECKING

import polars as pl
from polars.testing import assert_frame_equal

from marc21_learn.io import read_marc21

if TYPE_CHECKING:
    from pathlib import Path


def test_literal_projection(data_dir: Path) -> None:
    """Ensures that explicitly specifying a tag works."""
    path = data_dir.joinpath("ada.mrc")

    # literal
    lhs = read_marc21(path, "'foo'")
    rhs = pl.DataFrame({"column_1": ["foo"]})
    assert_frame_equal(lhs, rhs)

    # AS-clause
    lhs = read_marc21(path, "'foo' AS `info`")
    rhs = pl.DataFrame({"info": ["foo"]})
    assert_frame_equal(lhs, rhs)

    # long from
    lhs = read_marc21(path, "065{ a, 'gnd-sc' }")
    rhs = pl.DataFrame(
        {
            "column_1": ["28p", "9.5p"],
            "column_2": 2 * ["gnd-sc"],
        },
    )
    assert_frame_equal(lhs, rhs)
