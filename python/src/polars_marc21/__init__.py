import os
from glob import glob
from pathlib import Path
from typing import Iterator

import polars as pl
from polars.io.plugins import register_io_source

from polars_marc21._core import LazyReader

def normalize_path(path: Path) -> Path:
    return path.expanduser().absolute()


def scan_marc21(
    sources: str | Path | list[str] | list[Path],
    query: str,
    header: str | list[str] | None = None,
) -> pl.LazyFrame:
    if isinstance(sources, str):
        temp = os.path.expanduser(sources)
        paths = list(map(lambda x: Path(x), glob(temp)))
    elif isinstance(sources, list):
        temp = map(lambda x: os.path.expanduser(x), sources)
        paths = list(map(lambda x: Path(x), temp))
    else:
        paths = [Path(sources)]
    paths = list(map(normalize_path, paths))
    reader = LazyReader(paths, query)

    if not header:
        header = [f"col_{i}" for i in range(0, reader.arity())]
    elif isinstance(header, str):
        header = list(map(str.strip, header.split(",")))

    assert len(header) == reader.arity()

    schema = pl.Schema({k: pl.String for k in header})

    def source_generator(
        with_columns: list[str] | None,
        predicate: pl.Expr | None,
        n_rows: int | None,
        batch_size: int | None,
    ) -> Iterator[pl.DataFrame]:

        if batch_size is None:
            batch_size = 100

        while n_rows is None or n_rows > 0:
            if n_rows is not None:
                batch_size = min(batch_size, n_rows)

            rows = []

            for _ in range(batch_size):
                try:
                    row = next(reader)
                except StopIteration:
                    n_rows = 0
                    break
                except Exception as e:
                    print(e) # FIXME
                    break
                rows.append(row)

            df = pl.from_records(rows, schema=schema, orient="row")
            if n_rows is not None:
                n_rows -= df.height

            if with_columns is not None:
                df = df.select(with_columns)

            if predicate is not None:
                df = df.filter(predicate)

            yield df

    return register_io_source(io_source=source_generator, schema=schema)
