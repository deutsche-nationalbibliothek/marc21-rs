==============
Input / Output
==============


The I/O component provides the two fundamental operations of relational
algebra: :ref:`Projection <ref-projection>` allows MARC21 records to be
transformed into a rectangular data schema (tabular form), and
:ref:`selection <ref-selection>` allows records to be filtered based on
various criteria.

The ``marc21-learn`` toolkit offers two different approaches:
The :ref:`Polars API <ref-polars-integration>` allows you to tabulate
datasets using :ref:`queries <ref-projection>`, while the :ref:`Dataset API
<ref-datasets>` focuses on the extraction and preparation of features for
machine learning.


.. toctree::
   :maxdepth: 2
   :hidden:

   projection
   selection/index
