"""Query Test Suite."""

from pathlib import Path

import polars as pl
from polars.testing import assert_frame_equal

from polars_marc21 import scan_marc21

__all__ = []


def test_empty_query(data_dir: Path) -> None:
    """Ensures that empty (sub-)fields result in an empty column."""
    path = data_dir.joinpath("ada.mrc")

    df = scan_marc21(path, "009").collect()
    assert isinstance(df, pl.DataFrame)
    assert df.is_empty()

    df = scan_marc21(path, "042.b").collect()
    assert isinstance(df, pl.DataFrame)
    assert df.is_empty()

    df = scan_marc21(path, "042{ b }").collect()
    assert isinstance(df, pl.DataFrame)
    assert df.is_empty()


def test_query_control_field(data_dir: Path) -> None:
    path = data_dir.joinpath("ada.mrc")

    # non-repeated control field
    expected = pl.DataFrame({"column_1": ["119232022"]})
    actual = scan_marc21(path, "001").collect()
    assert isinstance(actual, pl.DataFrame)
    assert_frame_equal(actual, expected)

    # non-repeated fixed length control field
    expected = pl.DataFrame({"column_1": ["20250720173911.0"]})
    actual = scan_marc21(path, "005").collect()
    assert isinstance(actual, pl.DataFrame)
    assert_frame_equal(actual, expected)

    # single fixed length control field (with range)
    expected = pl.DataFrame({"column_1": ["20250720"]})
    actual = scan_marc21(path, "005[0:8]").collect()
    assert isinstance(actual, pl.DataFrame)
    assert_frame_equal(actual, expected)

    expected = pl.DataFrame({"column_1": ["0720"]})
    actual = scan_marc21(path, "005[4:8]").collect()
    assert isinstance(actual, pl.DataFrame)
    assert_frame_equal(actual, expected)

    # right-open range
    expected = pl.DataFrame({"column_1": ["173911.0"]})
    actual = scan_marc21(path, "005[8:]").collect()
    assert isinstance(actual, pl.DataFrame)
    assert_frame_equal(actual, expected)

    # left-open range
    expected = pl.DataFrame({"column_1": ["20250720"]})
    actual = scan_marc21(path, "005[:8]").collect()
    assert isinstance(actual, pl.DataFrame)
    assert_frame_equal(actual, expected)

    # out of range
    actual = scan_marc21(path, "005[100:2]").collect()
    assert isinstance(actual, pl.DataFrame)
    assert actual.is_empty()


def test_query_leader(data_dir: Path) -> None:
    path = data_dir.joinpath("ada.mrc")

    # base address
    expected = pl.DataFrame({"column_1": ["589"]})
    actual = scan_marc21(path, "ldr.base_address").collect()
    assert isinstance(actual, pl.DataFrame)
    assert_frame_equal(actual, expected)

    # encoding
    expected = pl.DataFrame({"column_1": ["a"]})
    actual = scan_marc21(path, "ldr.encoding").collect()
    assert isinstance(actual, pl.DataFrame)
    assert_frame_equal(actual, expected)

    # length
    expected = pl.DataFrame({"column_1": ["3612"]})
    actual = scan_marc21(path, "ldr.length").collect()
    assert isinstance(actual, pl.DataFrame)
    assert_frame_equal(actual, expected)

    # status
    expected = pl.DataFrame({"column_1": ["n"]})
    actual = scan_marc21(path, "ldr.status").collect()
    assert isinstance(actual, pl.DataFrame)
    assert_frame_equal(actual, expected)

    # type
    expected = pl.DataFrame({"column_1": ["z"]})
    actual = scan_marc21(path, "ldr.type").collect()
    assert isinstance(actual, pl.DataFrame)
    assert_frame_equal(actual, expected)


def test_query_data_field(data_dir: Path) -> None:
    path = data_dir.joinpath("ada.mrc")

    # non-repeated field, non-repeated subfield
    expected = pl.DataFrame({"column_1": ["gnd1"]})
    actual = scan_marc21(path, "042.a").collect()
    assert isinstance(actual, pl.DataFrame)
    assert_frame_equal(actual, expected)

    # non-repeated field, repeated subfield
    expected = pl.DataFrame({"column_1": list("fsz")})
    actual = scan_marc21(path, "079.q").collect()
    assert isinstance(actual, pl.DataFrame)
    assert_frame_equal(actual, expected)

    # repeated field, non-repeated subfield
    expected = pl.DataFrame({"column_1": ["28p", "9.5p"]})
    actual = scan_marc21(path, "065.a").collect()
    assert isinstance(actual, pl.DataFrame)
    assert_frame_equal(actual, expected)

    # predicate
    expected = pl.DataFrame({"column_1": ["p"]})
    actual = scan_marc21(path, "075{ b | 2 == 'gndgen' }").collect()
    assert isinstance(actual, pl.DataFrame)
    assert_frame_equal(actual, expected)

    expected = pl.DataFrame({"column_1": ["piz"]})
    actual = scan_marc21(
        path, "075{ b | 2 == 'gndspec' && b =^ 'p' }",
    ).collect()
    assert isinstance(actual, pl.DataFrame)
    assert_frame_equal(actual, expected)


