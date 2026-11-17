from marc21_learn import Match
from marc21_learn._typing import IsbnVersion

def isbn_validate(
    value: str,
    *,
    version: IsbnVersion | None = None,
) -> bool: ...
def isbn_canonicalize(
    value: str,
    *,
    hyphenate: bool = True,
    strict: bool = False,
) -> str | None: ...
def isbn_find(haystack: str, *, validate: bool = True) -> Match: ...
