"""Test Digital Object Identifier (DOI) functions."""

import pytest

from marc21_learn.pid import doi_from_isbn


def test_doi_from_isbn() -> None:
    """Tests the `isbn_validate` function."""
    assert doi_from_isbn("978-3-10-031820-6") == "10.978.310/0318206"
    assert doi_from_isbn("9783100318206") == "10.978.310/0318206"
    assert doi_from_isbn("3-10-031820-X") == "10.978.310/0318206"
    assert doi_from_isbn("310031820X") == "10.978.310/0318206"

    # strict
    assert doi_from_isbn("9783100318207", strict=False) is None

    with pytest.raises(
        ValueError,
        match=r"The value '9783100318207' is not a valid ISBN\.",
    ):
        doi_from_isbn("9783100318207", strict=True)
