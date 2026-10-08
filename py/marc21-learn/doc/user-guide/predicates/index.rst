:html_theme.sidebar_secondary.remove:

.. _ref-predicates:

Predicates
==========

.. currentmodule:: marc21_learn.io
 
Predicates are used within ``marc21-learn`` to *filter* records and fields
according to specific criteria. Filtering out records that are not needed
early on prevents unnecessary computations; in particular, it avoids
unnecessary memory allocations. In the context of the Polars integration, the
filter expressions are used for *predicate pushdown* to filter out records as
early as possible.

When filtering records, a distinction is made between the two basic
expressions: :ref:`leader matcher <ref-leader-matcher>` and :ref:`field
matcher <ref-field-matcher>`. These elementary expressions can be
combined into more complex expressions using the :ref:`Boolean connectives
<ref-boolean-connectives>` conjunction (``&&``) and disjunction (``||``),
as well as grouping (``(...)``). While the :ref:`leader matcher
<ref-leader-matcher>` makes statements about the leader of a record,
the :ref:`field matcher <ref-field-matcher>` is used to make statements about
the variable fields.

The following sections demonstrate the various ways to filter records using
the :func:`read_marc21` function and the *Ada Lovelace* test record. After
the predicate has been applied to the record, the height of the DataFrame
is checked. If the height is ``1``, the test record meets the criterion;
otherwise, it does not.


.. toctree::
	:maxdepth: 2
	:hidden:

	leader-matcher
	field-matcher
	boolean-connectives


