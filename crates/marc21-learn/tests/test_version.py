from pathlib import Path

import marc21_learn
import pytest
from packaging import version


def test_empty_query(data_dir: Path) -> None:
    try:
        version.parse(marc21_learn.__version__)
    except version.InvalidVersion:
        pytest.fail("invalid package version")
