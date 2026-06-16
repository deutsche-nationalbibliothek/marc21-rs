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
            (concat)
_arguments "${_arguments_options[@]}" : \
'-o+[Write output to <filename> instead of stdout]:filename:_files' \
'--output=[Write output to <filename> instead of stdout]:filename:_files' \
'-l+[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--limit=[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--strsim-threshold=[The minimum score for string similarity comparisons (0 <= score <= 100)]:n:_default' \
'--where=[An expression for filtering records]:predicate:_default' \
'-c+[Specify compression level]:n:_default' \
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
(cat)
_arguments "${_arguments_options[@]}" : \
'-o+[Write output to <filename> instead of stdout]:filename:_files' \
'--output=[Write output to <filename> instead of stdout]:filename:_files' \
'-l+[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--limit=[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--strsim-threshold=[The minimum score for string similarity comparisons (0 <= score <= 100)]:n:_default' \
'--where=[An expression for filtering records]:predicate:_default' \
'-c+[Specify compression level]:n:_default' \
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
(count)
_arguments "${_arguments_options[@]}" : \
'-o+[Write output to FILENAME instead of stdout]:FILENAME:_files' \
'--output=[Write output to FILENAME instead of stdout]:FILENAME:_files' \
'-l+[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--limit=[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--strsim-threshold=[The minimum score for string similarity comparisons (0 <= score <= 100)]:n:_default' \
'--where=[An expression for filtering records]:predicate:_default' \
'-c+[Specify compression level]:n:_default' \
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
'--strsim-threshold=[The minimum score for string similarity comparisons (0 <= score <= 100)]:n:_default' \
'--where=[An expression for filtering records]:predicate:_default' \
'-c+[Specify compression level]:n:_default' \
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
'-o+[Write output to FILENAME instead of stdout]:path:_files' \
'--output=[Write output to FILENAME instead of stdout]:path:_files' \
'-l+[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--limit=[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--strsim-threshold=[The minimum score for string similarity comparisons (0 <= score <= 100)]:n:_default' \
'--where=[An expression for filtering records]:predicate:_default' \
'-c+[Specify compression level]:n:_default' \
'--compression=[Specify compression level]:n:_default' \
'-s[Skip invalid records that can'\''t be decoded]' \
'--skip-invalid[Skip invalid records that can'\''t be decoded]' \
'-p[If set, show a progress bar]' \
'--progress[If set, show a progress bar]' \
'-h[Print help (see more with '\''--help'\'')]' \
'--help[Print help (see more with '\''--help'\'')]' \
'*::path:_files' \
&& ret=0
;;
(describe)
_arguments "${_arguments_options[@]}" : \
'-o+[Write output to <path> instead of stdout]:path:_files' \
'--output=[Write output to <path> instead of stdout]:path:_files' \
'-l+[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--limit=[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--strsim-threshold=[The minimum score for string similarity comparisons (0 <= score <= 100)]:n:_default' \
'--where=[An expression for filtering records]:predicate:_default' \
'-c+[Specify compression level]:n:_default' \
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
'-o+[Write output to FILENAME instead of stdout]:FILENAME:_files' \
'--output=[Write output to FILENAME instead of stdout]:FILENAME:_files' \
'-c+[Specify compression level]:n:_default' \
'--compression=[Specify compression level]:n:_default' \
'-s[Skip invalid records that can'\''t be decoded]' \
'--skip-invalid[Skip invalid records that can'\''t be decoded]' \
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
'-H+[Insert a header row before the data. The header should be entered as a comma-separated list. Leading and trailing spaces in each column are automatically removed]:header:_default' \
'--header=[Insert a header row before the data. The header should be entered as a comma-separated list. Leading and trailing spaces in each column are automatically removed]:header:_default' \
'-o+[Write output to <path> instead of stdout]:path:_files' \
'--output=[Write output to <path> instead of stdout]:path:_files' \
'-l+[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--limit=[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--strsim-threshold=[The minimum score for string similarity comparisons (0 <= score <= 100)]:n:_default' \
'--where=[An expression for filtering records]:predicate:_default' \
'-c+[Specify compression level]:n:_default' \
'--compression=[Specify compression level]:n:_default' \
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
'-H+[Insert a header row before the data. The header should be entered as a comma-separated list. Leading and trailing spaces in each column are automatically removed]:header:_default' \
'--header=[Insert a header row before the data. The header should be entered as a comma-separated list. Leading and trailing spaces in each column are automatically removed]:header:_default' \
'-o+[Write output to <path> instead of stdout]:path:_files' \
'--output=[Write output to <path> instead of stdout]:path:_files' \
'-l+[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--limit=[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--strsim-threshold=[The minimum score for string similarity comparisons (0 <= score <= 100)]:n:_default' \
'--where=[An expression for filtering records]:predicate:_default' \
'-c+[Specify compression level]:n:_default' \
'--compression=[Specify compression level]:n:_default' \
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
(hash)
_arguments "${_arguments_options[@]}" : \
'-o+[Write output to FILENAME instead of stdout]:FILENAME:_files' \
'--output=[Write output to FILENAME instead of stdout]:FILENAME:_files' \
'-l+[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--limit=[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--strsim-threshold=[The minimum score for string similarity comparisons (0 <= score <= 100)]:n:_default' \
'--where=[An expression for filtering records]:predicate:_default' \
'-c+[Specify compression level]:n:_default' \
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
'-c+[Specify compression level]:n:_default' \
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
'--strsim-threshold=[The minimum score for string similarity comparisons (0 <= score <= 100)]:n:_default' \
'--where=[An expression for filtering records]:predicate:_default' \
'-c+[Specify compression level]:n:_default' \
'--compression=[Specify compression level]:n:_default' \
'-s[Skip invalid records that can'\''t be decoded]' \
'--skip-invalid[Skip invalid records that can'\''t be decoded]' \
'-p[If set, show a progress bar]' \
'--progress[If set, show a progress bar]' \
'-h[Print help (see more with '\''--help'\'')]' \
'--help[Print help (see more with '\''--help'\'')]' \
':path -- A MARC-21 Path expression:_default' \
'*::filenames:_files' \
&& ret=0
;;
(print)
_arguments "${_arguments_options[@]}" : \
'--translit=[Transliterate the output into the specified Unicode normal form]:form:(nfd nfkd nfc nfkc)' \
'-o+[Write output to <path> instead of stdout]:path:_files' \
'--output=[Write output to <path> instead of stdout]:path:_files' \
'-l+[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--limit=[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--strsim-threshold=[The minimum score for string similarity comparisons (0 <= score <= 100)]:n:_default' \
'--where=[An expression for filtering records]:predicate:_default' \
'-c+[Specify compression level]:n:_default' \
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
(sample)
_arguments "${_arguments_options[@]}" : \
'--seed=[Initialize the RNG with a seed value to get deterministic random record]:number:_default' \
'-o+[Write output to FILENAME instead of stdout]:filename:_files' \
'--output=[Write output to FILENAME instead of stdout]:filename:_files' \
'-l+[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--limit=[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--strsim-threshold=[The minimum score for string similarity comparisons (0 <= score <= 100)]:n:_default' \
'--where=[An expression for filtering records]:predicate:_default' \
'-c+[Specify compression level]:n:_default' \
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
'-H+[Insert a header row before the data. The header should be entered as a comma-separated list. Leading and trailing spaces in each column are automatically removed]:header:_default' \
'--header=[Insert a header row before the data. The header should be entered as a comma-separated list. Leading and trailing spaces in each column are automatically removed]:header:_default' \
'-o+[Write output to <path> instead of stdout]:path:_files' \
'--output=[Write output to <path> instead of stdout]:path:_files' \
'-l+[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--limit=[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--strsim-threshold=[The minimum score for string similarity comparisons (0 <= score <= 100)]:n:_default' \
'--where=[An expression for filtering records]:predicate:_default' \
'-c+[Specify compression level]:n:_default' \
'--compression=[Specify compression level]:n:_default' \
'--tsv[Write output tab-separated (TSV)]' \
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
(split)
_arguments "${_arguments_options[@]}" : \
'--filename=[Filename template ("{}" is replaced by the chunk number)]:template:_default' \
'-o+[Write partitions into <path>]:path:_files' \
'--outdir=[Write partitions into <path>]:path:_files' \
'-l+[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--limit=[Limit the result to first <n> records (a limit value \`0\` means no limit)]:n:_default' \
'--strsim-threshold=[The minimum score for string similarity comparisons (0 <= score <= 100)]:n:_default' \
'--where=[An expression for filtering records]:predicate:_default' \
'-c+[Specify compression level]:n:_default' \
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
'concat:Concatenate records from multiple inputs' \
'cat:Concatenate records from multiple inputs' \
'count:Prints the number of records in the input data' \
'cnt:Prints the number of records in the input data' \
'dedup:Remove duplicate records from the input' \
'describe:Creates a frequency table of all subfield codes' \
'filter:Filter records that fulfill a specified condition' \
'frequency:Compute a frequency table of values' \
'freq:Compute a frequency table of values' \
'hash:Compute SHA-256 checksum of records' \
'invalid:Output invalid records that cannot be decoded' \
'partition:Partition records by values' \
'print:Print records in human readable format' \
'sample:Select a random permutation of records' \
'select:Transforms records into CSV or TSV format' \
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
