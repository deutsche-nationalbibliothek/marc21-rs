#!/usr/bin/env -S uv run -s --group bench

"""Benchmark ISSN validation with `marc21-learn`."""

import csv
import sys
from argparse import ArgumentParser
from pathlib import Path

from marc21_learn.pid import issn_validate


def main() -> None:
    parser = ArgumentParser()
    parser.add_argument("input", type=Path)

    args = parser.parse_args()
    count = 0

    with args.input.open(newline="") as f:
        reader = csv.DictReader(f)
        for row in reader:
            if not issn_validate(row["issn"]):
                print(row["issn"], file=sys.stderr)
                count += 1

    print(count)


if __name__ == "__main__":
    main()
