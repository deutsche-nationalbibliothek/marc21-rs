:html_theme.sidebar_secondary.remove:

.. _ref-boolean-connectives:

Boolean Connectives
===================

Both :ref:`leader matcher <ref-leader-matcher>` and :ref:`field matcher
<ref-field-matcher>` can be combined in any order using the Boolean
connectives conjunction ``&&`` and disjunction ``||``. The ``&&`` operator
has higher precedence than the ``||`` operator, which may require parentheses
(``(...)``) to be used around sub-expressions.


	>>> from marc21_learn.io import read_marc21
	>>>
	>>> filename = "../tests/data/ada.mrc"
	>>>
	>>> predicate = "ldr.type == 'z' \
	...     && 005[0:4] == '2025' \
	...     && 075{ b == 'piz' && 2 == 'gndspec' } \
	...     && 042.a == 'gnd1' \
	...     && 079.q == 's' \
	... "
	>>>
	>>> df = read_marc21(filename, "001", predicate=predicate)
	>>> assert df.height == 1
