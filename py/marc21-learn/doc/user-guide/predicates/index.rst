.. _ref-predicates:

Predicates
==========

A *predicate* (or *record matcher*) is one of the most important components of
the ``marc21-learn`` toolkit. It allows for the efficient filtering of records
based on various criteria. A predicate expression is either a :ref:`leader
matcher <ref-leader-matcher>` or a :ref:`field matcher <ref-field-matcher>`.
Both can be combined into more complex expressions through Boolean connectives
or grouping.

.. _ref-leader-matcher:

Leader Matcher
--------------

The *leader matcher* allows you to check the elements of the leader. The
following fields can be checked:

* ``length`` — Record length (position 00-04)
* ``status`` — Record status (position 05)
* ``type`` — Type of record (position 06)
* ``encoding`` — Character coding scheme (position 09)
* ``base_addr`` — Base address of data (position 12-16)

A leader matcher expression always consists of the prefix ``ldr.`` followed
by the field to which the matcher refers. This is followed by a comparison
operator (``==``, `!=`, ``>=``, ``<=``, ``>``, or ``<``), which specifies
the type of comparison, and a reference value against the comparison is to
be made. The data type of the reference value must match the data type of the
corresponding leader field; i.e., the base address and record length can only
be compared with a 32-bit unsigned integer value, and the remaining fields can
only be compared with a single character enclosed in either single or double
quotes.

Suppose the following leader of the *Ada Lovelace* test record:

.. code-block:: text

	LDR 03612nz  a2200589nc 4500

We can test this matcher using the :func:`read_marc21
<marc21_learn.io.read_marc21>` function and checking the height of the
resulting dataframe. If the height is ``1``, the leader of this record meets
the criterion; otherwise, it does not:

	>>> from marc21_learn.io import read_marc21
	>>>
	>>> filename = "../tests/data/ada.mrc.gz"
	>>> predicate = "ldr.length == 3612"
	>>> query = "001 AS `cn`"
	>>>
	>>> df = read_marc21(filename, query, predicate=predicate)
	>>> df.height
	1


.. _ref-field-matcher:

Field Matcher
-------------

*tba*
