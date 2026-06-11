from polars_marc21 import scan_marc21

import polars as pl
from polars.testing import assert_frame_equal


def test_scan_marc21():
    lhs = scan_marc21(
        "../tests/data/DUMP.mrc.gz",
        query="001, 075{ b | 2 == 'gndgen' }",
        header="ppn,gndgen",
    )

    rhs = pl.DataFrame(
        {
            "ppn": [
                "118540238",
                "118572121",
                "118607626",
                "118632477",
                "040992020",
                "040992918",
                "040993396",
            ],
            "gndgen": list("ppppuuu"),
        }
    ).lazy()

    assert_frame_equal(lhs, rhs)
