"""Python interface to the native Rust-based hmleqx parser."""

from ._core import parse, to_latex, version

# Convenient aliases matching the Rust crate's public API.
convert = to_latex
eq_to_latex = to_latex

__version__ = version()

__all__ = [
    "__version__",
    "convert",
    "eq_to_latex",
    "parse",
    "to_latex",
    "version",
]
