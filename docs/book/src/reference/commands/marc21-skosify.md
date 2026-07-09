# marc21-skosify(1)

## NAME

*marc21-skosify* --- Convert records to SKOS/RDF

## SYNOPSIS

`marc21 skosify` [_options_] [_path_]...

## DESCRIPTION

This command converts a set of arbitrary MARC21 records into a [SKOS/RDF]
graph. The specifications for how the conversion should be performed are
defined in a configuration file.

> [!NOTE]
> If you're interested in converting MARC21 to [SKOS/RDF], you should take
> a look at [mc2skos] as an alternative.

## CONFIGURATION

Parameterization is performed using a configuration file in [TOML] format,
which must be specified using the `-c` or `--config` option.

### Scope

If only a subset of the input records is to be processed, a filter
criterion can be specified using the `scope` option. Only records that
meet this criterion are included in the SKOS graph.

> [!NOTE]
> In addition to this option, the set of records to be processed can
> also be limited using the command-line option `--where`. If both
> methods are used, a record must meet **both** criteria.


In the following example, the scope is defined as follows: Only
authority records (leader type `z`) with an authentication code `gnd1`
(field [042 $a]) and an _authority data code_ `s` in field `079 $q`
are processed.

```toml
scope = 'ldr.type == "z" && 042.a == "gnd1" && 079.q == "s"'
```

### Concept URI

There are two ways to specify the URI of a concept: Either by directly
specifying it using a [path] expression (only the first value of the
expression is used). In the following example, the URI from the field
`024/7# $0` is used if the subfield `$2` in the same field contains the
value `gnd`:

```toml
uri = { path = '024/7#{ 0 | 2 == "gnd" }' }
```

Alternatively, the URI can be created by concatenating a base URI and
a value determined by a [path] expression (only the first value is
used). In the following example, the URI is formed from the base URI
`https://d-nb.info` and the control number of the record (field [001]):

```toml
uri = { base-uri = 'https://d-nb.info/', path = '001' }
```

### Groups

The properties of a concept are specified within a group. The group
defines which SKOS property is derived from which MARC21 values. This
type of specification takes into account the fact that the values are
often located in other fields, depending on a property of the record.
A group can therefore (optionally) be restricted to a subset of records
by specifying a `scope`. The groups are processed in order. Processing
of the groups stops when at least one triple has been generated for
a record.

In the following example, a `subject-heading` group is created
that refers only to GND subject headings (`scope`). The `prefLabel`
(`preferred`) is derived from field `150 $a`, and the `altLabel`
(`alternative`) from field `450 $a`. No `hiddenLabel`s (`hidden`) are
defined.

```toml
[group.subject-heading]
scope = '075{ b == "s" && 2 == "gndgen" }'
labels = [
  { kind = 'preferred', path = '150.a' },
  { kind = 'alternative', path = '450.a' },
]
```

### Miscellaneous

`pretty = true | false`
  : The `pretty` flag can be used to specify whether extra effort
    should be made to format the output nicely. This flag should be
    used with caution, as grouping related triples can be very time- and
    resource-intensive. If the flag is not set (default), output occurs
    in streaming mode, meaning subject and predicate "factorization"
    will only occur based on the previous triple(s) in the stream.


## OPTIONS

`-c`, `--config`
  : Specifies the configuration file that defines the parameters rules
    for the [SKOS/RDF] graph.

`-o <filename>`, `--output <filename>`
  : Write output to `<filename>` instead of `stdout`. The output is
    automatically Gzip-compressed if the file ends with the suffix
    `.gz`.

### Filter Options

{{ #include filter-opts.md }}

### Common Options

{{ #include common-opts.md }}


## EXIT STATUS

{{ #include exit-status.md }}


## EXAMPLES

In the following example, an SKOS graph is created that consists
solely of GND entities of type `saf` ("Formangabe"). The literals for
`prefLabel` and `altLabel` are taken from fields `150 $a` and `450 $a`.

```toml
scope = '''
  ldr.type == "z"
    && 075{ b == "saf" && b == "saz" && 2 == "gndspec" }
    && 042.a == "gnd1" && 079.q == "s"
'''

uri = {
  base-uri = "https://explore.gnd.network/gnd/",
  path = '024/7#{ a | 2 == "gnd" }'
}

[group.entity-type-saf]
labels = [
  { kind = 'preferred', path = '150.a' },
  { kind = 'alternative', path = '450.a' },
]
```

The graph can be generated using the following command (for the
sake of brevity, the output has been limited to the [Rezension]
(review/recension) entity):

```console
$ marc21 skosify -c saf.toml authorities-gnd-sachbegriff_dnbmarc.mrc.gz \
    --where '001 == "040497127"'
PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
<http://d-nb.info/gnd/4049712-4> a <http://www.w3.org/2004/02/skos/core#Concept>;
        <http://www.w3.org/2004/02/skos/core#prefLabel> "Rezension";
        <http://www.w3.org/2004/02/skos/core#altLabel> "Buchbesprechung",
                "Buchrezension",
                "Buchkritik".
```

[001]: https://www.loc.gov/marc/authority/ad001.html
[042 $a]: https://www.loc.gov/marc/authority/ad042.html
[mc2skos]: https://pypi.org/project/mc2skos/
[path]: ../../concepts/path.md
[Rezension]: https://explore.gnd.network/gnd/4049712-4
[SKOS/RDF]: https://www.w3.org/2004/02/skos/
[TOML]: https://toml.io/en/
