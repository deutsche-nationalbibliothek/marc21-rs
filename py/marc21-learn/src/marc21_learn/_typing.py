from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from typing import Literal

type UnicodeForm = Literal[
    "NFC",
    "NFKC",
    "NFD",
    "NFKD",
]

type IsbnVersion = Literal[
    "ISBN-13",
    "ISBN-10",
]

type IsbnElement = Literal[
    "prefix",
    "group",
    "registrant",
    "publication",
    "check-digit",
]

__all__ = [
    "IsbnElement",
    "IsbnVersion",
    "UnicodeForm",
]
