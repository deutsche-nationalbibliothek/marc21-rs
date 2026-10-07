:html_theme.sidebar_secondary.remove:

.. _ref-data-field-matcher:
 
Data Field Matcher
==================

A *data field matcher* consists of a :ref:`tag-matcher <ref-tag-matcher>`,
an optional :ref:`indicator matcher <ref-indicator-matcher>`, and
a :ref:`subfield matcher <ref-subfield-matcher>`, which makes statements
about the subfields of a data field. The :ref:`subfield matcher
<ref-subfield-matcher>` can occur either in long form (``{}`` notation) or,
for simple expressions, in short form (``.`` notation).

.. _ref-subfield-matcher:

.. TODO: link to query/path docs

A *subfield matcher* is an expression that is applied to a list of subfields
(of a data field) and checks whether that list meets the specified criteria.
The matcher is primarily used as part of a :ref:`data field predicate
<ref-data-field-matcher>` and as a constraint in a *query* or *path* expressions.

The following elementary matcher variants are distinguished:

* :ref:`Exists <ref-subfield-exists-matcher>` ``?`` --- checks whether a specific subfield is present
* :ref:`Count <ref-subfield-count-matcher>` ``#`` --- checks the number of occurrences of a subfield
* :ref:`Comparison <ref-subfield-comparison-matcher>` ``==``, ``!=``, ... --- compares the value of a subfield against a reference value
* :ref:`Substring <ref-subfield-substr-matcher>` ``=?``, ``!?`` --- checks whether a subfield contains a specific phrase
* :ref:`Member <ref-subfield-member-matcher>` ``in``, ``not in`` --- checks whether the value of a subfield comes from a reference list
* :ref:`Prefix <ref-subfield-prefix-matcher>` ``=^``, ``!^`` --- checks whether the value of a subfield begins with a prefix
* :ref:`Suffix <ref-subfield-suffix-matcher>` ``=$``, ``!$`` --- checks whether the value of a subfield ends with a suffix
* :ref:`Similarity <ref-subfield-strsim-matcher>` ``=*``, ``!*`` --- checks whether the value of a subfield is similar to a reference
* :ref:`Regex <ref-subfield-regex-matcher>` ``=~``, ``!~`` --- checks whether the value of a subfield matches a regular expression

These elementary subfield matcher can be combined into more complex statements
using the :ref:`Boolean connectives <ref-subfield-boolean-connectives>`
conjunction (``&&``) and disjunction (``||``), as well as grouping
(``(...)``).


.. _ref-subfield-exists-matcher:

Exists Matcher
--------------

The *exists matcher* ``?`` is used to check whether a data field has one
or more subfields. The specific value of the subfield is irrelevant in this
context. In the following example, the field `079` is checked to see if a
subfield named ``u`` exists:

	>>> from marc21_learn.io import read_marc21
	>>>
	>>> filename = "../tests/data/ada.mrc"
	>>>
	>>> df = read_marc21(filename, "001", predicate="079.a?")
	>>> assert df.height == 1

By *negating* the expression, you can check whether a field does *not* contain
a specific subfield:

	>>> from marc21_learn.io import read_marc21
	>>>
	>>> filename = "../tests/data/ada.mrc"
	>>>
	>>> df = read_marc21(filename, "001", predicate="079{ !u? }")
	>>> assert df.height == 0
	>>>
	>>> df = read_marc21(filename, "001", predicate="!079.x?")
	>>> assert df.height == 1

By specifying a *code class*, you can check whether one of the specified
subfields is present. The following example checks whether the field ``079``
contains either the subfield ``p`` or ``q``:

	>>> from marc21_learn.io import read_marc21
	>>>
	>>> filename = "../tests/data/ada.mrc"
	>>>
	>>> df = read_marc21(filename, "001", predicate="079{ [pq]? }")
	>>> assert df.height == 1
	>>>
	>>> df = read_marc21(filename, "001", predicate="079.[pq]?")
	>>> assert df.height == 1


.. _ref-subfield-count-matcher:

Count Matcher
-------------

The *count matcher* ``#`` counts the number of occurrences of one or more
subfields and compares that number against a reference value. The available
comparison operators are ``==``, ``!=``, ``>=``, ``>``, ``<=``, and ``<``.

.. note::
	The count matcher can only be used in the long form of a data field
	matcher.

Example:

	>>> from marc21_learn.io import read_marc21
	>>>
	>>> filename = "../tests/data/ada.mrc"
	>>>
	>>> df = read_marc21(filename, "001", predicate="079{ #u > 2 }")
	>>> assert df.height == 1
	>>>
	>>> df = read_marc21(filename, "001", predicate="079{ #[aq-w] >= 7 }")
	>>> assert df.height == 1


