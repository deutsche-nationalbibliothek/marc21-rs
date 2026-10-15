:html_theme.sidebar_secondary.remove:

.. _ref-count-matcher:
 
Count Matcher
=============

The *count matcher* can be used to determine the count of one or more
fields and compare it against a reference value. The expression begins with
a ``#`` character and consists of a :ref:`tag matcher <ref-tag-matcher>`
an optional :ref:`indicator matcher <ref-indicator-matcher>`, an optional
*subfield matcher* using the ``{}``- notation, a comparison operator (``==``,
``!=``, ``>=``, ``>``, ``<=``, or ``<``), and the comparison value (unsigned
integer).

In its simplest form, all fields that match the specified tag matcher* and, if
*applicable, an *indicator matcher* are counted:

	>>> from marc21_learn.io import read_marc21
	>>>
	>>> filename = "../tests/data/ada.mrc"
	>>>
	>>> df = read_marc21(filename, "001", predicate="#035 > 5")
	>>> assert df.height == 1
	>>>
	>>> df = read_marc21(filename, "001", predicate="#500/1# == 3")
	>>> assert df.height == 1
	>>>
	>>> df = read_marc21(filename, "001", predicate="#400/* < 20")
	>>> assert df.height == 1
	>>>
	>>> df = read_marc21(filename, "001", predicate="#099 == 0")
	>>> assert df.height == 1

In the following example, the number of fields ``400`` is counted using the
indicators ``1`` and ``#``, for which a subfield ``a`` exists and a subfield
``4`` is set to the value ``nafr``. The expression evaluates to true if this
number is equal to ``2``.

	>>> from marc21_learn.io import read_marc21
	>>>
	>>> filename = "../tests/data/ada.mrc"
	>>> predicate = "#400/1#{ a? && 4 == 'nafr' } == 2"
	>>>
	>>> df = read_marc21(filename, "001", predicate=predicate)
	>>> assert df.height == 1

