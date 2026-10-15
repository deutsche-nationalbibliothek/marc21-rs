.. _ref-projection:

=================
Record Projection
=================

In order to perform meaningful statistical data analytics or to
create :ref:`datasets <ref-datasets>` for machine learning, the nested data
of a MARC21 record must be converted into a rectangular schema (tabular
form). This transformation process is called *projection*. The query language
introduced in this section allows you to specify projections in order
to map selected attributes of a record to a set of relations, whereby all
non-selected attributes are discarded.


Query Language
--------------

A query consists of a non-empty, comma-separated list of expressions, each
describing a projection of a record to a table with :math:`m` rows (height)
and :math:`n` columns (width). The overall result of a query is calculated
as follows: Starting with an empty table, all non-empty projections are
successively folded from left to right by generating the `Cartesian product`_
of the rows of the current projection with the rows of the next projection. A
query with :math:`k` projections generates a table with :math:`\prod_{i = 1}
^{k}{\max\{m_i, 1\}}` rows and :math:`\sum_{i = 1}^{k}{n_i}` columns.

.. _Cartesian product: https://en.wikipedia.org/wiki/Cartesian_product

The toolkit offers the following kinds of projections that can be applied on a
MARC21 record:

* :ref:`Leader Fields <ref-leader-expr>` --- Projections of leader fields
* :ref:`Control Fields <ref-control-field-expr>` --- Projection of control fields
* :ref:`Data Fields <ref-data-field-expr>` --- Projection of data fields
* :ref:`String Literals <ref-literal-expr>` --- Output of a fixed string literal


Projections
^^^^^^^^^^^

.. _ref-leader-expr:

Leader Fields
"""""""""""""

The following fields of the leader can be read:

* ``base_addr`` — Base address of data (position 12-16)
* ``encoding`` — Character coding scheme (position 09)
* ``length`` — Record length (position 00-04)
* ``status`` — Record status (position 05)
* ``type`` — Type of record (position 06)

In the following example, the leader fields listed above from the *Ada
Lovelace* test record are queried:

	>>> from marc21_learn.io import read_marc21
	>>>
	>>> query = "ldr.length, \
	...     ldr.base_addr, \
	...     ldr.status, \
	...     ldr.encoding, \
	...     ldr.type \
	... "
	>>>
	>>> df = read_marc21("../tests/data/ada.mrc", query)
	>>> df
	shape: (1, 5)
	┌──────────┬──────────┬──────────┬──────────┬──────────┐
	│ column_1 ┆ column_2 ┆ column_3 ┆ column_4 ┆ column_5 │
	│ ---      ┆ ---      ┆ ---      ┆ ---      ┆ ---      │
	│ u32      ┆ u32      ┆ str      ┆ str      ┆ str      │
	╞══════════╪══════════╪══════════╪══════════╪══════════╡
	│ 3612     ┆ 589      ┆ n        ┆ a        ┆ z        │
	└──────────┴──────────┴──────────┴──────────┴──────────┘

Note that a leader projection always generates a table with one column and
one row.

By default, the columns of a query are numbered from ``column_1`` to
``column_n``. The columns can be reamed by using an ``AS`` clause:

	>>> from marc21_learn.io import read_marc21
	>>>
	>>> query = "ldr.length AS `length`, ldr.base_addr AS `base_addr`"
	>>>
	>>> df = read_marc21("../tests/data/ada.mrc", query)
	>>> df
	shape: (1, 2)
	┌────────┬───────────┐
	│ length ┆ base_addr │
	│ ---    ┆ ---       │
	│ u32    ┆ u32       │
	╞════════╪═══════════╡
	│ 3612   ┆ 589       │
	└────────┴───────────┘

Optionally, a *prefix* can be added to and/or a *suffix* can be appended to
the individual values:

	>>> from marc21_learn.io import read_marc21
	>>>
	>>> query = "'encoding: ' ldr.encoding AS `encoding`"
	>>> df = read_marc21("../tests/data/ada.mrc", query)
	>>> df
	shape: (1, 1)
	┌─────────────┐
	│ encoding    │
	│ ---         │
	│ str         │
	╞═════════════╡
	│ encoding: a │
	└─────────────┘

	>>> query = "ldr.encoding ' (encoding)' AS `encoding`"
	>>> df = read_marc21("../tests/data/ada.mrc", query)
	>>> df
	shape: (1, 1)
	┌──────────────┐
	│ encoding     │
	│ ---          │
	│ str          │
	╞══════════════╡
	│ a (encoding) │
	└──────────────┘



