"""Configuration file for the Sphinx documentation builder."""

import sys
from pathlib import Path

from marc21_learn import __version__

#
# PROJECT INFORMATION
#

project = "marc21-learn"
copyright = "2026, Nico Wagner"  # noqa: A001
author = "Nico Wagner"
release = __version__

#
# GENERAL CONFIGURATION
#

# Ensuring that the project code can be imported
sys.path.insert(0, str(Path("..", "src").resolve()))

extensions = [
    "sphinx.ext.autodoc",
    "sphinx.ext.autosummary",
    "sphinx.ext.doctest",
    "sphinx.ext.githubpages",
    "sphinx.ext.intersphinx",
    "sphinx.ext.mathjax",
    "numpydoc",
]

add_module_names = False
autosummary_generate = True
default_role = "code"
exclude_patterns = ["_build", "Thumbs.db", ".DS_Store"]
maximum_signature_line_length = 78
modindex_common_prefix = ["marc21_learn."]
numpydoc_show_class_members = False
root_doc = "index"
source_encoding = "utf-8"
templates_path = ["_templates"]


# Configure intersphinx extension
intersphinx_mapping = {
    "python": (f"https://docs.python.org/{sys.version_info.major}", None),
    "matplotlib": ("https://matplotlib.org/stable/", None),
    "numpy": ("https://numpy.org/doc/stable", None),
    "pandas": ("https://pandas.pydata.org/docs/", None),
    "polars": ("https://docs.pola.rs/api/python/stable", None),
    "scikit-learn": ("https://scikit-learn.org/stable/", None),
    "scipy": ("https://docs.scipy.org/doc/scipy/", None),
    "seaborn": ("https://seaborn.pydata.org/", None),
}

doctest_show_successes = True

#
# HTML OUTPUT
#

html_theme = "pydata_sphinx_theme"
html_short_title = "scikit-learn"
html_logo = "_static/img/logo.png"
html_static_path = ["_static"]
html_theme_options = {
    "back_to_top_button": False,
    "navbar_align": "left",
    "logo": {
        "text": "marc21-learn",
    },
    "icon_links": [
        {
            "name": "GitHub",
            "url": "https://github.com/deutsche-nationalbibliothek/marc21-rs/",
            "icon": "fa-brands fa-square-github",
            "type": "fontawesome",
        },
    ],
    "show_prev_next": False,
    "collapse_navigation": False,
}

html_css_files = [
    "custom.css",
]
