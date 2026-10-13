:html_theme.sidebar_secondary.remove:

.. _ref-control-field-matcher:

Control Field Matcher
=====================

A *control field matcher* consists of four components:

- a :ref:`tag matcher <ref-tag-matcher>` to select the fields
- an optional *range* to check only substrings
- an comparison operator (``==``, ``!=``, ``>=``, ``>``, ``<=``, ``<``) or ``in`` / ``not in`` operator,
- a value (comparison operator) or a list of values (``in`` / ``not in`` operator)

In the simplest case, control fields are compared by addressing the fields
using a :ref:`tag matcher <ref-tag-matcher>`, applying a comparison operator, and
specifying a comparison value.

	>>> from marc21_learn.io import read_marc21
	>>>
	>>> filename = "../tests/data/DUMP.mrc.gz"
	>>>
	>>> df = read_marc21(filename, "001", predicate="001 == '118540238'")
	>>> assert df.height == 1
	>>>
	>>> df = read_marc21(filename, "001", predicate="003 != 'DE-101'")
	>>> assert df.height == 0

Some control fields contain fixed-length data elements. To access individual
elements, you can optionally specify a *range* that defines the start
(inclusive) and end (exclusive). If the value of the field contains this
substring, it is compared to the reference value using the specified operator.

In the following example, all authority records are filtered where the date of
last transaction (field ``005``, first 8 characters) is earlier than January
1, 2025:

	>>> from marc21_learn.io import read_marc21
	>>>
	>>> filename = "../tests/data/DUMP.mrc.gz"
	>>>
	>>> df = read_marc21(filename, "001", predicate="005[0:8] <= '20250101'")
	>>> assert df.height == 1

.. note::
	If the start value is omitted (e.g., ``004[:4]``), the start is set to
	``0``. If the end value is omitted (e.g., `003[3:]`), the end is 
	automatically set to the length of the corresponding value. If the end value
	exceeds the length of the string, the expression evaluates to false.

The ``in`` operator can be used to check whether the value of a control field
comes from a reference list, or not (``not in``):

	>>> from marc21_learn.io import read_marc21
	>>>
	>>> filename = "../tests/data/DUMP.mrc.gz"
	>>>
	>>> df = read_marc21(filename, "001", predicate="003 in ['DE-101', 'DE-1979']")
	>>> assert df.height == 7
	>>>
	>>> df = read_marc21(filename, "001", predicate="005[:4] not in ['2025', '2026']")
	>>> assert df.height == 1