.. _ref-subfield-comparison-matcher:

Comparison Matcher
------------------

The *comparison matcher* compares the value of a subfield against a reference
value. The available comparison operators are ``==``, ``!=``, ``=``, ``>``,
``<=``, and ``<``.

	>>> from marc21_learn.io import read_marc21
	>>>
	>>> filename = "../tests/data/ada.mrc"
	>>>
	>>> df = read_marc21(filename, "001", predicate="075{ b == 'piz' }")
	>>> assert df.height == 1
	>>>
	>>> df = read_marc21(filename, "001", predicate="075.b != 'piz' ")
	>>> assert df.height == 1

Optionally, the statement can be quantified using the universal quantifier
``ALL`` or the existential quantifier ``ANY``. By default, the existential
quantifier is used; that is, the existence of at least one subfield that
corresponds to the reference value according to the operator is sufficient for
the statement to be true.

.. note::
	Quantified expressions can't be used in the short form of a data field
	matcher.

Examples:

	>>> from marc21_learn.io import read_marc21
	>>>
	>>> filename = "../tests/data/ada.mrc"
	>>>
	>>> df = read_marc21(filename, "001", predicate="079{ ALL u >= 'k' }")
	>>> assert df.height == 1
	>>>
	>>> df = read_marc21(filename, "001", predicate="079{ ANY u != 'k' }")
	>>> assert df.height == 1


.. _ref-subfield-substr-matcher:

Substring Matcher
-----------------

The *substring matcher* ``=?`` checks whether the specified substring is
contained in the specified subfield value. Internally, the matcher uses the
`Aho–Corasick algorithm`_ to enable efficient substring searches.

.. _Aho–Corasick algorithm: https://en.wikipedia.org/wiki/Aho%E2%80%93Corasick_algorithm

	>>> from marc21_learn.io import read_marc21
	>>>
	>>> filename = "../tests/data/ada.mrc"
	>>>
	>>> df = read_marc21(filename, "001", predicate="400/1#{ a =? 'Augusta' }")
	>>> assert df.height == 1
	>>>
	>>> df = read_marc21(filename, "001", predicate="400/1#.a =? 'Augusta'")
	>>> assert df.height == 1


The matcher also allows you to search for multiple substrings at once by
specifying a list of substrings. The expression evaluates to true if at least
one of the specified phrases appears in the specified subfield.

	>>> from marc21_learn.io import read_marc21
	>>>
	>>> filename = "../tests/data/ada.mrc"
	>>>
	>>> df = read_marc21(filename, "001", predicate="400/1#.a =? ['Hate', 'Love']")
	>>> assert df.height == 1

To check whether a substring (or a list of substrings) is *not* contained, use
the ``!?`` operator:

	>>> from marc21_learn.io import read_marc21
	>>>
	>>> filename = "../tests/data/ada.mrc"
	>>>
	>>> df = read_marc21(filename, "001", predicate="400/1#.a !? 'Curie'")
	>>> assert df.height == 1

Finally, the statements can be quantified using the universal quantifier
``ALL`` or the existential quantifier ``ANY``.

.. note::
	A quantifier can't be used in the short form of a data field matcher.

Example:

	>>> from marc21_learn.io import read_marc21
	>>>
	>>> filename = "../tests/data/ada.mrc"
	>>>
	>>> df = read_marc21(filename, "001", predicate="035{ ALL [az] =? 'DE-' }")
	>>> assert df.height == 1


.. _ref-subfield-member-matcher:

Member Matcher
--------------

The *member matcher* ``in`` / ``not in`` checks whether the value of a
subfield comes from a reference list. The values are specified as a non-empty,
comma-separated list enclosed in square brackets. If you want to check whether
the value of a field *does not* come from a list, use the ``not in`` operator
instead of ``in``.

	>>> from marc21_learn.io import read_marc21
	>>>
	>>> filename = "../tests/data/ada.mrc"
	>>>
	>>> df = read_marc21(filename, "001", predicate="075{ b in ['p', 's', 'u'] }")
	>>> assert df.height == 1
	>>>
	>>> df = read_marc21(filename, "001", predicate="075.b in ['p', 's', 'u']")
	>>> assert df.height == 1
	>>>
	>>> df = read_marc21(filename, "001", predicate="075.b not in ['b', 'f', 'g']")
	>>> assert df.height == 1

