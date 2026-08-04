# marc21-check(1)

## NAME

*marc21-check* --- Validate records against rule sets

## SYNOPSIS

`marc21 check` [_options_] [_path_]...


## DESCRIPTION

The `check` command can be used to verify whether records conform to
a set of rules. The rules are defined by the user and are referenced
by a unique identifier that can be chosen freely. When the command is
executed, the input records are checked against each rule. If validation
for a rule fails, the record is written to the output along with the
control number (field 001), an error message, and the rule’s identifier.
By default, the output is in [DVRF] format.

A set of rules is defined in [TOML] format and contains a list of
rule definitions. The rule set can optionally be restricted using
the (global) `scope` option. The rules contained in the file are then
checked only for the records within the scope. In the following example,
two rules with the identifiers `RULE001` and `RULE002` are defined. In
accordance with the scope, these rules are applied only to authority
records.

```toml
scope = 'ldr.type == "z"'

[rule.R001]
message = 'invalid field ABC'
...

[rule.R002]
message = 'invalid field DEF'
...
```

A rule consists, in addition to the identifier, of a error `message`
(required),  a (local) `scope`, the error `level` (`warning`, `info`,
`error`), and the the `validator` specification (required). Depending on
the selected [validator], there might be additional required fields.

The output format is automatically determined based on the file
extension. The following formats are supported: [DVRF] format (file
extensions `.json` or `.json.gz`), CSV format (file extensions `.csv`
or `.csv.gz`), text format (file extensions `.txt` or `.txt.gz`). If
the format cannot be determined based on the file extension, the [DVRF]
format is used by default. In text format, the control number is written
to the output line by line without any additional information.

In the following example, the records in the file `DUMP.mrc.gz` are
checked against the two rule sets `gnd.toml` and `dnb.toml`; the output
is in [DVRF] format:

```console,ignore
$ marc21 check -s -R gnd.toml -R dnb.toml DUMP.mrc.gz -o result.json
```


## VALIDATORS

### Filter

The filter validator checks whether a record matches a filter criterion
([record matcher]). Unless explicitly stated by the `invert-match` flag,
the validation fails if the record match the filter expression. The
filter expression must therefore be constructed in such a way that it
identifies invalid records.

In the following example, the field `075` is checked to ensure that only
valid entity codes appear in the subfield `$b` when the subfield `$2` is
set to the value `gndgen`. The validation of a record fails if a value
is found in the field `$b` that is not `b`, `f`, `g`, `p`, `s`, or `u`.

```toml
[rule.GND-001-INVALID-ENTITY-CODE]
message = 'invalid entity code'
validator = 'filter'
filter = '075{ b not in ["b", "f", "g", "p", "s", "u"] && 2 == "gndgen" }'
```

The validator supports the following options/flags:

`filter = <record-matcher>`
  : A [record matcher] expression that is evaluated against the record.

`invert-matcher = true | false`
  : If this flag is set, the result is inverted: A record fails
    validation if the filter expression is `false`.

## OPTIONS

### FILTER OPTIONS

{{ #include filter-opts.md }}

### COMMON OPTIONS

{{ #include common-opts.md }}

## EXIT STATUS

{{ #include exit-status.md }}



[DVRF]: https://gbv.github.io/data-validation-report-format/
[TOML]: https://toml.io/en/
[record matcher]: ../../concepts/record-matcher.md
[validator]: #validators