.. _ref-control-field-expr:

Control Fields
""""""""""""""

Control fields can be queried by specifying the corresponding tag:
 
	>>> from marc21_learn.io import read_marc21
	>>>
	>>> query = "001, 003"
	>>>
	>>> df = read_marc21("../tests/data/ada.mrc", query)
	>>> df
	shape: (1, 2)
	┌───────────┬──────────┐
	│ column_1  ┆ column_2 │
	│ ---       ┆ ---      │
	│ str       ┆ str      │
	╞═══════════╪══════════╡
	│ 119232022 ┆ DE-101   │
	└───────────┴──────────┘

The columns can be reamed by using an ``AS`` clause:

	>>> from marc21_learn.io import read_marc21
	>>>
	>>> query = "001 AS `cn`, 003 AS `isil`"
	>>>
	>>> df = read_marc21("../tests/data/ada.mrc", query)
	>>> df
	shape: (1, 2)
	┌───────────┬────────┐
	│ cn        ┆ isil   │
	│ ---       ┆ ---    │
	│ str       ┆ str    │
	╞═══════════╪════════╡
	│ 119232022 ┆ DE-101 │
	└───────────┴────────┘

The elements of fixed-length fields are accessed by specifying a *range*
``[start:end]`` that defines the start (inclusive) and end (exclusive):


	>>> from marc21_learn.io import read_marc21
	>>>
	>>> query = "001 AS `cn`, 005[0:8] AS `created`"
	>>>
	>>> df = read_marc21("../tests/data/ada.mrc", query)
	>>> df
	shape: (1, 2)
	┌───────────┬──────────┐
	│ cn        ┆ created  │
	│ ---       ┆ ---      │
	│ str       ┆ str      │
	╞═══════════╪══════════╡
	│ 119232022 ┆ 20250720 │
	└───────────┴──────────┘

.. note::
	If the start value is omitted (e.g., ``005[:4]``), the start is set to 	``0``. 	
	If the end value is omitted (e.g., `005[3:]`), the end is automatically set to
	the length of the corresponding value. If the end value exceeds the length 	of
	the string or the end value is less than or equal than start, the projection
	of the expression is an empty string.

		>>> from marc21_learn.io import read_marc21
		>>>
		>>> df = read_marc21("../tests/data/ada.mrc", "001, 005[0:42]")
		>>> df
		shape: (1, 2)
		┌───────────┬──────────┐
		│ column_1  ┆ column_2 │
		│ ---       ┆ ---      │
		│ str       ┆ str      │
		╞═══════════╪══════════╡
		│ 119232022 ┆          │
		└───────────┴──────────┘

And finally, a prefix or suffix can be added to an expression:


	>>> from marc21_learn.io import read_marc21
	>>>
	>>> query = query = "\
	...     'https://d-nb.info/' 001 AS `url`, \
	...     001 ' (cn)' AS `identifier`, \
	...     005[0:8] AS `created` \
	... "
	>>>
	>>> df = read_marc21("../tests/data/ada.mrc", query)
	>>> df
	shape: (1, 3)
	┌─────────────────────────────┬────────────────┬──────────┐
	│ url                         ┆ identifier     ┆ created  │
	│ ---                         ┆ ---            ┆ ---      │
	│ str                         ┆ str            ┆ str      │
	╞═════════════════════════════╪════════════════╪══════════╡
	│ https://d-nb.info/119232022 ┆ 119232022 (cn) ┆ 20250720 │
	└─────────────────────────────┴────────────────┴──────────┘

.. _ref-data-field-expr:

Data Fields
""""""""""""

Projecting data fields is somewhat more complicated, since in addition to
selecting the fields, you must also specify the desired subfields. In the
following examples, the first column contains the control number from control
field 001 to illustrate how multiple rows can be generated per record/data
field.

In the simplest case, a non-repeated data field and a single, non-repeated
subfield are queried by specifying the tag, the indicators (optional), and the
subfield code:

	>>> from marc21_learn.io import read_marc21
	>>>
	>>> query = query = "001 AS `cn`, 100/1#.a AS `name`"
	>>> df = read_marc21("../tests/data/ada.mrc", query)
	>>> df
	shape: (1, 2)
	┌───────────┬───────────────┐
	│ cn        ┆ name          │
	│ ---       ┆ ---           │
	│ str       ┆ str           │
	╞═══════════╪═══════════════╡
	│ 119232022 ┆ Lovelace, Ada │
	└───────────┴───────────────┘

