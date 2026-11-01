from marc21_learn._typing import IsbnVersion

def isbn_validate(
    value: str,
    *,
    version: IsbnVersion | None = None,
) -> bool: ...