def test_query_cartesian_product1(data_dir: Path) -> None:
    path = data_dir.joinpath("ada.mrc")

    # catesian product of:
    # --------------------
    #   - non-repeated field, non-repeated subfield
    #   - non-repeated field, non-repeated subfield
    expected = pl.DataFrame(
        {
            "column_1": "119232022",
            "column_2": "gnd1",
        },
    )

    actual = scan_marc21(path, "001,042.a").collect()
    assert isinstance(actual, pl.DataFrame)
    assert_frame_equal(actual, expected)

    # catesian product of:
    # --------------------
    #   - non-repeated field, non-repeated subfield
    #   - non-repeated field, repeated subfield
    expected = pl.DataFrame(
        {
            "column_1": ["gnd1"] * 3,
            "column_2": list("fsz"),
        },
    )

    actual = scan_marc21(path, "042.a,079.q").collect()
    assert isinstance(actual, pl.DataFrame)
    assert_frame_equal(actual, expected)

    # catesian product of:
    # --------------------
    #   - non-repeated field, non-repeated subfield
    #   - repeated field, repeated subfield
    expected = pl.DataFrame(
        {
            "column_1": ["gnd1"] * 2,
            "column_2": ["p", "piz"],
        },
    )

    actual = scan_marc21(path, "042.a,075.b").collect()
    assert isinstance(actual, pl.DataFrame)
    assert_frame_equal(actual, expected)

    # catesian product of:
    # --------------------
    #   - repeated field, non-repeated subfield
    #   - non-repeated field, repeated subfield
    expected = pl.DataFrame(
        {
            "column_1": ["p", "p", "p", "piz", "piz", "piz"],
            "column_2": list("wkv") * 2,
        },
    )

    actual = scan_marc21(path, "075.b,079.u").collect()
    assert isinstance(actual, pl.DataFrame)
    assert_frame_equal(actual, expected)


def test_query_cartesian_product2(data_dir: Path) -> None:
    path = data_dir.joinpath("ada.mrc")

    # catesian product of:
    # --------------------
    #   - non-repeated subfield
    #   - repeated subfield
    expected = pl.DataFrame(
        {
            "column_1": ["g"] * 3,
            "column_2": list("fsz"),
        },
    )

    actual = scan_marc21(path, "079{ a, q }").collect()
    assert isinstance(actual, pl.DataFrame)
    assert_frame_equal(actual, expected)

    # catesian product of:
    # --------------------
    #   - repeated subfield
    #   - repeated subfield
    expected = pl.DataFrame(
        {
            "column_1": list("fffssszzz"),
            "column_2": list("wkvwkvwkv"),
        },
    )

    actual = scan_marc21(path, "079{ q, u }").collect()
    assert isinstance(actual, pl.DataFrame)
    assert_frame_equal(actual, expected)

    # catesian product of:
    # --------------------
    #   - repeated subfield
    #   - non-existing subfield
    expected = pl.DataFrame(
        {
            "column_1": list("fsz"),
            "column_2": [""] * 3,
        },
    )

    actual = scan_marc21(path, "079{ q, z }").collect()
    assert isinstance(actual, pl.DataFrame)
    assert_frame_equal(actual, expected)


def test_query_cartesian_product3(data_dir: Path) -> None:
    path = data_dir.joinpath("ada.mrc")

    # catesian product of:
    # --------------------
    #   - non-repeated subfield
    #   - repeated subfield
    expected = pl.DataFrame(
        {
            "column_1": ["119232022"] * 3,
            "column_2": ["g"] * 3,
            "column_3": list("fsz"),
        },
    )

    actual = scan_marc21(path, "001,079{ a, q }").collect()
    assert isinstance(actual, pl.DataFrame)
    assert_frame_equal(actual, expected)