The statements can be quantified using the universal quantifier ``ALL`` or the
existential quantifier ``ANY``. A quantifier can't be used in the short form
of a data field matcher.

	>>> from marc21_learn.io import read_marc21
	>>>
	>>> filename = "../tests/data/ada.mrc"
	>>>
	>>> df = read_marc21(filename, "001", predicate="079{ ALL u in ['w', 'k', 'v'] }")
	>>> assert df.height == 1


.. _ref-subfield-prefix-matcher:

Prefix Matcher
--------------

The *prefix matcher* ``=?`` checks whether the value of a subfield begins
with a prefix. If the matcher is to search for multiple possible prefixes,
the values are specified as a comma-separated list. To check whether the value
does *not* begin with a prefix, the ``!^`` operator is used.

	>>> from marc21_learn.io import read_marc21
	>>>
	>>> filename = "../tests/data/ada.mrc"
	>>>
	>>> df = read_marc21(filename, "001", predicate="400/1#{ a =^ 'Love' }")
	>>> assert df.height == 1
	>>>
	>>> df = read_marc21(filename, "001", predicate="400/1#.a =^ 'Lovelace'")
	>>> assert df.height == 1
	>>>
	>>> df = read_marc21(filename, "001", predicate="400/1#.a =^ ['Love', 'Hate']")
	>>> assert df.height == 1
	>>>
	>>> df = read_marc21(filename, "001", predicate="400/1#.a !^ 'Hate'")
	>>> assert df.height == 1

The statements can be quantified using the universal quantifier ``ALL`` or the
existential quantifier ``ANY``. 

.. note::
	A quantifier can't be used in the short form of a data field matcher.

Example:

	>>> from marc21_learn.io import read_marc21
	>>>
	>>> filename = "../tests/data/ada.mrc"
	>>>
	>>> df = read_marc21(filename, "001", predicate="400/1#{ ALL d =^ '1815' }")
	>>> assert df.height == 1


.. _ref-subfield-suffix-matcher:

Suffix Matcher
--------------

The *suffix matcher* ``=$`` checks whether the value of a subfield ends with a
suffix. If the matcher is to search for multiple possible suffixes, the values
are specified as a comma-separated list. To check whether the value does *not*
end with a suffix, the ``!$`` operator is used.

	>>> from marc21_learn.io import read_marc21
	>>>
	>>> filename = "../tests/data/ada.mrc"
	>>>
	>>> df = read_marc21(filename, "001", predicate="400/1#{ a =$ 'Ada' }")
	>>> assert df.height == 1
	>>>
	>>> df = read_marc21(filename, "001", predicate="400/1#.a =$ 'Ada'")
	>>> assert df.height == 1
	>>>
	>>> df = read_marc21(filename, "001", predicate="400/1#.a =$ ['Ada', 'Bob']")
	>>> assert df.height == 1
	>>>
	>>> df = read_marc21(filename, "001", predicate="400/1#.a !$ 'Ada'")
	>>> assert df.height == 1


The statements can be quantified using the universal quantifier ``ALL`` or the
existential quantifier ``ANY``.

.. note::
	A quantifier can't be used in the short form of a data field matcher.

Example:

	>>> from marc21_learn.io import read_marc21
	>>>
	>>> filename = "../tests/data/ada.mrc"
	>>>
	>>> df = read_marc21(filename, "001", predicate="400/1#{ ALL d =$ '1852' }")
	>>> assert df.height == 1


.. _ref-subfield-strsim-matcher:

Similarity Matcher
------------------

The *similarity matcher* ``=*`` checks whether the value of a subfield
is similar to a reference value. Similarity is determined by calculating
the normalized `Levenshtein distance`_ between the subfield value and the
reference value. The values range from 0.0 to 1.0 (inclusive), where a value
of 1.0 indicates that the two values match. A match is considered to exist
if the similarity value is greater than or equal to the threshold value. The
threshold is set to 80% (≙ 0.8) by default. To check for non-similarity, the
``!*`` operator is used.

.. _Levenshtein distance: https://en.wikipedia.org/wiki/Levenshtein_distance

.. note::
	The threshold can't be set yet.

Examples:

	>>> from marc21_learn.io import read_marc21
	>>>
	>>> filename = "../tests/data/ada.mrc"
	>>>
	>>> df = read_marc21(filename, "001", predicate="400/1#{ a =* 'Kong, Ada' }")
	>>> assert df.height == 1
	>>>
	>>> df = read_marc21(filename, "001", predicate="400/1#.a =* 'Kong, Ada'")
	>>> assert df.height == 1
	>>>
	>>> df = read_marc21(filename, "001", predicate="400/1#{ a =* 'Hateless, Ada' }")
	>>> assert df.height == 0
	>>>
	>>> df = read_marc21(filename, "001", predicate="400/1#{ a !* 'Hateless, Ada' }")
	>>> assert df.height == 1

