"""Test Polars Integration."""

import pytest

from marc21_learn.pid import isbn_canonicalize, isbn_validate


def test_isbn_validate() -> None:
    """Tests the `isbn_validate` function."""
    assert isbn_validate("978-3-10-031820-6")
    assert isbn_validate("978 3 10 031820 6")
    assert isbn_validate("9783100318206")

    assert isbn_validate("3-10-031820-X")
    assert isbn_validate("3 10 031820 X")
    assert isbn_validate("310031820X")

    assert isbn_validate("978-3-10-031820-6", version="ISBN-13")
    assert not isbn_validate("978-3-10-031820-6", version="ISBN-10")
    assert not isbn_validate("3-10-031820-X", version="ISBN-13")
    assert isbn_validate("3-10-031820-X", version="ISBN-10")

    with pytest.raises(
        ValueError,
        match=r"`version` must be one of {'ISBN-13', 'ISBN-10'}",
    ):
        isbn_validate("978-3-10-031820-6", version="ISBN-12")  # ty: ignore


def test_isbn_canonicalize() -> None:
    """Tests the `isbn_validate` function."""
    assert isbn_canonicalize("978-3-10-031820-6") == "978-3-10-031820-6"
    assert isbn_canonicalize("9783100318206") == "978-3-10-031820-6"

    assert isbn_canonicalize("3-10-031820-X") == "978-3-10-031820-6"
    assert isbn_canonicalize("310031820X") == "978-3-10-031820-6"
    assert isbn_canonicalize("310031820x") == "978-3-10-031820-6"

    # hypehnate
    assert (
        isbn_canonicalize("978-3-10-031820-6", hyphenate=False)
        == "9783100318206"
    )
    assert (
        isbn_canonicalize("978-3-10-031820-6", hyphenate=True)
        == "978-3-10-031820-6"
    )

    # strict
    assert not isbn_canonicalize("978-3-10-031820-7", strict=False)
    with pytest.raises(
        ValueError,
        match=r"The value '978-3-10-031820-7' is not a valid ISBN.",
    ):
        isbn_canonicalize("978-3-10-031820-7", strict=True)
