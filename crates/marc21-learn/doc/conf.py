"""Configuration file for the Sphinx documentation builder."""

import sys
from pathlib import Path

#
# PROJECT INFORMATION
#

project = "marc21-learn"
copyright = "2026, Nico Wagner"  # noqa: A001
author = "Nico Wagner"

#
# GENERAL CONFIGURATION
#

# Ensuring that the project code can be imported
sys.path.insert(0, str(Path("..", "src").resolve()))

extensions = [
    "sphinx.ext.autodoc",
    "sphinx.ext.autosummary",
    "sphinx.ext.mathjax",
    "numpydoc",
]

templates_path = [
    "_templates",
]

exclude_patterns = [
    "_build",
    "Thumbs.db",
    ".DS_Store",
]

add_module_names = False
modindex_common_prefix = ["marc21_learn."]

#
# HTML OUTPUT
#

html_theme = "pydata_sphinx_theme"
html_logo = "_static/logo.png"
html_static_path = ["_static"]
html_theme_options = {
    "back_to_top_button": False,
    "navbar_align": "left",
    "logo": {
        "text": "marc21-learn",
    },
}
