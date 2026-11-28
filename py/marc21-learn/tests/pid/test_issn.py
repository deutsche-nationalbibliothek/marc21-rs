"""Test  functions."""

from marc21_learn.pid import issn_validate


def test_issn_validate() -> None:
    """Tests the `issn_validate` function."""
    assert issn_validate("2194-8860")
    assert issn_validate("2569-7641")
    assert issn_validate("0038-7452")
    assert issn_validate("0720-6763")
    assert issn_validate("2191-4664")
    assert issn_validate("2367-2765")
    assert issn_validate("2509-8586")
    assert issn_validate("1861-6097")
    assert issn_validate("1011-0798")
    assert issn_validate("0040-5329")

    # invalid checksum
    assert not issn_validate("2194-8861")

    # invalid separator
    assert not issn_validate("2194 8860")

    # invalid length
    assert not issn_validate("2194-8860 (ISSN)")

    # invalid size
    assert not issn_validate("2194-886")
    assert not issn_validate("")
