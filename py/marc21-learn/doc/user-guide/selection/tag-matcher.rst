:html_theme.sidebar_secondary.remove:

.. _ref-tag-matcher:

Tag Matcher
===========

In MARC21, variable fields are uniquely identified by tags. A *tag matcher* is
an expression used to filter those fields that match a specific tag.

In its simplest form, only the three numerical digits of a tag are specified.
A match with a tag only exists if these digits exactly match:

	>>> from marc21_learn.io import read_marc21
	>>>
	>>> filename = "../tests/data/ada.mrc.gz"
	>>> df = read_marc21(filename, "001", predicate="001?")
	>>> assert df.height == 1

In order to identify more than one field, a pattern-based comparison must
be performed. Each numerical digit of a tag can be specified by one of the
following variants.

First, a digit can be represented by the wildcard character `.` that accepts
all possible values from `0` to `9`. For example, the following tag matcher
``0.8`` accepts all fields that that begins with ``0`` and ends with ``8``.
The middle position can contain any digit.

	>>> from marc21_learn.io import read_marc21
	>>>
	>>> filename = "../tests/data/ada.mrc.gz"
	>>> df = read_marc21(filename, "001", predicate="0.8?")
	>>> assert df.height == 1

Next, a digit can also be represented by specifying a class of possible
digits. In the following example ``0[235]5``, all fields that start with a
``0``, have either a ``2``, ``3``, or ``5`` in the second position, and end
with a ``5`` are accepted.

	>>> from marc21_learn.io import read_marc21
	>>>
	>>> filename = "../tests/data/ada.mrc.gz"
	>>> df = read_marc21(filename, "001", predicate="0[235]5?")
	>>> assert df.height == 1

Similar to the character classes of a regular expression, several consecutive
digits within a class can be combined into a *range*. The range is inclusive,
and the upper interval limit must be greater than the lower limit. Note that a
class can consist of more than one range expression.

	>>> from marc21_learn.io import read_marc21
	>>>
	>>> filename = "../tests/data/ada.mrc.gz"
	>>> df = read_marc21(filename, "001", predicate="0[2-5]5?")
	>>> assert df.height == 1

A class can also be specified in *negated form* (``^``). In this case, the
matcher checks that the digit in the corresponding position of the tag does
not originate from the class digits:

	>>> from marc21_learn.io import read_marc21
	>>>
	>>> filename = "../tests/data/ada.mrc.gz"
	>>> df = read_marc21(filename, "001", predicate="04[^0-3]?")
	>>> assert df.height == 0

After all, every options can be used in all positions of a tag matcher. In
the most extreme case, the expressions ``...`` and ``[0-9][0-9][0-9]`` accept
every field, while the expression ``[^0-9][^0-9][^0-9]`` accepts no fields.
In the following example, all fields that begin with any digit not followed
by a ``0`` and end with a ``1``, ``2`` ``3``, ``5``, or ``6`` are taken into
account.

	>>> from marc21_learn.io import read_marc21
	>>>
	>>> filename = "../tests/data/ada.mrc.gz"
	>>> df = read_marc21(filename, "001", predicate=".[^0][1-356]?")
	>>> assert df.height == 1
