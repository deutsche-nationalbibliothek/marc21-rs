"""Smoke test for `pyton-polars` wheen and sdist."""

import sys


def success(message: str) -> None:
    print(f"SUCCESS {message}")


def failure(message: str) -> None:
    print(f"FAILURE {message}", file=sys.stderr)
    sys.exit(1)


# check package import
try:
    import polars_marc21

    success("import `polars_marc21`")
except ImportError as e:
    failure(f"import `polars_marc21`: {e}")

# check scan_marc21
try:
    df = polars_marc21.scan_marc21(
        "../tests/data/ada.mrc",
        "001",
    ).collect()

    assert df.height == 1

    success("call `scan_marc21`")
except AssertionError as e:
    failure(f"check `scan_marc21`: {e}")
