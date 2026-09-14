from pathlib import Path

import numpy as np
from numpy.testing import assert_allclose

from marc21_learn import scale_array


def test_empty_query(data_dir: Path) -> None:
    X = np.array([2.0, 3.0])

    actual = np.array(scale_array(2.0, X))
    expected = np.array([4.0, 6.0])

    assert_allclose(actual, expected)
