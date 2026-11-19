from marc21_learn import Match
from marc21_learn._typing import IsbnElement, IsbnVersion

def doi_from_isbn(
    value: str,
    strict: bool = False,
) -> str | None: ...
def isbn_from_doi(
    value: str,
    *,
    hyphenate: bool = True,
    strict: bool = False,
) -> str | None: ...
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
def isbn_findall(haystack: str, *, validate: bool = True) -> list[Match]: ...
def isbn_truncate(
    value: str,
    *,
    element: IsbnElement = "registrant",
    hyphenate: bool = True,
    strict: bool = False,
) -> str | None: ...
