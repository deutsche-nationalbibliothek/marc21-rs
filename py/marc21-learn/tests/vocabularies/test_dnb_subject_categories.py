"""Test DNB Vocabularies."""

import polars as pl

from marc21_learn.vocabularies import dnb_subject_categories

DNB_SUBJECT_CATEGORIES: list[str] = [
    "000",
    "004",
    "010",
    "020",
    "030",
    "050",
    "060",
    "070",
    "080",
    "090",
    "100",
    "130",
    "150",
    "200",
    "220",
    "230",
    "290",
    "300",
    "310",
    "320",
    "330",
    "333.7",
    "340",
    "350",
    "355",
    "360",
    "370",
    "380",
    "390",
    "400",
    "420",
    "430",
    "439",
    "440",
    "450",
    "460",
    "470",
    "480",
    "490",
    "491.8",
    "500",
    "510",
    "520",
    "530",
    "540",
    "550",
    "560",
    "570",
    "580",
    "590",
    "600",
    "610",
    "620",
    "621.3",
    "624",
    "630",
    "640",
    "650",
    "660",
    "670",
    "690",
    "700",
    "710",
    "720",
    "730",
    "740",
    "741.5",
    "750",
    "760",
    "770",
    "780",
    "790",
    "791",
    "792",
    "793",
    "796",
    "800",
    "810",
    "820",
    "830",
    "839",
    "840",
    "850",
    "860",
    "870",
    "880",
    "890",
    "891.8",
    "900",
    "910",
    "914.3",
    "920",
    "930",
    "940",
    "943",
    "950",
    "960",
    "970",
    "980",
    "990",
    "B",
    "K",
    "S",
]


def test_default() -> None:
    """Test the `dnb_subject_categories` function without params."""
    # check for completnes
    dnb_sc = dnb_subject_categories()
    assert dnb_sc.height == 103
    assert dnb_sc.width == 4

    # check for uniqueness
    for notation in DNB_SUBJECT_CATEGORIES:
        assert dnb_sc.filter(pl.col("notation") == notation).height == 1

    assert "label_de" in dnb_sc
    assert "label_en" in dnb_sc
    assert "note_de" not in dnb_sc
    assert "note_en" not in dnb_sc


def test_language() -> None:
    """Test the `dnb_subject_categories` function without params."""
    dnb_sc = dnb_subject_categories(language="de")
    assert dnb_sc.height == 103
    assert dnb_sc.width == 3
    assert "label" in dnb_sc

    dnb_sc = dnb_subject_categories(language="en")
    assert dnb_sc.height == 103
    assert dnb_sc.width == 3
    assert "label" in dnb_sc


def test_include_note() -> None:
    """Test the `dnb_subject_categories` function without params."""
    dnb_sc = dnb_subject_categories(include_note=True)
    assert dnb_sc.height == 103
    assert dnb_sc.width == 6

    assert "note_de" in dnb_sc
    assert "note_en" in dnb_sc

    dnb_sc = dnb_subject_categories(include_note=False)
    assert dnb_sc.height == 103
    assert dnb_sc.width == 4

    assert "note_de" not in dnb_sc
    assert "note_en" not in dnb_sc


def test_normalize() -> None:
    """Test the normalize parameter."""
    # test that result is NFC encoded
    lhs = dnb_subject_categories(language="de").with_columns(
        pl.col("label").str.len_bytes().alias("len"),
    )
    assert lhs.height == 103
    assert lhs.width == 4

    rhs = dnb_subject_categories(language="de", normalize="NFC").with_columns(
        pl.col("label").str.len_bytes().alias("len"),
    )
    assert rhs.height == 103
    assert rhs.width == 4

    assert (
        lhs.join(rhs, on="notation", how="inner")
        .filter(pl.col("len") != pl.col("len_right"))
        .is_empty()
    )

    # NFC
    df = dnb_subject_categories(language="de", normalize="NFC").with_columns(
        pl.col("label").str.len_bytes().alias("len"),
    )
    assert df.height == 103
    assert df.width == 4

    assert (
        df.filter(pl.col("notation") == "980")
        .filter(pl.col("len") == 23)
        .height
        == 1
    )

    # NFD
    df = dnb_subject_categories(language="de", normalize="NFD").with_columns(
        pl.col("label").str.len_bytes().alias("len"),
    )
    assert df.height == 103
    assert df.width == 4

    assert (
        df.filter(pl.col("notation") == "980")
        .filter(pl.col("len") == 24)
        .height
        == 1
    )