If the data field appears multiple times but the subfield is not repeated, a
new row is created for each value and combined with the first column (``cn``):

	>>> from marc21_learn.io import read_marc21
	>>>
	>>> query = query = "001 AS `cn`, 075.b AS `code`"
	>>> df = read_marc21("../tests/data/ada.mrc", query)
	>>> df
	shape: (2, 2)
	┌───────────┬──────┐
	│ cn        ┆ code │
	│ ---       ┆ ---  │
	│ str       ┆ str  │
	╞═══════════╪══════╡
	│ 119232022 ┆ p    │
	│ 119232022 ┆ piz  │
	└───────────┴──────┘

The sequential combination of the rows from the partial projections leads
to undesirable effects when querying multiple subfields. This is because
subfields that do not belong together are then combined, as illustrated in the
following example:

	>>> from marc21_learn.io import read_marc21
	>>>
	>>> query = "001 AS `cn`, 075.b AS `code`, 075.2 AS `source`"
	>>> df = read_marc21("../tests/data/ada.mrc", query)
	>>> df
	shape: (4, 3)
	┌───────────┬──────┬─────────┐
	│ cn        ┆ code ┆ source  │
	│ ---       ┆ ---  ┆ ---     │
	│ str       ┆ str  ┆ str     │
	╞═══════════╪══════╪═════════╡
	│ 119232022 ┆ p    ┆ gndgen  │
	│ 119232022 ┆ p    ┆ gndspec │
	│ 119232022 ┆ piz  ┆ gndgen  │
	│ 119232022 ┆ piz  ┆ gndspec │
	└───────────┴──────┴─────────┘

To keep subfields that belong together grouped, you must use the ``{}``
notation:

	>>> from marc21_learn.io import read_marc21
	>>>
	>>> query = "001 AS `cn`, 075{ b AS `code`, 2 AS `source` }"
	>>> df = read_marc21("../tests/data/ada.mrc", query)
	>>> df
	shape: (2, 3)
	┌───────────┬──────┬─────────┐
	│ cn        ┆ code ┆ source  │
	│ ---       ┆ ---  ┆ ---     │
	│ str       ┆ str  ┆ str     │
	╞═══════════╪══════╪═════════╡
	│ 119232022 ┆ p    ┆ gndgen  │
	│ 119232022 ┆ piz  ┆ gndspec │
	└───────────┴──────┴─────────┘

If not all fields are to be included in the result, fields can be filtered in
advance using a :ref:`subfield matcher <ref-subfield-matcher>`. Conditions are
always specified using ``{}`` notation following a ``|`` symbol:

	>>> from marc21_learn.io import read_marc21
	>>>
	>>> query = "001 AS `cn`, 075{ b AS `code` | 2 == 'gndspec' }"
	>>> df = read_marc21("../tests/data/ada.mrc", query)
	>>> df
	shape: (1, 2)
	┌───────────┬──────┐
	│ cn        ┆ code │
	│ ---       ┆ ---  │
	│ str       ┆ str  │
	╞═══════════╪══════╡
	│ 119232022 ┆ piz  │
	└───────────┴──────┘

The values of different subfields can be merged as follows:

	>>> from marc21_learn.io import read_marc21
	>>>
	>>> query = "001 AS `cn`, 079.[aqu] AS `code`"
	>>> df = read_marc21("../tests/data/ada.mrc", query)
	>>> df
	shape: (7, 2)
	┌───────────┬──────┐
	│ cn        ┆ code │
	│ ---       ┆ ---  │
	│ str       ┆ str  │
	╞═══════════╪══════╡
	│ 119232022 ┆ g    │
	│ 119232022 ┆ f    │
	│ 119232022 ┆ s    │
	│ 119232022 ┆ z    │
	│ 119232022 ┆ w    │
	│ 119232022 ┆ k    │
	│ 119232022 ┆ v    │
	└───────────┴──────┘

Within ``{}`` notation, columns can be disabled using ``_``:


	>>> from marc21_learn.io import read_marc21
	>>>
	>>> query = "001, 075{ b, _ }"
	>>> df = read_marc21("../tests/data/ada.mrc", query)
	>>> df
	shape: (2, 2)
	┌───────────┬──────────┐
	│ column_1  ┆ column_2 │
	│ ---       ┆ ---      │
	│ str       ┆ str      │
	╞═══════════╪══════════╡
	│ 119232022 ┆ p        │
	│ 119232022 ┆ piz      │
	└───────────┴──────────┘
	
