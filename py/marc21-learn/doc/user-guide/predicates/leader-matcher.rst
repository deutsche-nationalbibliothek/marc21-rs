:html_theme.sidebar_secondary.remove:

.. _ref-leader-matcher:

Leader Matcher
==============

.. currentmodule:: marc21_learn.io

Records can be filtered based on selected properties of the leader. The
following properties can be checked:

* ``base_addr`` — Base address of data (position 12-16)
* ``encoding`` — Character coding scheme (position 09)
* ``length`` — Record length (position 00-04)
* ``status`` — Record status (position 05)
* ``type`` — Type of record (position 06)

A *leader predicate* always consists of the prefix ``ldr.`` followed by
the property to which the predicate refers. This prefix is followed by a
comparison operator (``==``, ``!=``, ``>=``, ``<=``, ``>``, or ``<``), which
specifies the type of comparison, and a reference value against the comparison
is to be made. Note that the data type of the reference value must match
the data type of the corresponding leader field; i.e., the ``base_addr`` and
``length`` field can only be compared with a 32-bit unsigned integer value,
and the remaining fields can only be compared with a single character enclosed
in either single or double quotes.

Suppose the following leader of the *Ada Lovelace* test record:

.. code-block:: text

	LDR 03612nz  a2200589nc 4500

Examples:

	>>> from marc21_learn.io import read_marc21
	>>>
	>>> filename = "../tests/data/ada.mrc.gz"
	>>>
	>>> df = read_marc21(filename, "001", predicate="ldr.base_addr == 589")
	>>> assert df.height == 1
	>>>
	>>> df = read_marc21(filename, "001", predicate="ldr.length >= 3600")
	>>> assert df.height == 1
	>>>
	>>> df = read_marc21(filename, "001", predicate="ldr.status == 'n'")
	>>> assert df.height == 1
	>>>
	>>> df = read_marc21(filename, "001", predicate="ldr.type == 'z'")
	>>> assert df.height == 1
	>>>
	>>> df = read_marc21(filename, "001", predicate="ldr.encoding != ' '")
	>>> assert df.height == 1
