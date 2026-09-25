"""MARC21 learn."""

from marc21_learn._runtime import __version

from . import io

__version__: str = __version()
del __version

__all__ = [
    "io",
]
