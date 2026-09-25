"""Tests of the top-level module."""

import pytest
from packaging import version

import marc21_learn


def test_version() -> None:
    """Ensure that the version string can be parsed."""
    try:
        version.parse(marc21_learn.__version__)
    except version.InvalidVersion:
        pytest.fail("invalid package version")
