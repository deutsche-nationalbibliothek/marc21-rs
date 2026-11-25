#!/usr/bin/env -S uv run -s --group bench

"""Benchmark ISBN validation with `pyisbn`."""

import csv
import sys
from argparse import ArgumentParser
from pathlib import Path

from pyisbn import Isbn13


def main() -> None:
    parser = ArgumentParser()
    parser.add_argument("input", type=Path)

    args = parser.parse_args()
    count = 0

    with args.input.open(newline="") as f:
        reader = csv.DictReader(f)
        for row in reader:
            if not Isbn13(row["isbn"]).validate():  # ty: ignore[invalid-argument-type]
                print(row["isbn"], file=sys.stderr)
                count += 1

    print(count)


if __name__ == "__main__":
    main()
