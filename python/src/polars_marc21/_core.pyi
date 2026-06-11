from typing import Iterator
from pathlib import Path


class LazyReader(Iterator[list[str]]):
    def __init__(self, paths: list[Path], query: str) -> None: ...

    def arity(self) -> int: ...
