from pathlib import Path
from polars_marc21._core import scan_marc21_impl
import polars as pl
from typing import Iterator
from polars.io.plugins import register_io_source


def normalize_path(path: Path) -> Path:
    return path.expanduser().absolute()


def scan_marc21(
    source: str | Path,
    header: str | list[str],
    query: str,
) -> pl.LazyFrame:
    if isinstance(source, str):
        source = Path(source)

    source = normalize_path(source)

    if isinstance(header, str):
        header = list(map(str.strip, header.split(",")))
    schema = pl.Schema({k: pl.String for k in header})

    def source_generator(
        with_columns: list[str] | None,
        predicate: pl.Expr | None,
        n_rows: int | None,
        batch_size: int | None,
    ) -> Iterator[pl.DataFrame]:

        if batch_size is None:
            batch_size = 100

        reader = scan_marc21_impl(source, query)

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


def main() -> None:

    path = "~/tmp/marc/authorities-gnd-sachbegriff_dnbmarc.mrc"
    query = "001, 075{ b | 2 == 'gndgen' }, 150{ a, g }"
    header = ["cn", "gndgen", "label", "suffix"]
    df = (
        scan_marc21(path, header, query)
        .filter((pl.col("label") == "Python") | (pl.col("cn") == "1078438080"))
        .collect()
    )
    print(df)
