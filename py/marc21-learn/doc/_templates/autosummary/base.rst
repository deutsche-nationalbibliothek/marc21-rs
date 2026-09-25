{% if objtype == "function" %}
:html_theme.sidebar_secondary.remove:
{% endif %}

{{ name | escape | underline }}

.. currentmodule:: {{ module }}

.. auto{{ objtype }}:: {{ objname }}
