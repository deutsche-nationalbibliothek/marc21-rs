"""Shared test fixtures."""

from pathlib import Path

import pytest


@pytest.fixture(scope="module")
def data_dir() -> Path:
    """Return the tests data directory."""
    return Path().cwd().joinpath("../tests/data").resolve()
