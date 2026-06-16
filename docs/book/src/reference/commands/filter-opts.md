`-l`, `--limit <n>`
: Limit the result to first `<n>` records (a limit value `0` means no
limit)

`-s`, `--skip-invalid`
: Skip invalid records that can't be decoded

`--strsim-threshold`
: The minimum score for string similarity comparisons (0 <= score <= 100)

`--where`
: An expression for filtering records

`--filter-normalization <form>`
: Transliterate the given filter or query expression into the specified
Unicode normal form. Possible values: `nfd`, `nfkd`, `nfc`, `nfkc`.
This option can also be specified by setting the environment variable
`MARC21_FILTER_NORMALIZATION`.
