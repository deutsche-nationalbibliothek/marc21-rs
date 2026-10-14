from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from typing import Literal

type UnicodeForm = Literal["NFC", "NFKC", "NFD", "NFKD"]

__all__ = ["UnicodeForm"]
