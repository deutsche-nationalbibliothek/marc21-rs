"""Test Polars Integration."""

import pytest

from marc21_learn.pid import (
    isbn_canonicalize,
    isbn_find,
    isbn_findall,
    isbn_truncate,
    isbn_validate,
)


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


def test_isbn_find() -> None:
    """Tests the `isbn_find` function."""
    match = isbn_find("978-1-4493-1943-4")
    assert match.value == "978-1-4493-1943-4"
    assert match.start == 0
    assert match.end == 17

    match = isbn_find("9781449319434")
    assert match.value == "9781449319434"
    assert match.start == 0
    assert match.end == 13

    match = isbn_find("978 1 4493 1943 4")
    assert match.value == "978 1 4493 1943 4"
    assert match.start == 0
    assert match.end == 17

    match = isbn_find("1-4493-1943-2")
    assert match.value == "1-4493-1943-2"
    assert match.start == 0
    assert match.end == 13

    match = isbn_find("1 4493 1943 2")
    assert match.value == "1 4493 1943 2"
    assert match.start == 0
    assert match.end == 13

    match = isbn_find("1449319432")
    assert match.value == "1449319432"
    assert match.start == 0
    assert match.end == 10

    match = isbn_find("978-1-4493-1943-4", validate=True)
    assert match.value == "978-1-4493-1943-4"
    assert match.start == 0
    assert match.end == 17

    match = isbn_find("978-1-4493-1943-4", validate=False)
    assert match.value == "978-1-4493-1943-4"
    assert match.start == 0
    assert match.end == 17

    assert not isbn_find("978-1-4493-1943-5", validate=True)
    match = isbn_find("978-1-4493-1943-5", validate=False)
    assert match.value == "978-1-4493-1943-5"
    assert match.start == 0
    assert match.end == 17

    assert not isbn_find("978-1-4493-1943-5")
    assert not isbn_find("foobar")


def test_isbn_findall() -> None:
    """Tests the `isbn_findall` function."""
    matches = isbn_findall("978-1-4493-1943-4")
    assert len(matches) == 1
    assert matches[0].value == "978-1-4493-1943-4"
    assert matches[0].start == 0
    assert matches[0].end == 17

    matches = isbn_findall("ISBN-13: 978-1-4493-1943-4")
    assert len(matches) == 1
    assert matches[0].value == "978-1-4493-1943-4"
    assert matches[0].start == 9
    assert matches[0].end == 26

    matches = isbn_findall("ISBN-13: 978-1-0981-7130-8 ISBN-10: 1098171306")
    assert len(matches) == 2
    assert matches[0].value == "978-1-0981-7130-8"
    assert matches[0].start == 9
    assert matches[0].end == 26
    assert matches[1].value == "1098171306"
    assert matches[1].start == 36
    assert matches[1].end == 46

    assert not isbn_findall("978-1-4493-1943-5", validate=True)
    matches = isbn_findall("978-1-4493-1943-5", validate=False)
    assert len(matches) == 1
    assert matches[0].value == "978-1-4493-1943-5"
    assert matches[0].start == 0
    assert matches[0].end == 17


def test_isbn_truncate() -> None:
    """Tests the `isbn_truncate` function."""
    # ISBN-13 / hyphenate=True (default)
    isbn = "978-1098156084"
    assert isbn_truncate(isbn) == "978-1-0981"
    assert isbn_truncate(isbn, element="prefix") == "978"
    assert isbn_truncate(isbn, element="group") == "978-1"
    assert isbn_truncate(isbn, element="registrant") == "978-1-0981"
    assert isbn_truncate(isbn, element="publication") == "978-1-0981-5608"
    assert isbn_truncate(isbn, element="check-digit") == "978-1-0981-5608-4"

    # ISBN-10 / hyphenate=True (default)
    isbn = "1098156080"
    assert isbn_truncate(isbn) == "978-1-0981"
    assert isbn_truncate(isbn, element="prefix") == "978"
    assert isbn_truncate(isbn, element="group") == "978-1"
    assert isbn_truncate(isbn, element="registrant") == "978-1-0981"
    assert isbn_truncate(isbn, element="publication") == "978-1-0981-5608"
    assert isbn_truncate(isbn, element="check-digit") == "978-1-0981-5608-4"

    # ISBN-13 / hyphenate=False
    isbn = "978-1098156084"
    assert isbn_truncate(isbn, hyphenate=False) == "97810981"
    assert isbn_truncate(isbn, element="prefix", hyphenate=False) == "978"
    assert isbn_truncate(isbn, element="group", hyphenate=False) == "9781"
    assert (
        isbn_truncate(isbn, element="registrant", hyphenate=False)
        == "97810981"
    )
    assert (
        isbn_truncate(isbn, element="publication", hyphenate=False)
        == "978109815608"
    )
    assert (
        isbn_truncate(isbn, element="check-digit", hyphenate=False)
        == "9781098156084"
    )

    # ISBN-10 / hyphenate=False
    isbn = "1098156080"
    assert isbn_truncate(isbn, hyphenate=False) == "97810981"
    assert isbn_truncate(isbn, element="prefix", hyphenate=False) == "978"
    assert isbn_truncate(isbn, element="group", hyphenate=False) == "9781"
    assert (
        isbn_truncate(isbn, element="registrant", hyphenate=False)
        == "97810981"
    )
    assert (
        isbn_truncate(isbn, element="publication", hyphenate=False)
        == "978109815608"
    )
    assert (
        isbn_truncate(isbn, element="check-digit", hyphenate=False)
        == "9781098156084"
    )

    # strict
    assert not isbn_truncate("978-3-10-031820-7", strict=False)
    with pytest.raises(
        ValueError,
        match=r"The value '978-3-10-031820-7' is not a valid ISBN.",
    ):
        isbn_truncate("978-3-10-031820-7", strict=True)
