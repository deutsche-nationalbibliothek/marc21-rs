`-l`, `--limit <n>`
  : Limit the result to first `<n>` records (a limit value `0` means
    no limit)

`-s`, `--skip-invalid`
  : Skip invalid records that can't be decoded

`--strsim-threshold <value>`
  : The minimum score for string similarity comparisons. The value must
    be between 0 and 100.

`--where`
  : An [filter] expression for filtering records

`--filter-normalization <form>`
  : Transliterate the given filter or query expression into the
    specified Unicode normal form. Possible values: `nfd`, `nfkd`,
    `nfc`, `nfkc`. This option can also be specified by setting the
    environment variable `MARC21_FILTER_NORMALIZATION`.


[filter]: ../../concepts/filter.md
