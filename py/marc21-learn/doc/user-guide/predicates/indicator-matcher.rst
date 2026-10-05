:html_theme.sidebar_secondary.remove:

.. _ref-indicator-matcher:

Indicator Matcher
=================

Data fields are distinguished not only by the tag, but also by indicators
consisting of two lowercase alphabetic or numeric characters, or an
space (U+0020) character. The *indicator matcher* checks whether a data
field matches the specified values. It always appears in combination with
a :ref:`tag matcher <ref-tag-matcher>` and is preceded by the prefix ``/``.

The indicator matcher distinguishes between three types:

* explicit matching (``/10``, ``/#1``),
* pattern based matching (``/1[23]``, ``/1[2-5]``),
* wildcard matching (``/*``)

.. note::
	If no indicator matcher is specified, only the fields that contain a space in
	both positions are taken into account.


The simplest form is to explicitly specify the two indicator positions,
preceded by the prefix ``/``. A space is not a valid value and must be
replaced with ``#``.

	>>> from marc21_learn.io import read_marc21
	>>>
	>>> filename = "../tests/data/ada.mrc.gz"
	>>> df = read_marc21(filename, "001", predicate="400/1#?")
	>>> assert df.height == 1
	>>>
	>>> df = read_marc21(filename, "001", predicate="400/11?")
	>>> assert df.height == 0

If you need to specify more than one specific value for an indicator, the
pattern-based approach can be helpful. In this case, a position can be either
an explicit value, a class of values (``[1-3]``), or any possible value
(``.``). The elements of a class are specified by listing the allowed values
enclosed in square brackets (e.g., ``[136]``). A class can also contain one
or more ranges, such as ``[13-56-9]``. Negation is also supported by prefixing
the class with ``^`` (``[^234]``); this class matches all positions that are
not ``2``, ``3``, or ``4``.

Examples:

	>>> from marc21_learn.io import read_marc21
	>>>
	>>> filename = "../tests/data/minna.mrc"
	>>>
	>>> df = read_marc21(filename, "001", predicate="083/0.?")
	>>> assert df.height == 1
	>>>
	>>> df = read_marc21(filename, "001", predicate="083/0[34]?")
	>>> assert df.height == 1
	>>>
	>>> df = read_marc21(filename, "001", predicate="083/0[3-5]?")
	>>> assert df.height == 1
	>>>
	>>> df = read_marc21(filename, "001", predicate="083/0[^5-9#]?")
	>>> assert df.height == 1
	>>>
	>>> df = read_marc21(filename, "001", predicate="083/[30][34]?")
	>>> assert df.height == 1
	>>>
	>>> df = read_marc21(filename, "001", predicate="100/[0-9#][0-9#]?")
	>>> assert df.height == 1
	>>>
	>>> df = read_marc21(filename, "001", predicate="100/..?")
	>>> assert df.height == 1

If you want to accept all possible indicators associated with a data field,
you could use the wildcard expression ``/*``:

	>>> from marc21_learn.io import read_marc21
	>>>
	>>> filename = "../tests/data/minna.mrc"
	>>>
	>>> df = read_marc21(filename, "001", predicate="100/*?")
	>>> assert df.height == 1
