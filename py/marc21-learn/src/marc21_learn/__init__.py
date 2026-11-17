"""MARC21 learn."""

from marc21_learn._runtime import Match, __version

from . import io, pid, vocabularies

__version__: str = __version()
del __version

__all__ = [
    "Match",
    "io",
    "pid",
    "vocabularies",
]
