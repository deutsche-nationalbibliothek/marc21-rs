#compdef marc21

autoload -U is-at-least

_marc21() {
    typeset -A opt_args
    typeset -a _arguments_options
    local ret=1

    if is-at-least 5.2; then
        _arguments_options=(-s -S -C)
    else
        _arguments_options=(-s -C)
    fi

    local context curcontext="$curcontext" state line
    _arguments "${_arguments_options[@]}" : \
'-h[Print help]' \
'--help[Print help]' \
'-V[Print version]' \
'--version[Print version]' \
":: :_marc21_commands" \
"*::: :->marc21" \
&& ret=0
    case $state in
    (marc21)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:marc21-command-$line[1]:"
        case $line[1] in
            (check)
_arguments "${_arguments_options[@]}" : \
'*-R+[A set of rules to be checked]:rule-set:_files' \
'*--rule-set=[A set of rules to be checked]:rule-set:_files' \
'-o+[Write output to <filename> instead of stdout]:filename:_files' \
'--output=[Write output to <filename> instead of stdout]:filename:_files' \
'-l+[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--limit=[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--strsim-threshold=[The minimum score for string similarity comparisons. The value must be between 0 and 100]:value:_default' \
'--where=[An expression for filtering records]:predicate:_default' \
'--filter-normalization=[Transliterate the given filter or query expression into the specified Unicode normal form]:form:(nfd nfkd nfc nfkc)' \
'--compression=[Specify compression level]:n:_default' \
'-s[Skip invalid records that can'\''t be decoded]' \
'--skip-invalid[Skip invalid records that can'\''t be decoded]' \
'-p[If set, show a progress bar]' \
'--progress[If set, show a progress bar]' \
'-h[Print help]' \
'--help[Print help]' \
'*::input -- MARC21 files to be processed as input. If no file is specified, or if the filename is `-`, the data is read from standard input (`stdin`) by default:_files' \
&& ret=0
;;
(concat)
_arguments "${_arguments_options[@]}" : \
'--tee=[Write to another output file at the same time]:path:_files' \
'-o+[Write output to <filename> instead of stdout]:filename:_files' \
'--output=[Write output to <filename> instead of stdout]:filename:_files' \
'-l+[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--limit=[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--strsim-threshold=[The minimum score for string similarity comparisons. The value must be between 0 and 100]:value:_default' \
'--where=[An expression for filtering records]:predicate:_default' \
'--filter-normalization=[Transliterate the given filter or query expression into the specified Unicode normal form]:form:(nfd nfkd nfc nfkc)' \
'--compression=[Specify compression level]:n:_default' \
'-a[Append to the given file, do not overwrite]' \
'--append[Append to the given file, do not overwrite]' \
'-s[Skip invalid records that can'\''t be decoded]' \
'--skip-invalid[Skip invalid records that can'\''t be decoded]' \
'-p[If set, show a progress bar]' \
'--progress[If set, show a progress bar]' \
'-h[Print help (see more with '\''--help'\'')]' \
'--help[Print help (see more with '\''--help'\'')]' \
'*::path:_files' \
&& ret=0
;;
(cat)
_arguments "${_arguments_options[@]}" : \
'--tee=[Write to another output file at the same time]:path:_files' \
'-o+[Write output to <filename> instead of stdout]:filename:_files' \
'--output=[Write output to <filename> instead of stdout]:filename:_files' \
'-l+[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--limit=[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--strsim-threshold=[The minimum score for string similarity comparisons. The value must be between 0 and 100]:value:_default' \
'--where=[An expression for filtering records]:predicate:_default' \
'--filter-normalization=[Transliterate the given filter or query expression into the specified Unicode normal form]:form:(nfd nfkd nfc nfkc)' \
'--compression=[Specify compression level]:n:_default' \
'-a[Append to the given file, do not overwrite]' \
'--append[Append to the given file, do not overwrite]' \
'-s[Skip invalid records that can'\''t be decoded]' \
'--skip-invalid[Skip invalid records that can'\''t be decoded]' \
'-p[If set, show a progress bar]' \
'--progress[If set, show a progress bar]' \
'-h[Print help (see more with '\''--help'\'')]' \
'--help[Print help (see more with '\''--help'\'')]' \
'*::path:_files' \
&& ret=0
;;
(count)
_arguments "${_arguments_options[@]}" : \
'-o+[Write output to FILENAME instead of stdout]:FILENAME:_files' \
'--output=[Write output to FILENAME instead of stdout]:FILENAME:_files' \
'-l+[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--limit=[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--strsim-threshold=[The minimum score for string similarity comparisons. The value must be between 0 and 100]:value:_default' \
'--where=[An expression for filtering records]:predicate:_default' \
'--filter-normalization=[Transliterate the given filter or query expression into the specified Unicode normal form]:form:(nfd nfkd nfc nfkc)' \
'--compression=[Specify compression level]:n:_default' \
'-s[Skip invalid records that can'\''t be decoded]' \
'--skip-invalid[Skip invalid records that can'\''t be decoded]' \
'-p[If set, show a progress bar]' \
'--progress[If set, show a progress bar]' \
'-h[Print help]' \
'--help[Print help]' \
'*::path:_files' \
&& ret=0
;;
(cnt)
_arguments "${_arguments_options[@]}" : \
'-o+[Write output to FILENAME instead of stdout]:FILENAME:_files' \
'--output=[Write output to FILENAME instead of stdout]:FILENAME:_files' \
'-l+[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--limit=[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--strsim-threshold=[The minimum score for string similarity comparisons. The value must be between 0 and 100]:value:_default' \
'--where=[An expression for filtering records]:predicate:_default' \
'--filter-normalization=[Transliterate the given filter or query expression into the specified Unicode normal form]:form:(nfd nfkd nfc nfkc)' \
'--compression=[Specify compression level]:n:_default' \
'-s[Skip invalid records that can'\''t be decoded]' \
'--skip-invalid[Skip invalid records that can'\''t be decoded]' \
'-p[If set, show a progress bar]' \
'--progress[If set, show a progress bar]' \
'-h[Print help]' \
'--help[Print help]' \
'*::path:_files' \
&& ret=0
;;
(dedup)
_arguments "${_arguments_options[@]}" : \
'--strategy=[Use the given strategy to determine duplicate records]:strategy:(cn hash)' \
'-o+[Write output to FILENAME instead of stdout]:path:_files' \
'--output=[Write output to FILENAME instead of stdout]:path:_files' \
'-l+[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--limit=[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--strsim-threshold=[The minimum score for string similarity comparisons. The value must be between 0 and 100]:value:_default' \
'--where=[An expression for filtering records]:predicate:_default' \
'--filter-normalization=[Transliterate the given filter or query expression into the specified Unicode normal form]:form:(nfd nfkd nfc nfkc)' \
'--compression=[Specify compression level]:n:_default' \
'-s[Skip invalid records that can'\''t be decoded]' \
'--skip-invalid[Skip invalid records that can'\''t be decoded]' \
'-p[If set, show a progress bar]' \
'--progress[If set, show a progress bar]' \
'-h[Print help (see more with '\''--help'\'')]' \
'--help[Print help (see more with '\''--help'\'')]' \
'*::input -- MARC21 files to be processed as input. If no file is specified, or if the filename is `-`, the data is read from standard input (`stdin`) by default:_files' \
&& ret=0
;;
(describe)
_arguments "${_arguments_options[@]}" : \
'-o+[Write output to <path> instead of stdout]:path:_files' \
'--output=[Write output to <path> instead of stdout]:path:_files' \
'-l+[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--limit=[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--strsim-threshold=[The minimum score for string similarity comparisons. The value must be between 0 and 100]:value:_default' \
'--where=[An expression for filtering records]:predicate:_default' \
'--filter-normalization=[Transliterate the given filter or query expression into the specified Unicode normal form]:form:(nfd nfkd nfc nfkc)' \
'--compression=[Specify compression level]:n:_default' \
'--tsv[Write output tab-separated (TSV)]' \
'-s[Skip invalid records that can'\''t be decoded]' \
'--skip-invalid[Skip invalid records that can'\''t be decoded]' \
'-p[If set, show a progress bar]' \
'--progress[If set, show a progress bar]' \
'-h[Print help]' \
'--help[Print help]' \
'*::path:_files' \
&& ret=0
;;
(filter)
_arguments "${_arguments_options[@]}" : \
'-l+[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--limit=[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--strsim-threshold=[The minimum score for string similarity comparisons (0 <= score <= 100)]:n:_default' \
'--filter-normalization=[Transliterate the given filter expression into the specified Unicode normal form]:form:(nfd nfkd nfc nfkc)' \
'-o+[Write output to FILENAME instead of stdout]:FILENAME:_files' \
'--output=[Write output to FILENAME instead of stdout]:FILENAME:_files' \
'--compression=[Specify compression level]:n:_default' \
'-s[Skip invalid records that can'\''t be decoded]' \
'--skip-invalid[Skip invalid records that can'\''t be decoded]' \
'-v[Inverts the specified filter criterion, which means that only records that do not match the criterion are returned]' \
'--invert-match[Inverts the specified filter criterion, which means that only records that do not match the criterion are returned]' \
'-p[If set, show a progress bar]' \
'--progress[If set, show a progress bar]' \
'-h[Print help]' \
'--help[Print help]' \
':filter -- An expression for filtering records:_default' \
'*::path:_files' \
&& ret=0
;;
(frequency)
_arguments "${_arguments_options[@]}" : \
'-t+[Ignore rows with a frequency less than <n>]:n:_default' \
'--threshold=[Ignore rows with a frequency less than <n>]:n:_default' \
'-n+[Limit result to the <n> most frequent subfield values]:n:_default' \
'--num=[Limit result to the <n> most frequent subfield values]:n:_default' \
'--separator=[Sets the separator used for squashing of repeated subfield values into a single value. Note that it'\''s possible to use the empty string as a separator]:SEPARATOR:_default' \
'--quote-style=[The quoting style to use when writing CSV/TSV]:QUOTE_STYLE:(always necessary non-numeric never)' \
'-H+[Insert a header row before the data. The header should be entered as a comma-separated list. Leading and trailing spaces in each column are automatically removed]:header:_default' \
'--header=[Insert a header row before the data. The header should be entered as a comma-separated list. Leading and trailing spaces in each column are automatically removed]:header:_default' \
'-o+[Write output to <path> instead of stdout]:path:_files' \
'--output=[Write output to <path> instead of stdout]:path:_files' \
'-l+[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--limit=[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--strsim-threshold=[The minimum score for string similarity comparisons. The value must be between 0 and 100]:value:_default' \
'--where=[An expression for filtering records]:predicate:_default' \
'--filter-normalization=[Transliterate the given filter or query expression into the specified Unicode normal form]:form:(nfd nfkd nfc nfkc)' \
'--compression=[Specify compression level]:n:_default' \
'-u[This flag ensures that all values generated for a record are counted only once in the frequency table]' \
'--unique[This flag ensures that all values generated for a record are counted only once in the frequency table]' \
'-r[Sort results in reverse order]' \
'--reverse[Sort results in reverse order]' \
'(--merge)--squash[Whether to squash all values of a repeated subfield into a single value or not. The separator can be specified by the \`--separator\` option]' \
'(--squash)--merge[If set, values of a column are merged into a single value. The separator can be specified by the \`--separator\`]' \
'--tsv[Write output tab-separated (TSV)]' \
'-s[Skip invalid records that can'\''t be decoded]' \
'--skip-invalid[Skip invalid records that can'\''t be decoded]' \
'-p[If set, show a progress bar]' \
'--progress[If set, show a progress bar]' \
'-h[Print help (see more with '\''--help'\'')]' \
'--help[Print help (see more with '\''--help'\'')]' \
':query -- A query expression:_default' \
'*::path:_files' \
&& ret=0
;;
(freq)
_arguments "${_arguments_options[@]}" : \
'-t+[Ignore rows with a frequency less than <n>]:n:_default' \
'--threshold=[Ignore rows with a frequency less than <n>]:n:_default' \
'-n+[Limit result to the <n> most frequent subfield values]:n:_default' \
'--num=[Limit result to the <n> most frequent subfield values]:n:_default' \
'--separator=[Sets the separator used for squashing of repeated subfield values into a single value. Note that it'\''s possible to use the empty string as a separator]:SEPARATOR:_default' \
'--quote-style=[The quoting style to use when writing CSV/TSV]:QUOTE_STYLE:(always necessary non-numeric never)' \
'-H+[Insert a header row before the data. The header should be entered as a comma-separated list. Leading and trailing spaces in each column are automatically removed]:header:_default' \
'--header=[Insert a header row before the data. The header should be entered as a comma-separated list. Leading and trailing spaces in each column are automatically removed]:header:_default' \
'-o+[Write output to <path> instead of stdout]:path:_files' \
'--output=[Write output to <path> instead of stdout]:path:_files' \
'-l+[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--limit=[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--strsim-threshold=[The minimum score for string similarity comparisons. The value must be between 0 and 100]:value:_default' \
'--where=[An expression for filtering records]:predicate:_default' \
'--filter-normalization=[Transliterate the given filter or query expression into the specified Unicode normal form]:form:(nfd nfkd nfc nfkc)' \
'--compression=[Specify compression level]:n:_default' \
'-u[This flag ensures that all values generated for a record are counted only once in the frequency table]' \
'--unique[This flag ensures that all values generated for a record are counted only once in the frequency table]' \
'-r[Sort results in reverse order]' \
'--reverse[Sort results in reverse order]' \
'(--merge)--squash[Whether to squash all values of a repeated subfield into a single value or not. The separator can be specified by the \`--separator\` option]' \
'(--squash)--merge[If set, values of a column are merged into a single value. The separator can be specified by the \`--separator\`]' \
'--tsv[Write output tab-separated (TSV)]' \
'-s[Skip invalid records that can'\''t be decoded]' \
'--skip-invalid[Skip invalid records that can'\''t be decoded]' \
'-p[If set, show a progress bar]' \
'--progress[If set, show a progress bar]' \
'-h[Print help (see more with '\''--help'\'')]' \
'--help[Print help (see more with '\''--help'\'')]' \
':query -- A query expression:_default' \
'*::path:_files' \
&& ret=0
;;
(glimpse)
_arguments "${_arguments_options[@]}" : \
'-n+[Maximum number of values to show per subfield]:n:_default' \
'--max-values=[Maximum number of values to show per subfield]:n:_default' \
'-o+[Write output to FILENAME instead of stdout]:FILENAME:_files' \
'--output=[Write output to FILENAME instead of stdout]:FILENAME:_files' \
'-l+[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--limit=[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--strsim-threshold=[The minimum score for string similarity comparisons. The value must be between 0 and 100]:value:_default' \
'--where=[An expression for filtering records]:predicate:_default' \
'--filter-normalization=[Transliterate the given filter or query expression into the specified Unicode normal form]:form:(nfd nfkd nfc nfkc)' \
'--compression=[Specify compression level]:n:_default' \
'-s[Skip invalid records that can'\''t be decoded]' \
'--skip-invalid[Skip invalid records that can'\''t be decoded]' \
'-p[If set, show a progress bar]' \
'--progress[If set, show a progress bar]' \
'-h[Print help]' \
'--help[Print help]' \
':path -- A path expression:_default' \
'*::input:_files' \
&& ret=0
;;
(grep)
_arguments "${_arguments_options[@]}" : \
'*--or=[Search for multiple, possibly overlapping, regexes in a single search. The regular expression consists of the main pattern and all other pattern passed by this option. The regex matches if a subfield is found that matches against at least one pattern]:pattern:_default' \
'-o+[Write output to <filename> instead of stdout]:filename:_files' \
'--output=[Write output to <filename> instead of stdout]:filename:_files' \
'-l+[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--limit=[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--strsim-threshold=[The minimum score for string similarity comparisons. The value must be between 0 and 100]:value:_default' \
'--where=[An expression for filtering records]:predicate:_default' \
'--filter-normalization=[Transliterate the given filter or query expression into the specified Unicode normal form]:form:(nfd nfkd nfc nfkc)' \
'--compression=[Specify compression level]:n:_default' \
'-i[If this flag is set, matching will be performed case insensitive]' \
'--ignore-case[If this flag is set, matching will be performed case insensitive]' \
'-v[Inverts the specified regular expression, which means that only records that do not match the criterion are returned]' \
'--invert-match[Inverts the specified regular expression, which means that only records that do not match the criterion are returned]' \
'-s[Skip invalid records that can'\''t be decoded]' \
'--skip-invalid[Skip invalid records that can'\''t be decoded]' \
'-p[If set, show a progress bar]' \
'--progress[If set, show a progress bar]' \
'-h[Print help (see more with '\''--help'\'')]' \
'--help[Print help (see more with '\''--help'\'')]' \
':pattern -- A regular expression used for searching:_default' \
'*::input -- MARC21 files to be processed as input. If no file is specified, or if the filename is `-`, the data is read from standard input (`stdin`) by default:_files' \
&& ret=0
;;
(hash)
_arguments "${_arguments_options[@]}" : \
'-o+[Write output to FILENAME instead of stdout]:FILENAME:_files' \
'--output=[Write output to FILENAME instead of stdout]:FILENAME:_files' \
'-l+[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--limit=[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--strsim-threshold=[The minimum score for string similarity comparisons. The value must be between 0 and 100]:value:_default' \
'--where=[An expression for filtering records]:predicate:_default' \
'--filter-normalization=[Transliterate the given filter or query expression into the specified Unicode normal form]:form:(nfd nfkd nfc nfkc)' \
'--compression=[Specify compression level]:n:_default' \
'--tsv[Write output tab-separated (TSV)]' \
'-s[Skip invalid records that can'\''t be decoded]' \
'--skip-invalid[Skip invalid records that can'\''t be decoded]' \
'-p[If set, show a progress bar]' \
'--progress[If set, show a progress bar]' \
'-h[Print help]' \
'--help[Print help]' \
'*::path:_files' \
&& ret=0
;;
(invalid)
_arguments "${_arguments_options[@]}" : \
'-o+[Write output to FILENAME instead of stdout]:FILENAME:_files' \
'--output=[Write output to FILENAME instead of stdout]:FILENAME:_files' \
'--compression=[Specify compression level]:n:_default' \
'-p[If set, show a progress bar]' \
'--progress[If set, show a progress bar]' \
'-h[Print help]' \
'--help[Print help]' \
'*::path:_files' \
&& ret=0
;;
(partition)
_arguments "${_arguments_options[@]}" : \
'-t+[A template for naming the individual partitions. The placeholder \`{}\` is replaced by the value of the path expression. If the template ends with the suffix \`.gz\`, the partitions are compressed in Gzip format]:template:_default' \
'--template=[A template for naming the individual partitions. The placeholder \`{}\` is replaced by the value of the path expression. If the template ends with the suffix \`.gz\`, the partitions are compressed in Gzip format]:template:_default' \
'-o+[Write output to <path>; by default all partitions are written to the current working directory]:path:_files' \
'--output=[Write output to <path>; by default all partitions are written to the current working directory]:path:_files' \
'-l+[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--limit=[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--strsim-threshold=[The minimum score for string similarity comparisons. The value must be between 0 and 100]:value:_default' \
'--where=[An expression for filtering records]:predicate:_default' \
'--filter-normalization=[Transliterate the given filter or query expression into the specified Unicode normal form]:form:(nfd nfkd nfc nfkc)' \
'--compression=[Specify compression level]:n:_default' \
'-s[Skip invalid records that can'\''t be decoded]' \
'--skip-invalid[Skip invalid records that can'\''t be decoded]' \
'-p[If set, show a progress bar]' \
'--progress[If set, show a progress bar]' \
'-h[Print help (see more with '\''--help'\'')]' \
'--help[Print help (see more with '\''--help'\'')]' \
':path -- A path expression:_default' \
'*::filenames:_files' \
&& ret=0
;;
(print)
_arguments "${_arguments_options[@]}" : \
'--translit=[Transliterate the output into the specified Unicode normal form]:form:(nfd nfkd nfc nfkc)' \
'--format=[Choose between the standard output format (\`default\`) and the Mnemonic MARC Text File Format (\`mnemonic\`). If no explicit selection is made, the \`mnemonic\` format is used for file extensions \`.mrk\` and \`.mrk.gz\`; otherwise, the output is in the standard format]:format:(mnemonic default)' \
'-o+[Write output to <path> instead of stdout]:path:_files' \
'--output=[Write output to <path> instead of stdout]:path:_files' \
'-l+[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--limit=[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--strsim-threshold=[The minimum score for string similarity comparisons. The value must be between 0 and 100]:value:_default' \
'--where=[An expression for filtering records]:predicate:_default' \
'--filter-normalization=[Transliterate the given filter or query expression into the specified Unicode normal form]:form:(nfd nfkd nfc nfkc)' \
'--compression=[Specify compression level]:n:_default' \
'-s[Skip invalid records that can'\''t be decoded]' \
'--skip-invalid[Skip invalid records that can'\''t be decoded]' \
'-p[If set, show a progress bar]' \
'--progress[If set, show a progress bar]' \
'-h[Print help]' \
'--help[Print help]' \
'*::input -- MARC21 files to be processed as input. If no file is specified, or if the filename is `-`, the data is read from standard input (`stdin`) by default:_files' \
&& ret=0
;;
(sample)
_arguments "${_arguments_options[@]}" : \
'--seed=[Initialize the RNG with a seed value to get deterministic random record]:number:_default' \
'-o+[Write output to FILENAME instead of stdout]:filename:_files' \
'--output=[Write output to FILENAME instead of stdout]:filename:_files' \
'-l+[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--limit=[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--strsim-threshold=[The minimum score for string similarity comparisons. The value must be between 0 and 100]:value:_default' \
'--where=[An expression for filtering records]:predicate:_default' \
'--filter-normalization=[Transliterate the given filter or query expression into the specified Unicode normal form]:form:(nfd nfkd nfc nfkc)' \
'--compression=[Specify compression level]:n:_default' \
'-s[Skip invalid records that can'\''t be decoded]' \
'--skip-invalid[Skip invalid records that can'\''t be decoded]' \
'-p[If set, show a progress bar]' \
'--progress[If set, show a progress bar]' \
'-h[Print help]' \
'--help[Print help]' \
':sample_size -- Sample size:_default' \
'*::path:_files' \
&& ret=0
;;
(select)
_arguments "${_arguments_options[@]}" : \
'--separator=[Sets the separator used for squashing of repeated subfield values into a single value. Note that it'\''s possible to use the empty string as a separator]:SEPARATOR:_default' \
'--quote-style=[The quoting style to use when writing CSV/TSV]:QUOTE_STYLE:(always necessary non-numeric never)' \
'-H+[Insert a header row before the data. The header should be entered as a comma-separated list. Leading and trailing spaces in each column are automatically removed. Note that This option takes precedence over column names specified using an \`AS\` clause]:header:_default' \
'--header=[Insert a header row before the data. The header should be entered as a comma-separated list. Leading and trailing spaces in each column are automatically removed. Note that This option takes precedence over column names specified using an \`AS\` clause]:header:_default' \
'-o+[Write output to <path> instead of stdout]:path:_files' \
'--output=[Write output to <path> instead of stdout]:path:_files' \
'-l+[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--limit=[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--strsim-threshold=[The minimum score for string similarity comparisons. The value must be between 0 and 100]:value:_default' \
'--where=[An expression for filtering records]:predicate:_default' \
'--filter-normalization=[Transliterate the given filter or query expression into the specified Unicode normal form]:form:(nfd nfkd nfc nfkc)' \
'--compression=[Specify compression level]:n:_default' \
'(--merge)--squash[Whether to squash all values of a repeated subfield into a single value or not. The separator can be specified by the \`--separator\` option]' \
'(--squash)--merge[If set, all values of a column are merged into a single value. The separator can be specified by the \`--separator\`]' \
'--tsv[Write output tab-separated (TSV)]' \
'--no-header[If set, no header is included in the output]' \
'-s[Skip invalid records that can'\''t be decoded]' \
'--skip-invalid[Skip invalid records that can'\''t be decoded]' \
'-p[If set, show a progress bar]' \
'--progress[If set, show a progress bar]' \
'-h[Print help (see more with '\''--help'\'')]' \
'--help[Print help (see more with '\''--help'\'')]' \
':query -- A query expression:_default' \
'*::filenames:_files' \
&& ret=0
;;
(skosify)
_arguments "${_arguments_options[@]}" : \
'-c+[]:CONFIG:_files' \
'--config=[]:CONFIG:_files' \
'--format=[]:FORMAT:(turtle nt)' \
'-o+[Write output to <filename> instead of stdout]:filename:_files' \
'--output=[Write output to <filename> instead of stdout]:filename:_files' \
'-l+[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--limit=[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--strsim-threshold=[The minimum score for string similarity comparisons. The value must be between 0 and 100]:value:_default' \
'--where=[An expression for filtering records]:predicate:_default' \
'--filter-normalization=[Transliterate the given filter or query expression into the specified Unicode normal form]:form:(nfd nfkd nfc nfkc)' \
'--compression=[Specify compression level]:n:_default' \
'-s[Skip invalid records that can'\''t be decoded]' \
'--skip-invalid[Skip invalid records that can'\''t be decoded]' \
'-p[If set, show a progress bar]' \
'--progress[If set, show a progress bar]' \
'-h[Print help]' \
'--help[Print help]' \
'*::input -- MARC21 files to be processed as input. If no file is specified, or if the filename is `-`, the data is read from standard input (`stdin`) by default:_files' \
&& ret=0
;;
(split)
_arguments "${_arguments_options[@]}" : \
'--filename=[Filename template ("{}" is replaced by the chunk number)]:template:_default' \
'-o+[Write partitions into <path>]:path:_files' \
'--outdir=[Write partitions into <path>]:path:_files' \
'-l+[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--limit=[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--strsim-threshold=[The minimum score for string similarity comparisons. The value must be between 0 and 100]:value:_default' \
'--where=[An expression for filtering records]:predicate:_default' \
'--filter-normalization=[Transliterate the given filter or query expression into the specified Unicode normal form]:form:(nfd nfkd nfc nfkc)' \
'--compression=[Specify compression level]:n:_default' \
'-s[Skip invalid records that can'\''t be decoded]' \
'--skip-invalid[Skip invalid records that can'\''t be decoded]' \
'-p[If set, show a progress bar]' \
'--progress[If set, show a progress bar]' \
'-h[Print help (see more with '\''--help'\'')]' \
'--help[Print help (see more with '\''--help'\'')]' \
':chunk_size -- Chunk size:_default' \
'*::paths:_files' \
&& ret=0
;;
(build-completion)
_arguments "${_arguments_options[@]}" : \
'-o+[Write output to <filename>]:filename:_files' \
'--output=[Write output to <filename>]:filename:_files' \
'-h[Print help]' \
'--help[Print help]' \
':shell -- Output the shell completion file for the given shell:(bash elvish fish powershell zsh)' \
&& ret=0
;;
(build-man)
_arguments "${_arguments_options[@]}" : \
'-o+[Write output to <path>]:path:_files' \
'--outdir=[Write output to <path>]:path:_files' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
        esac
    ;;
esac
}

(( $+functions[_marc21_commands] )) ||
_marc21_commands() {
    local commands; commands=(
'check:Validate records against rule sets' \
'concat:Concatenate records from multiple inputs' \
'cat:Concatenate records from multiple inputs' \
'count:Prints the number of records in the input data' \
'cnt:Prints the number of records in the input data' \
'dedup:Remove duplicate records from the input' \
'describe:Creates a frequency table of all subfield codes' \
'filter:Filter records that fulfill a specified condition' \
'frequency:Compute a frequency table of values' \
'freq:Compute a frequency table of values' \
'glimpse:Print a dense preview of a data field' \
'grep:Search for records whose values match a pattern' \
'hash:Compute SHA-256 checksum of records' \
'invalid:Output invalid records that cannot be decoded' \
'partition:Partition records by values' \
'print:Print records in human readable format' \
'sample:Select a random permutation of records' \
'select:Transforms records into CSV or TSV format' \
'skosify:Convert records to SKOS/RDF' \
'split:Splits a list of records into chunks' \
'build-completion:Generate shell completions (e.g. Bash or ZSH)' \
'build-man:' \
    )
    _describe -t commands 'marc21 commands' commands "$@"
}
(( $+functions[_marc21__subcmd__build-completion_commands] )) ||
_marc21__subcmd__build-completion_commands() {
    local commands; commands=()
    _describe -t commands 'marc21 build-completion commands' commands "$@"
}
(( $+functions[_marc21__subcmd__build-man_commands] )) ||
_marc21__subcmd__build-man_commands() {
    local commands; commands=()
    _describe -t commands 'marc21 build-man commands' commands "$@"
}
(( $+functions[_marc21__subcmd__check_commands] )) ||
_marc21__subcmd__check_commands() {
    local commands; commands=()
    _describe -t commands 'marc21 check commands' commands "$@"
}
(( $+functions[_marc21__subcmd__concat_commands] )) ||
_marc21__subcmd__concat_commands() {
    local commands; commands=()
    _describe -t commands 'marc21 concat commands' commands "$@"
}
(( $+functions[_marc21__subcmd__count_commands] )) ||
_marc21__subcmd__count_commands() {
    local commands; commands=()
    _describe -t commands 'marc21 count commands' commands "$@"
}
(( $+functions[_marc21__subcmd__dedup_commands] )) ||
_marc21__subcmd__dedup_commands() {
    local commands; commands=()
    _describe -t commands 'marc21 dedup commands' commands "$@"
}
(( $+functions[_marc21__subcmd__describe_commands] )) ||
_marc21__subcmd__describe_commands() {
    local commands; commands=()
    _describe -t commands 'marc21 describe commands' commands "$@"
}
(( $+functions[_marc21__subcmd__filter_commands] )) ||
_marc21__subcmd__filter_commands() {
    local commands; commands=()
    _describe -t commands 'marc21 filter commands' commands "$@"
}
(( $+functions[_marc21__subcmd__frequency_commands] )) ||
_marc21__subcmd__frequency_commands() {
    local commands; commands=()
    _describe -t commands 'marc21 frequency commands' commands "$@"
}
(( $+functions[_marc21__subcmd__glimpse_commands] )) ||
_marc21__subcmd__glimpse_commands() {
    local commands; commands=()
    _describe -t commands 'marc21 glimpse commands' commands "$@"
}
(( $+functions[_marc21__subcmd__grep_commands] )) ||
_marc21__subcmd__grep_commands() {
    local commands; commands=()
    _describe -t commands 'marc21 grep commands' commands "$@"
}
(( $+functions[_marc21__subcmd__hash_commands] )) ||
_marc21__subcmd__hash_commands() {
    local commands; commands=()
    _describe -t commands 'marc21 hash commands' commands "$@"
}
(( $+functions[_marc21__subcmd__invalid_commands] )) ||
_marc21__subcmd__invalid_commands() {
    local commands; commands=()
    _describe -t commands 'marc21 invalid commands' commands "$@"
}
(( $+functions[_marc21__subcmd__partition_commands] )) ||
_marc21__subcmd__partition_commands() {
    local commands; commands=()
    _describe -t commands 'marc21 partition commands' commands "$@"
}
(( $+functions[_marc21__subcmd__print_commands] )) ||
_marc21__subcmd__print_commands() {
    local commands; commands=()
    _describe -t commands 'marc21 print commands' commands "$@"
}
(( $+functions[_marc21__subcmd__sample_commands] )) ||
_marc21__subcmd__sample_commands() {
    local commands; commands=()
    _describe -t commands 'marc21 sample commands' commands "$@"
}
(( $+functions[_marc21__subcmd__select_commands] )) ||
_marc21__subcmd__select_commands() {
    local commands; commands=()
    _describe -t commands 'marc21 select commands' commands "$@"
}
(( $+functions[_marc21__subcmd__skosify_commands] )) ||
_marc21__subcmd__skosify_commands() {
    local commands; commands=()
    _describe -t commands 'marc21 skosify commands' commands "$@"
}
(( $+functions[_marc21__subcmd__split_commands] )) ||
_marc21__subcmd__split_commands() {
    local commands; commands=()
    _describe -t commands 'marc21 split commands' commands "$@"
}

if [ "$funcstack[1]" = "_marc21" ]; then
    _marc21 "$@"
else
    compdef _marc21 marc21
fi
