# Indicator Matcher

Data fields are distinguished not only by the tag, but also by an
indicator consisting of two lowercase ASCII alphabetic, numeric, or
blank characters. The indicator matcher checks whether a data field
matches the specified value. It always appears in combination with a
[tag matcher] and is preceded by the prefix `/`.

The indicator matcher distinguishes between three types:

- [explicit matching] (`/10`, `/#1`),
- [pattern based matching] (`/1[23]`, `/1[2-5]`),
- [wildcard matching] (`/*`)

> [!NOTE]
> If no indicator matcher is specified, only the fields that contain a
> space in both positions are taken into account.


## Explicit Matching

The simplest form is to explicitly specify the two indicator positions,
preceded by the prefix `/`. A space is not a valid value and must be
replaced with `#`.

### Examples

```console
$ marc21 count tests/data/ada.mrc --where '400/1#?'
1

$ marc21 count tests/data/ada.mrc --where '400/11?'
0

```

## Pattern Based Matching

If you need to specify more than one specific value for an indicator,
the pattern-based approach can be helpful. In this case, a position
can be either an explicit value, a class of values (`[1-3]`), or any
value (`.`). The elements of a class are specified by listing the
allowed values enclosed in square brackets (e.g., `[136]`). A class can
also contain one or more ranges, such as `[13-56-9]`. Negation is also
supported by prefixing the class with `^` (`[^234]`); this class matches
all positions that are not `2`, `3`, or `4`.

### Examples

- `/2.` --- Indicators that begin with `2`, followed by any value (e.g., `25` or `2#`)
- `/2[35]` --- Indicators that begin with `2`, followed by a `3` or `5` (`22` or `25`)
- `/2[3-5]` --- Indicators that begin with `2`, followed by a `3`, `4`, or `5` (`23`, `24`, or `25`)
- `/2[^35]` --- Indicators that begin with `2` and are not followed by a `3` or `5` (e.g., `21`, `2#`, `29`)
- `/[12][1#]` --- Indicators that begin with `1` or `2`, followed by `1` or a blank (`#`) (e.g., `11`, `12`, `1#`)
- `/..` or `/[0-9#][0-9#]` --- Accepts all indicators

```console
$ marc21 count tests/data/minna.mrc --where '083/0.?'
1

$ marc21 count tests/data/minna.mrc --where '083/0[34]?'
1

$ marc21 count tests/data/minna.mrc --where '083/0[3-5]?'
1

$ marc21 count tests/data/minna.mrc --where '083/0[^5-9#]?'
1

$ marc21 count tests/data/minna.mrc --where '083/[30][34]?'
1

$ marc21 count tests/data/minna.mrc --where '100/[0-9#][0-9#]?'
1

$ marc21 count tests/data/minna.mrc --where '083/..?'
1

```

## Wildcard Matching

If you want to accept all possible indicators associated with a data
field, you could use the wildcard expression `/*`:

```console
$ marc21 count tests/data/ada.mrc --where '100/*.a =? "Ada"'
1

```



[tag matcher]: ./tag-matcher.md
[explicit matching]: #explicit-matching
[pattern based matching]: #pattern-based-matching
[wildcard matching]: #wildcard-matching