.. _ref-literal-expr:

String Literals
"""""""""""""""

Sometimes it is helpful to populate a column with a constant string, for
example, to store specific contextual information. This is done by specifying
a string literal:

	>>> from marc21_learn.io import read_marc21
	>>>
	>>> query = "001 AS `cn`, 'foo' AS `info`"
	>>> df = read_marc21("../tests/data/ada.mrc", query)
	>>> df
	shape: (1, 2)
	┌───────────┬──────┐
	│ cn        ┆ info │
	│ ---       ┆ ---  │
	│ str       ┆ str  │
	╞═══════════╪══════╡
	│ 119232022 ┆ foo  │
	└───────────┴──────┘

String literals can also be used in the ``{}`` notation:

	>>> from marc21_learn.io import read_marc21
	>>>
	>>> query = "\
	...     001 AS `cn`, \
	...     075{ b AS `notation`, 'gnd-sc' AS `vocabulary` | 2 == 'gndspec' } \
	... "
	>>>
	>>> df = read_marc21("../tests/data/ada.mrc", query)
	>>> df
	shape: (1, 3)
	┌───────────┬──────────┬────────────┐
	│ cn        ┆ notation ┆ vocabulary │
	│ ---       ┆ ---      ┆ ---        │
	│ str       ┆ str      ┆ str        │
	╞═══════════╪══════════╪════════════╡
	│ 119232022 ┆ piz      ┆ gnd-sc     │
	└───────────┴──────────┴────────────┘

Note that string literals always generates a table with one column and one
row.

Grammar
^^^^^^^

*tba*

.. .. productionlist::
.. 	query: WS+ constituent { "," WS+ constituent } WS+ ;
..     constituent: control-field-expr
.. 	           : | data-field-expr
.. 			   : | leader-expr 
.. 			   : | literal ;


.. _ref-path-expr:

Path Expressions
----------------

A query that consists of exactly one projection **and** returns either no
columns (an empty query) or only one column is also referred to as a *path*.
The result of a path projection is always a linear list of values, not a
table. Path expressions are used, for example, to describe the features of
a :ref:`dataset <ref-datasets>`.

Examples
--------

The German National Library (DNB) uses a subset of the Gemeinsame Normdatei
(GND) to describe the specific characteristics of a work's primary content
(RDA 7.2 *nature of content*). This subset can be filtered and tabulated as
follows:

	>>> from marc21_learn.io import read_marc21
	>>>
	>>> filename = "authorities-gnd-sachbegriff_dnbmarc.mrc.gz"
	>>> query = "001 AS `ppn`, 024/7#.a AS `nid`, 150.a AS `label`"
	>>> predicate = "ldr.type == 'z' && 042.a in ['gnd1', 'gndz'] \
	...     && 075{ b == 'saf' && 2 == 'gndspec' }"
	>>>
	>>> df = read_marc21(filename, query, predicate=predicate) # doctest: +SKIP
	>>> df.sort("label") # doctest: +SKIP
	shape: (238, 3)
	┌───────────┬───────────┬──────────────────────┐
	│ ppn       ┆ nid       ┆ label                │
	│ ---       ┆ ---       ┆ ---                  │
	│ str       ┆ str       ┆ str                  │
	╞═══════════╪═══════════╪══════════════════════╡
	│ 041414519 ┆ 4141451-2 ┆ Adressbuch           │
	│ 960106200 ┆ 4611904-8 ┆ Altkarte             │
	│ 041321197 ┆ 4132119-4 ┆ Amateurfilm          │
	│ 041423003 ┆ 4142300-8 ┆ Amtliche Publikation │
	│ 041424840 ┆ 4142484-0 ┆ Animationsfilm       │
	│ …         ┆ …         ┆ …                    │
	│ 04127900X ┆ 4127900-1 ┆ Zeichnung            │
	│ 040674886 ┆ 4067488-5 ┆ Zeitschrift          │
	│ 041906314 ┆ 4190631-7 ┆ Zeittafel            │
	│ 040675106 ┆ 4067510-5 ┆ Zeitung              │
	│ 04117724X ┆ 4117724-1 ┆ Zitatensammlung      │
	└───────────┴───────────┴──────────────────────┘






