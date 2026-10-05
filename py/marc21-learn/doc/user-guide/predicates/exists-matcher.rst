:html_theme.sidebar_secondary.remove:

.. _ref-exists-matcher:

Exists Matcher
==============

The *exists matcher* has already been used to illustrate how a :ref:`tag
matcher <ref-tag-matcher>` or :ref:`indicator matcher <ref-indicator-matcher>`
works. It can be used to verify the existence of a field. The
expression consists of a :ref:`tag matcher <ref-tag-matcher>`, an
optional :ref:`indicator matcher <ref-indicator-matcher>`, followed by a
question mark ``?`` character.

The matcher evaluates to true if at least one field was found that matches the
specified *tag matcher* and, if applicable, the *indicator matcher*.

Examples:

	>>> from marc21_learn.io import read_marc21
	>>>
	>>> filename = "../tests/data/ada.mrc.gz"
	>>>
	>>> df = read_marc21(filename, "001", predicate="024/7#?")
	>>> assert df.height == 1
	>>>
	>>> df = read_marc21(filename, "001", predicate="0[45]5/*?")
	>>> assert df.height == 0
