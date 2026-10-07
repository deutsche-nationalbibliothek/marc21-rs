:html_theme.sidebar_secondary.remove:

.. _ref-field-matcher:

Field Matcher
=============

The *field matcher* allows you to define criteria that must apply to `variable
fields`_. There are four different types:

* :ref:`exists matcher <ref-exists-matcher>` checks whether a field exists or not
* :ref:`control field predicate <ref-control-field-matcher>` operates on control fields
* :ref:`data field predicate <ref-data-field-matcher>` operates on data fields
* :ref:`count predicate <ref-count-matcher>` checks whether a specific number of fields are present

.. _variable fields: https://www.loc.gov/marc/specifications/specrecstruc.html#varifields


Control and data fields are identified by three-digit tags. Only numeric
digits from ``0`` to ``9`` are permitted in each position. If a tag begins
with two zeros, it is a control field; otherwise, it is a data field. A
data field is additionally identified by two indicators. Valid values for an
indicator are ``a`` to ``z``, ``0`` to ``9``, and a space (U+0020).

A :ref:`tag matcher <ref-tag-matcher>` is used to select the fields to which
the field matcher will be applied. When using a :ref:`data field matcher
<ref-data-field-matcher>`, you can also specify a :ref:`indicator matcher
<ref-indicator-matcher>` to define the indicator assignments.

.. toctree::
	:maxdepth: 2
	:hidden:

	tag-matcher
	indicator-matcher
	exists-matcher
	control-field-matcher
	data-field-matcher
	count-matcher
