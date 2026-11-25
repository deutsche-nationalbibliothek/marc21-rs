#!/usr/bin/env -S uv run -s --group bench

"""Benchmark ISBN validation with `pydantic`."""

import csv
import sys
from argparse import ArgumentParser
from pathlib import Path

from pydantic_core import PydanticCustomError
from pydantic_extra_types.isbn import ISBN


def main() -> None:
    parser = ArgumentParser()
    parser.add_argument("input", type=Path)

    args = parser.parse_args()
    count = 0

    with args.input.open(newline="") as f:
        reader = csv.DictReader(f)
        for row in reader:
            try:
                ISBN.validate_isbn_format(row["isbn"])
            except PydanticCustomError:
                print(row["isbn"], file=sys.stderr)
                count += 1

    print(count)


if __name__ == "__main__":
    main()
