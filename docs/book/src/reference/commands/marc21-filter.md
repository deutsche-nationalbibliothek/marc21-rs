# marc21-filter(1)

## NAME

*marc21-filter* --- Filter records that fulfill a specified condition

## SYNOPSIS

`marc21 filter` [_options_] [_path_]...

## DESCRIPTION

_tba_

## OPTIONS

`--filter-normalization <form>`
: Transliterate the given filter expression into the specified Unicode
normal form. Possible values: `nfd`, `nfkd`, `nfc`, `nfkc`. This
option can also be specified by setting the environment variable
`MARC21_FILTER_NORMALIZATION`.

`--query-normalization <form>`
: Transliterate all query or path expression into the specified Unicode normal
form. Possible values: `nfd`, `nfkd`, `nfc`, `nfkc`. This option can also be
specified by setting the environment variable `MARC21_QUERY_NORMALIZATION`.

`-A <filename>`, `--allow-list <filename>`
: Ignores all records that are *not* explicitly listed in one of the allow
  lists.

  Allow lists in CSV format (*.csv, *.csv.gz) and plain text format
  (*.txt, *.txt.gz) are supported. Unless otherwise specified
  (`--filter-list-source`), the values from the allow lists are compared
  against the control number from control field 001. When processing
  CSV files, the column labeled `cn` is selected by default for reading
  the values. A different column name can be specified using the
  `--filter-list-column` option. Note that, if the allow list is empty, all
  records are blocked. With more than one allow list, the filter list is
  made up of all partial lists.

`-D <filename>`, `--deny-list <filename>`
: Ignores all records that are listed in one of the deny lists.

  Deny lists in CSV format (*.csv, *.csv.gz) and plain text format
  (*.txt, *.txt.gz) are supported. Unless otherwise specified
  (`--filter-list-source`), the values from the allow lists are compared
  against the control number from control field 001. When processing
  CSV files, the column labeled `cn` is selected by default for reading
  the values. A different column name can be specified using the
  `--filter-list-column` option. Note that, if the deny list is empty, no
  records are discarded. With more than one deny list, the filter list is
  made up of all partial lists.

`--filter-list-source <path>`
: Specify a path expression to determine the values to be
  compared against Allow/Deny lists. By default the control
  number of a record (field `001`) is used.

`--filter-list-column <name>`
: Specify the column name from which the values for an Allow/Deny
  list are sourced in CSV format. By default, the column name
  `cn` is used.

`-v`, `--invert_match`
: Inverts the specified filter criterion, which means that only records
  that do not match the criterion are returned.


### COMMON OPTIONS

{{ #include common-opts.md }}

## EXIT STATUS

{{ #include exit-status.md }}

## EXAMPLES

_tba_