The statements can be quantified using the universal quantifier ``ALL`` or the
existential quantifier ``ANY``. 

.. note::
	A quantifier can't be used in the short form of a data field matcher.


.. _ref-subfield-regex-matcher:

Regex Matcher
-------------

The *regex matcher* ``=~`` checks whether the value of a subfield matches a
regular expression. To check multiple patterns at once, list all patterns as
a comma-separated list. To check whether the value *does not* match a regular
expression, use the ``!~`` operator.

.. note::
	Not all regex functions are supported. Please consult the `syntax
	documentation`_ of the regex library used in this project if you have
	any questions.

	Also keep in mind that, depending on the context and the type of
	quotes used (single or double), special characters in the regular
	expression may need to be quoted.

.. _syntax documentation: https://docs.rs/regex/latest/regex/#syntax

.. hint::
	The `Rustexp`_ website offers a regular expression editor and tester.

.. _Rustexp: https://rustexp.lpil.uk/

Examples:

	>>> from marc21_learn.io import read_marc21
	>>>
	>>> filename = "../tests/data/ada.mrc"
	>>>
	>>> df = read_marc21(filename, "001", predicate=r"400/1#{ d =~ '^\\d{4}-\\d{4}$' }")
	>>> assert df.height == 1
	>>>
	>>> df = read_marc21(filename, "001", predicate=r"400/1#.d =~ '^\\d{4}-\\d{4}'")
	>>> assert df.height == 1
	>>>
	>>> df = read_marc21(filename, "001", predicate=r"075.b =~ ['^[bfg]$', '^piz$']")
	>>> assert df.height == 1
	>>>
	>>> df = read_marc21(filename, "001", predicate=r"ALL 075.2 =~ '^gnd(gen|spec)$'")
	>>> assert df.height == 1
	>>>
	>>> df = read_marc21(filename, "001", predicate=r"075.b !~ '^[bfg]$'")
	>>> assert df.height == 1

The statements can be quantified using the universal quantifier  ``ALL`` or
the existential quantifier ``ANY``.

.. note::
	A quantifier can't be used in the short form of a data field matcher.

Example:

	>>> from marc21_learn.io import read_marc21
	>>>
	>>> filename = "../tests/data/ada.mrc"
	>>>
	>>> df = read_marc21(filename, "001", predicate=r"079{ ALL u =~ '^[a-z]$' }")
	>>> assert df.height == 1


.. _ref-subfield-boolean-connectives:

Boolean Connectives
-------------------

More complex statements can be formed using the two Boolean connectives
conjunction ``&&`` and disjunction ``||``. Since the connected sub-statements
always refer to the same field, Boolean connectives can only be used in the
long form of a field matcher.

In the following example, the record must contain a field ``075`` for  which
the following is true: There exists a subfield ``b`` with the value ``p``
**and**, within the same field, there exists a subfield ``2`` with the value
``gndgen``.

	>>> from marc21_learn.io import read_marc21
	>>>
	>>> filename = "../tests/data/ada.mrc"
	>>>
	>>> predicate = "075{ b == 'p' && 2 == 'gndgen' }"
	>>> df = read_marc21(filename, "001", predicate=predicate)
	>>> assert df.height == 1
	>>>
	>>> predicate = "079{ q == 'f' || q == 's' }"
	>>> df = read_marc21(filename, "001", predicate=predicate)
	>>> assert df.height == 1
	>>>
	>>> predicate = "075{ b == 'p' && 2 == 'gndspec' }"
	>>> df = read_marc21(filename, "001", predicate=predicate)
	>>> assert df.height == 0


The `&&` operator has higher precedence than the `||` operator, which means
that the expression `A || B && C` is equivalent to `A || (B && C)`. It follows
that parentheses may be necessary:

	>>> from marc21_learn.io import read_marc21
	>>>
	>>> filename = "../tests/data/ada.mrc"
	>>>
	>>> predicate = "075{ \
	...        (b == 'piz' && 2 == 'gndspec') \
	...     || (b == 'p' && 2 == 'gndgen') \
	... }"
	>>>
	>>> df = read_marc21(filename, "001", predicate=predicate)
	>>> assert df.height == 1
