from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from typing import Literal

type UnicodeForm = Literal["NFC", "NFKC", "NFD", "NFKD"]

type IsbnVersion = Literal["ISBN-13", "ISBN-10"]

__all__ = [
    "IsbnVersion",
    "UnicodeForm",
]
