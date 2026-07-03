# marc21-grep(1)

## NAME

*marc21-grep* --- Search for records whose values match a pattern

## SYNOPSIS

`marc21 grep` [_OPTIONS_] `<PATTERN>` [_INPUT_]...

## DESCRIPTION

The `grep` command searches for records whose values match one or
multiple patterns (regular expressions). Only values from control and
data fields are searched.

## ARGUMENTS

`<PATTERN>`
: A regular expression used for searching.

## OPTIONS

`--or <pattern>`
: Search for multiple, possibly overlapping, regexes in a single search.
The regular expression consists of the main pattern and all other
pattern passed by this option. The regex matches if a subfield is found
that matches against at least one pattern.

`-i`, `--ignore-case`
:  If this flag is set, matching will be performed case insensitive.
This setting applies to all specified patterns. If you want to match
only a single pattern in a case-insensitive mode, you can do so
using the inline flag `i`. For example, `(?i:foo)` matches `foo` case
insensitively while `(?-i:foo)` matches `foo` case sensitively.

`-v`, `--invert-match`
: Inverts the specified regular expression, which means that only
records that do not match the criterion are returned.

`-o <filename>`, `--output <filename>`
: Write output to `<filename>` instead of `stdout`.

### FILTER OPTIONS

{{ #include filter-opts.md }}

### COMMON OPTIONS

{{ #include common-opts.md }}

## EXIT STATUS

{{ #include exit-status.md }}

## EXAMPLES

In the following example, all records that have a field with a value
that matches the regular expression `^MARC[-\s]21$` are returned:


```console
$ marc21 grep '^MARC[-\s]21$' GND.mrc.gz -o out.mrc
```
