from collections.abc import Iterator
from pathlib import Path

class LazyReader(Iterator[list[str]]):
    def __init__(self, paths: list[Path], query: str, predicate: str | None) -> None: ...

    def arity(self) -> int: ...
