"""Leader Projection Tests."""

from typing import TYPE_CHECKING

import polars as pl
from polars.testing import assert_frame_equal

from marc21_learn.io import read_marc21

if TYPE_CHECKING:
    from pathlib import Path


def test_base_addr(data_dir: Path) -> None:
    """Ensures that the projection of the base address works correctly."""
    path = data_dir.joinpath("ada.mrc")

    lhs = read_marc21(path, "ldr.base_addr")
    rhs = pl.DataFrame({"column_1": [589]}, schema={"column_1": pl.UInt32})
    assert_frame_equal(lhs, rhs)

    # AS clause
    lhs = read_marc21(path, "ldr.base_addr AS `base_addr`")
    rhs = pl.DataFrame({"base_addr": [589]}, schema={"base_addr": pl.UInt32})
    assert_frame_equal(lhs, rhs)

    # prefix
    lhs = read_marc21(path, "'addr: ' ldr.base_addr")
    rhs = pl.DataFrame({"column_1": ["addr: 589"]})
    assert_frame_equal(lhs, rhs)

    # suffix
    lhs = read_marc21(path, "ldr.base_addr ' (address)'")
    rhs = pl.DataFrame({"column_1": ["589 (address)"]})
    assert_frame_equal(lhs, rhs)

    # prefix + suffix
    lhs = read_marc21(path, "'A: ' ldr.base_addr ' (B)'")
    rhs = pl.DataFrame({"column_1": ["A: 589 (B)"]})
    assert_frame_equal(lhs, rhs)


def test_length(data_dir: Path) -> None:
    """Ensures that the projection of the length works correctly."""
    path = data_dir.joinpath("ada.mrc")

    lhs = read_marc21(path, "ldr.length")
    rhs = pl.DataFrame({"column_1": [3612]}, schema={"column_1": pl.UInt32})
    assert_frame_equal(lhs, rhs)

    # AS clause
    lhs = read_marc21(path, "ldr.length AS `len`")
    rhs = pl.DataFrame({"len": [3612]}, schema={"len": pl.UInt32})
    assert_frame_equal(lhs, rhs)

    # prefix
    lhs = read_marc21(path, "'len: ' ldr.length")
    rhs = pl.DataFrame({"column_1": ["len: 3612"]})
    assert_frame_equal(lhs, rhs)

    # suffix
    lhs = read_marc21(path, "ldr.length ' (length)'")
    rhs = pl.DataFrame({"column_1": ["3612 (length)"]})
    assert_frame_equal(lhs, rhs)

    # prefix + suffix
    lhs = read_marc21(path, "'A: ' ldr.length ' (B)'")
    rhs = pl.DataFrame({"column_1": ["A: 3612 (B)"]})
    assert_frame_equal(lhs, rhs)


def test_status(data_dir: Path) -> None:
    """Ensures that the projection of the status works correctly."""
    path = data_dir.joinpath("ada.mrc")

    lhs = read_marc21(path, "ldr.status")
    rhs = pl.DataFrame({"column_1": ["n"]})
    assert_frame_equal(lhs, rhs)

    # AS clause
    lhs = read_marc21(path, "ldr.status AS `status`")
    rhs = pl.DataFrame({"status": ["n"]})
    assert_frame_equal(lhs, rhs)

    # prefix
    lhs = read_marc21(path, "'status: ' ldr.status")
    rhs = pl.DataFrame({"column_1": ["status: n"]})
    assert_frame_equal(lhs, rhs)

    # suffix
    lhs = read_marc21(path, "ldr.status ' (status)'")
    rhs = pl.DataFrame({"column_1": ["n (status)"]})
    assert_frame_equal(lhs, rhs)

    # prefix + suffix
    lhs = read_marc21(path, "'status = \"' ldr.status '\"'")
    rhs = pl.DataFrame({"column_1": ['status = "n"']})
    assert_frame_equal(lhs, rhs)


def test_encoding(data_dir: Path) -> None:
    """Ensures that the projection of the encoding field works correctly."""
    path = data_dir.joinpath("ada.mrc")

    lhs = read_marc21(path, "ldr.status")
    rhs = pl.DataFrame({"column_1": ["n"]})
    assert_frame_equal(lhs, rhs)

    # AS clause
    lhs = read_marc21(path, "ldr.encoding AS `encoding`")
    rhs = pl.DataFrame({"encoding": ["a"]})
    assert_frame_equal(lhs, rhs)

    # prefix
    lhs = read_marc21(path, "'encoding: ' ldr.encoding")
    rhs = pl.DataFrame({"column_1": ["encoding: a"]})
    assert_frame_equal(lhs, rhs)

    # suffix
    lhs = read_marc21(path, "ldr.encoding ' (encoding)'")
    rhs = pl.DataFrame({"column_1": ["a (encoding)"]})
    assert_frame_equal(lhs, rhs)

    # prefix + suffix
    lhs = read_marc21(path, "'encoding = \"' ldr.encoding '\"'")
    rhs = pl.DataFrame({"column_1": ['encoding = "a"']})
    assert_frame_equal(lhs, rhs)


def test_type(data_dir: Path) -> None:
    """Ensures that the projection of the `type` field works correctly."""
    path = data_dir.joinpath("ada.mrc")

    lhs = read_marc21(path, "ldr.type")
    rhs = pl.DataFrame({"column_1": ["z"]})
    assert_frame_equal(lhs, rhs)

    # AS clause
    lhs = read_marc21(path, "ldr.type AS `type`")
    rhs = pl.DataFrame({"type": ["z"]})
    assert_frame_equal(lhs, rhs)

    # prefix
    lhs = read_marc21(path, "'type: ' ldr.type")
    rhs = pl.DataFrame({"column_1": ["type: z"]})
    assert_frame_equal(lhs, rhs)

    # suffix
    lhs = read_marc21(path, "ldr.type ' (type)'")
    rhs = pl.DataFrame({"column_1": ["z (type)"]})
    assert_frame_equal(lhs, rhs)

    # prefix + suffix
    lhs = read_marc21(path, "'type = \"' ldr.type '\"'")
    rhs = pl.DataFrame({"column_1": ['type = "z"']})
    assert_frame_equal(lhs, rhs)
