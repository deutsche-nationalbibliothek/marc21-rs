#!/usr/bin/env -S uv run -s --group bench

"""Benchmark ISBN validation with `is-isbn`."""

import csv
import sys
from argparse import ArgumentParser
from pathlib import Path

from is_isbn.is_isbn import is_isbn


def main() -> None:
    """Foo."""
    parser = ArgumentParser()
    parser.add_argument("input", type=Path)

    args = parser.parse_args()
    count = 0

    with args.input.open(newline="") as f:
        reader = csv.DictReader(f)
        for row in reader:
            if not is_isbn(row["isbn"]):
                print(row["isbn"], file=sys.stderr)
                count += 1

    print(count)


if __name__ == "__main__":
    main()
