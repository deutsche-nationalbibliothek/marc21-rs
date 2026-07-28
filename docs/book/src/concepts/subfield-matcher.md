# Subfield Matcher

A subfield matcher is an expression that is applied to a list of
subfields (of a data field) and checks whether that list meets the
specified criteria. The matcher is primarily used as part of a [field
matcher] and as a constraint in [query] or [path] expressions.

The following elementary matcher variants are distinguished:

| Matcher                           | Operator       | Description                                                                  |
|-----------------------------------|----------------|------------------------------------------------------------------------------|
| [Exists](#exists-matcher)         | `?`            | checks whether a specific subfield is present                                |
| [Count](#count-matcher)           | `#`            | checks the number of occurrences of a subfield                               |
| [Comparison](#comparison-matcher) | `==`, `!=`, `>=`, `>`, `<=`, `<` | compares the value of a subfield against a reference value |
| [Substring](#substring-matcher)   | `=?`, `!?`     | checks whether a subfield contains a specific phrase                         |
| [Member](#member-matcher)         | `in`, `not in` | checks whether the value of a subfield comes from a reference list           |
| [Prefix](#prefix-matcher)         | `=^`, `!^`     | checks whether the value of a subfield begins with a prefix                  |
| [Suffix](#suffix)                 | `=$`, `!$`     | checks whether the value of a subfield ends with a suffix                    |
| [Similarity](#similarity-matcher) | `=*`. `!*`     | checks whether the value of a subfield is similar to a reference             |
| [Regex](#regex-matcher)           | `=~`, `!~`     | checks whether the value of a subfield matches a regular expression          |

Using the [Boolean connectives] _AND_ `&&` and _OR_ `||`, the elementary
matcher variants can be combined to more complex statements.

## Exists Matcher

The `?` operator is used to check whether a data field has one or more
subfields. The specific value of the subfield is irrelevant in this
context. In the following example, the field `079` is checked to see if
a subfield named `u` exists:

```console
$ marc21 count tests/data/ada.mrc --where '079.a?'
1

```

By negating the expression, you can check whether a field does not
contain a specific subfield:

```console
$ marc21 count tests/data/ada.mrc --where '079{ !u? }'
0

$ marc21 count tests/data/ada.mrc --where '!079.x?'
1

```

By specifying a code class, you can check whether one of the specified
subfields is present. The following example checks whether the field
`079` contains either the subfield `p` or `q`:

```console
$ marc21 count tests/data/ada.mrc --where '079{ [pq]? }'
1

$ marc21 count tests/data/ada.mrc --where '079.[pq]?'
1

```

## Count Matcher

The _count matcher_ (`#`) counts the number of occurrences of one or
more subfields and compares that number against a reference value. The
available comparison operators are `==`, `!=`, `>=`, `>`, `<=`, and
`<`. The count matcher can only be used in the long form of a [field
matcher].

```console
$ marc21 count tests/data/ada.mrc --where '079{ #u > 2 }'
1

$ marc21 count tests/data/ada.mrc --where '079{ #[aq-w] >= 7 }'
1

```

## Comparison Matcher

The comparison matcher compares the value of a subfield to a reference
value. The available comparison operators are `==`, `!=`, `>=`, `>`,
`<=`, and `<`.

```console
$ marc21 count tests/data/ada.mrc --where '075{ b == "piz" }'
1

$ marc21 count tests/data/ada.mrc --where '075.b != "piz"'
1

```

Optionally, the statement can be quantified using the universal
quantifier `ALL` or the existential quantifier `ANY`. By default, the
existential quantifier is used; that is, the existence of at least
one  subfield that corresponds to the reference value according to
the operator is sufficient for the statement to be true. Quantified
expressions can't be used in the short form of a [field matcher].

```console
$ marc21 count tests/data/ada.mrc --where '079{ ALL u >= "k" }'
1

$ marc21 count tests/data/ada.mrc --where '079{ ANY u != "k" }'
1

```

## Substring Matcher

The _substr matcher_ checks whether the specified substring is
contained in the specified subfield. Internally, the matcher uses the
[Aho–Corasick algorithm] (with [SIMD] acceleration in some cases) to
enable efficient substring searches.

```console
$ marc21 count tests/data/ada.mrc --where '400/1#{ a =? "Augusta" }'
1

$ marc21 count tests/data/ada.mrc --where '400/1#.a =? "Augusta"'
1

```

The matcher also allows you to search for multiple substrings at once by
specifying a list of substrings:

```console
$ marc21 count tests/data/ada.mrc --where '400/1#.a =? ["Hate", "Love"]'
1

```

To check whether a substring (or a list of substrings) is not contained,
use the `!?` operator:

```console
$ marc21 count tests/data/ada.mrc --where '400/1#.a !? "Curie"'
1

```

Finally, the statements can be quantified using the universal quantifier
`ALL` or the existential quantifier `ANY`. A quantifier can't be used in
the short form of a [field matcher].


```console
$ marc21 count tests/data/ada.mrc --where '035{ ALL [az] =? "DE-" }'
1

```

## Member Matcher

The _member matcher_ checks whether the value of a subfield comes from a
reference list. The values are specified as a non-empty, comma-separated
list enclosed in square brackets. If you want to check whether the
value of a field *does not* come from a list, use the `not in` operator
instead of `in`.

```console
$ marc21 count tests/data/ada.mrc --where '075{ b in ["p", "s", "u"] }'
1

$ marc21 count tests/data/ada.mrc --where '075.b in ["p", "s", "u"]'
1

$ marc21 count tests/data/ada.mrc --where '075.b not in ["b", "f", "g"]'
1

```

The statements can be quantified using the universal quantifier `ALL`
or the existential quantifier `ANY`. A quantifier can't be used in the
short form of a [field matcher].

```console
$ marc21 count tests/data/ada.mrc --where '079{ ALL u in ["w", "k", "v"] }'
1

```

## Prefix Matcher

The _prefix matcher_ checks whether the value of a subfield begins with
a prefix. If the matcher is to search for multiple possible prefixes,
the values are specified as a list. To check whether the value does
*not* begin with a prefix, the `!^` operator is used.

```console
$ marc21 count tests/data/ada.mrc --where '400/1#{ a =^ "Love" }'
1

$ marc21 count tests/data/ada.mrc --where '400/1#.a =^ "Lovelace"'
1

$ marc21 count tests/data/ada.mrc --where '400/1#.a =^ ["Love", "Hate"]'
1

$ marc21 count tests/data/ada.mrc --where '400/1#.a !^ "Hate"'
1

```

The statements can be quantified using the universal quantifier `ALL`
or the existential quantifier `ANY`. A quantifier can't be used in the
short form of a [field matcher].

```console
$ marc21 count tests/data/ada.mrc --where '400/1#{ ALL d =^ "1815" }'
1

```

## Suffix Matcher

The _suffix matcher_ checks whether the value of a subfield ends with a
suffix. If the matcher is to search for multiple possible suffixes, the
values are specified as a list. To check whether the value does *not*
end with a suffix, the `!$` operator is used.

```console
$ marc21 count tests/data/ada.mrc --where '400/1#{ a =$ "Ada" }'
1

$ marc21 count tests/data/ada.mrc --where '400/1#.a =$ "Ada"'
1

$ marc21 count tests/data/ada.mrc --where '400/1#.a =$ ["Ada", "Bob"]'
1

$ marc21 count tests/data/ada.mrc --where '400/1#.a !$ "Ada"'
1

```

The statements can be quantified using the universal quantifier `ALL`
or the existential quantifier `ANY`. A quantifier can't be used in the
short form of a [field matcher].

```console
$ marc21 count tests/data/ada.mrc --where '400/1#{ ALL d =$ "1852" }'
1

```

## Similarity Matcher

The [similarity matcher] checks whether the value of a subfield is
similar to a reference value. Similarity is determined by calculating
the normalized [Levenshtein distance] between the subfield value and
the reference value. The values range from `0.0` to `1.0` (inclusive),
where a value of `1.0` indicates that the two values match. A match is
considered to exist if the similarity value is greater than or equal
to the threshold value, which can be configured using the command-line
option `--strsim-threshold` and defaults to `80` (≙ `0.8`). To check for
non-similarity, the `!*` operator is used.

```console
$ marc21 count tests/data/ada.mrc --where '400/1#{ a =* "Kong, Ada" }'
1

$ marc21 count tests/data/ada.mrc --where '400/1#{ a =* "Hatless, Ada" }'
0

$ marc21 count tests/data/ada.mrc --where '400/1#{ a !* "Hatless, Ada" }'
1

```

The statements can be quantified using the universal quantifier `ALL`
or the existential quantifier `ANY`. A quantifier can't be used in the
short form of a [field matcher].

## Regex Matcher

The [regex matcher] `=~` checks whether the value of a subfield matches
a regular expression. To check multiple patterns at once, list all
patterns in square brackets. To check whether the value *does not* match
a regular expression, use the `!~` operator.

> [!NOTE]
> Not all regex functions are supported. Please consult the [syntax
> documentation] of the regex library used in this project if you have
> any questions.
>
> Also keep in mind that, depending on the context and the type of
> quotes used (single or double), special characters in the regular
> expression may need to be quoted.


```console
$ marc21 count tests/data/ada.mrc --where '400/1#{ d =~ "^\\d{4}-\\d{4}$" }'
1

$ marc21 count tests/data/ada.mrc --where '075.b =~ ["^[bfg]$", "^piz$"]'
1

$ marc21 count tests/data/ada.mrc --where '400/1#.d =~ "^\\d{4}-\\d{4}$"'
1

$ marc21 count tests/data/ada.mrc --where 'ALL 075.2 =~ "^gnd(gen|spec)$"'
1

$ marc21 count tests/data/ada.mrc --where '075.b !~ "^[bfg]$"'
1

```

The statements can be quantified using the universal quantifier `ALL`
or the existential quantifier `ANY`. A quantifier can't be used in the
short form of a [field matcher].

```console
$ marc21 count tests/data/ada.mrc --where '079{ ALL u =~ "^[a-z]$" }'
1

```

> [!TIP]
> The [Rustexp] website offers a regular expression editor and tester.


## Boolean Connectives

More complex statements can be formed using the two Boolean operators
AND `&&` and OR `||`. Since the connected sub-statements always refer to
a single field, Boolean operators can only be used in the long form of a
[field matcher].

In the following example, the data record must contain a field `075` for
which the following is true: There exists a subfield `b` with the value
`p` **and**, within the same field, there exists a subfield `2` with the
value `gndgen`.

```console
$ marc21 count tests/data/ada.mrc --where '075{ b == "p" && 2 == "gndgen" }'
1

$ marc21 count tests/data/ada.mrc --where '075{ b == "p" && 2 == "gndspec" }'
0

```

The `&&` operator has higher precedence than the `||` operator, which
means that the expression `A || B && C` is equivalent to `A || (B &&
C)`. It follows that parentheses may be necessary:

```console
$ marc21 count tests/data/ada.mrc --where '075{ (((b == "p") || b == "p") && 2 == "gndgen")  }'
1

```


[boolean connectives]: #boolean-connectives
[comparison matcher]: #comparison-matcher
[count matcher]: #count-matcher
[exists matcher]: #exists-matcher
[field matcher]: ./record-matcher.md#field-matcher
[member matcher]: #member-matcher
[path]: ./query-and-path.md#path
[prefix matcher]: #prefix-matcher
[query]: ./query-and-path.md#query
[regex matcher]: #regex-matcher
[similarity matcher]: #similarity-matcher
[substring matcher]: #substring-matcher
[suffix matcher]: #suffix-matcher

[Aho–Corasick algorithm]: https://en.wikipedia.org/wiki/Aho%E2%80%93Corasick_algorithm
[Levenshtein distance]: https://en.wikipedia.org/wiki/Levenshtein_distance
[SIMD]: https://en.wikipedia.org/wiki/Single_instruction,_multiple_data
[syntax documentation]: https://docs.rs/regex/latest/regex/#syntax
[Rustexp]: https://rustexp.lpil.uk/
