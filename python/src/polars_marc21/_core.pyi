from pathlib import Path
from typing import Iterator

class LazyReader(Iterator[list[str]]):
    def __init__(self, paths: list[Path], query: str) -> None:...

    def arity(self) -> int: ...

