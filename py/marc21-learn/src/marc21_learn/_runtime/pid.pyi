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
