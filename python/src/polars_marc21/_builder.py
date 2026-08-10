from pathlib import Path
from typing import Self

from polars import DataFrame, LazyFrame

from ._scan import scan_marc21


class QueryBuilder:
    _query: str | None = None
    _sources: str | Path | list[str] | list[Path] | None = None
    _predicate: str | None = None
    _header: str | list[str] | None = None

    def select(self, query: str) -> Self:
        self._query = query
        return self

    def from_(
        self,
        sources: str | Path | list[str] | list[Path],
    ) -> Self:
        self._sources = sources
        return self

    def where(self, predicate: str) -> Self:
        if self._predicate:
            self._predicate = f"({self._predicate}) && " + predicate
        else:
            self._predicate = predicate

        return self

    def and_(self, predicate: str) -> Self:
        self.where(predicate)
        return self

    def or_(self, predicate: str) -> Self:
        if self._predicate:
            self._predicate = f"({self._predicate}) || " + predicate
        else:
            self._predicate = predicate

        return self

    def header(self, header: str | list[str]) -> Self:
        self._header = header
        return self

    def scan(self) -> LazyFrame:
        return scan_marc21(
            self._sources,
            self._query,
            header=self._header,
            where=self._predicate,
        )

    def collect(self) -> DataFrame:
        return self.scan().collect()


def marc21_query() -> QueryBuilder:
    return QueryBuilder()


def marc21_select(query: str) -> QueryBuilder:
    return QueryBuilder().select(query)
