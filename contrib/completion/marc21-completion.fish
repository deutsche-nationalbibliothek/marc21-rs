# Print an optspec for argparse to handle cmd's options that are independent of any subcommand.
function __fish_marc21_global_optspecs
	string join \n h/help V/version
end

function __fish_marc21_needs_command
	# Figure out if the current invocation already has a command.
	set -l cmd (commandline -opc)
	set -e cmd[1]
	argparse -s (__fish_marc21_global_optspecs) -- $cmd 2>/dev/null
	or return
	if set -q argv[1]
		# Also print the command, so this can be used to figure out what it is.
		echo $argv[1]
		return 1
	end
	return 0
end

function __fish_marc21_using_subcommand
	set -l cmd (__fish_marc21_needs_command)
	test -z "$cmd"
	and return 1
	contains -- $cmd[1] $argv
end

complete -c marc21 -n "__fish_marc21_needs_command" -s h -l help -d 'Print help'
complete -c marc21 -n "__fish_marc21_needs_command" -s V -l version -d 'Print version'
complete -c marc21 -n "__fish_marc21_needs_command" -f -a "concat" -d 'Concatenate records from multiple inputs'
complete -c marc21 -n "__fish_marc21_needs_command" -f -a "cat" -d 'Concatenate records from multiple inputs'
complete -c marc21 -n "__fish_marc21_needs_command" -f -a "count" -d 'Prints the number of records in the input data'
complete -c marc21 -n "__fish_marc21_needs_command" -f -a "cnt" -d 'Prints the number of records in the input data'
complete -c marc21 -n "__fish_marc21_needs_command" -f -a "filter" -d 'Concatenate records from multiple inputs'
complete -c marc21 -n "__fish_marc21_needs_command" -f -a "hash" -d 'Compute SHA-256 checksum of records'
complete -c marc21 -n "__fish_marc21_needs_command" -f -a "invalid" -d 'Outputs invalid records that cannot be decoded'
complete -c marc21 -n "__fish_marc21_needs_command" -f -a "print" -d 'Print records in human readable format'
complete -c marc21 -n "__fish_marc21_needs_command" -f -a "sample" -d 'Selects a random permutation of records'
complete -c marc21 -n "__fish_marc21_needs_command" -f -a "split" -d 'Splits a list of records into chunks'
complete -c marc21 -n "__fish_marc21_needs_command" -f -a "build-completion" -d 'Generate shell completions (e.g. Bash or ZSH)'
complete -c marc21 -n "__fish_marc21_needs_command" -f -a "build-man"
complete -c marc21 -n "__fish_marc21_using_subcommand concat" -s o -l output -d 'Write output to <filename> instead of stdout' -r -F
complete -c marc21 -n "__fish_marc21_using_subcommand concat" -l strsim-threshold -d 'The minimum score for string similarity comparisons (0 <= score <= 100)' -r
complete -c marc21 -n "__fish_marc21_using_subcommand concat" -l where -d 'An expression for filtering records' -r
complete -c marc21 -n "__fish_marc21_using_subcommand concat" -s c -l compression -d 'Specify compression level' -r
complete -c marc21 -n "__fish_marc21_using_subcommand concat" -s s -l skip-invalid -d 'Skip invalid records that can\'t be decoded'
complete -c marc21 -n "__fish_marc21_using_subcommand concat" -s p -l progress -d 'If set, show a progress bar'
complete -c marc21 -n "__fish_marc21_using_subcommand concat" -s h -l help -d 'Print help'
complete -c marc21 -n "__fish_marc21_using_subcommand cat" -s o -l output -d 'Write output to <filename> instead of stdout' -r -F
complete -c marc21 -n "__fish_marc21_using_subcommand cat" -l strsim-threshold -d 'The minimum score for string similarity comparisons (0 <= score <= 100)' -r
complete -c marc21 -n "__fish_marc21_using_subcommand cat" -l where -d 'An expression for filtering records' -r
complete -c marc21 -n "__fish_marc21_using_subcommand cat" -s c -l compression -d 'Specify compression level' -r
complete -c marc21 -n "__fish_marc21_using_subcommand cat" -s s -l skip-invalid -d 'Skip invalid records that can\'t be decoded'
complete -c marc21 -n "__fish_marc21_using_subcommand cat" -s p -l progress -d 'If set, show a progress bar'
complete -c marc21 -n "__fish_marc21_using_subcommand cat" -s h -l help -d 'Print help'
complete -c marc21 -n "__fish_marc21_using_subcommand count" -s o -l output -d 'Write output to FILENAME instead of stdout' -r -F
complete -c marc21 -n "__fish_marc21_using_subcommand count" -l strsim-threshold -d 'The minimum score for string similarity comparisons (0 <= score <= 100)' -r
complete -c marc21 -n "__fish_marc21_using_subcommand count" -l where -d 'An expression for filtering records' -r
complete -c marc21 -n "__fish_marc21_using_subcommand count" -s c -l compression -d 'Specify compression level' -r
complete -c marc21 -n "__fish_marc21_using_subcommand count" -s s -l skip-invalid -d 'Skip invalid records that can\'t be decoded'
complete -c marc21 -n "__fish_marc21_using_subcommand count" -s p -l progress -d 'If set, show a progress bar'
complete -c marc21 -n "__fish_marc21_using_subcommand count" -s h -l help -d 'Print help'
complete -c marc21 -n "__fish_marc21_using_subcommand cnt" -s o -l output -d 'Write output to FILENAME instead of stdout' -r -F
complete -c marc21 -n "__fish_marc21_using_subcommand cnt" -l strsim-threshold -d 'The minimum score for string similarity comparisons (0 <= score <= 100)' -r
complete -c marc21 -n "__fish_marc21_using_subcommand cnt" -l where -d 'An expression for filtering records' -r
complete -c marc21 -n "__fish_marc21_using_subcommand cnt" -s c -l compression -d 'Specify compression level' -r
complete -c marc21 -n "__fish_marc21_using_subcommand cnt" -s s -l skip-invalid -d 'Skip invalid records that can\'t be decoded'
complete -c marc21 -n "__fish_marc21_using_subcommand cnt" -s p -l progress -d 'If set, show a progress bar'
complete -c marc21 -n "__fish_marc21_using_subcommand cnt" -s h -l help -d 'Print help'
complete -c marc21 -n "__fish_marc21_using_subcommand filter" -l strsim-threshold -d 'The minimum score for string similarity comparisons (0 <= score <= 100)' -r
complete -c marc21 -n "__fish_marc21_using_subcommand filter" -s o -l output -d 'Write output to FILENAME instead of stdout' -r -F
complete -c marc21 -n "__fish_marc21_using_subcommand filter" -s c -l compression -d 'Specify compression level' -r
complete -c marc21 -n "__fish_marc21_using_subcommand filter" -s s -l skip-invalid -d 'Skip invalid records that can\'t be decoded'
complete -c marc21 -n "__fish_marc21_using_subcommand filter" -s p -l progress -d 'If set, show a progress bar'
complete -c marc21 -n "__fish_marc21_using_subcommand filter" -s h -l help -d 'Print help'
complete -c marc21 -n "__fish_marc21_using_subcommand hash" -s o -l output -d 'Write output to FILENAME instead of stdout' -r -F
complete -c marc21 -n "__fish_marc21_using_subcommand hash" -l strsim-threshold -d 'The minimum score for string similarity comparisons (0 <= score <= 100)' -r
complete -c marc21 -n "__fish_marc21_using_subcommand hash" -l where -d 'An expression for filtering records' -r
complete -c marc21 -n "__fish_marc21_using_subcommand hash" -s c -l compression -d 'Specify compression level' -r
complete -c marc21 -n "__fish_marc21_using_subcommand hash" -l tsv -d 'Write output tab-separated (TSV)'
complete -c marc21 -n "__fish_marc21_using_subcommand hash" -s s -l skip-invalid -d 'Skip invalid records that can\'t be decoded'
complete -c marc21 -n "__fish_marc21_using_subcommand hash" -s p -l progress -d 'If set, show a progress bar'
complete -c marc21 -n "__fish_marc21_using_subcommand hash" -s h -l help -d 'Print help'
complete -c marc21 -n "__fish_marc21_using_subcommand invalid" -s o -l output -d 'Write output to FILENAME instead of stdout' -r -F
complete -c marc21 -n "__fish_marc21_using_subcommand invalid" -s c -l compression -d 'Specify compression level' -r
complete -c marc21 -n "__fish_marc21_using_subcommand invalid" -s p -l progress -d 'If set, show a progress bar'
complete -c marc21 -n "__fish_marc21_using_subcommand invalid" -s h -l help -d 'Print help'
complete -c marc21 -n "__fish_marc21_using_subcommand print" -s o -l output -d 'Write output to FILENAME instead of stdout' -r -F
complete -c marc21 -n "__fish_marc21_using_subcommand print" -l strsim-threshold -d 'The minimum score for string similarity comparisons (0 <= score <= 100)' -r
complete -c marc21 -n "__fish_marc21_using_subcommand print" -l where -d 'An expression for filtering records' -r
complete -c marc21 -n "__fish_marc21_using_subcommand print" -s c -l compression -d 'Specify compression level' -r
complete -c marc21 -n "__fish_marc21_using_subcommand print" -s s -l skip-invalid -d 'Skip invalid records that can\'t be decoded'
complete -c marc21 -n "__fish_marc21_using_subcommand print" -s p -l progress -d 'If set, show a progress bar'
complete -c marc21 -n "__fish_marc21_using_subcommand print" -s h -l help -d 'Print help'
complete -c marc21 -n "__fish_marc21_using_subcommand sample" -l seed -d 'Initialize the RNG with a seed value to get deterministic random record' -r
complete -c marc21 -n "__fish_marc21_using_subcommand sample" -s o -l output -d 'Write output to FILENAME instead of stdout' -r -F
complete -c marc21 -n "__fish_marc21_using_subcommand sample" -l strsim-threshold -d 'The minimum score for string similarity comparisons (0 <= score <= 100)' -r
complete -c marc21 -n "__fish_marc21_using_subcommand sample" -l where -d 'An expression for filtering records' -r
complete -c marc21 -n "__fish_marc21_using_subcommand sample" -s c -l compression -d 'Specify compression level' -r
complete -c marc21 -n "__fish_marc21_using_subcommand sample" -s s -l skip-invalid -d 'Skip invalid records that can\'t be decoded'
complete -c marc21 -n "__fish_marc21_using_subcommand sample" -s p -l progress -d 'If set, show a progress bar'
complete -c marc21 -n "__fish_marc21_using_subcommand sample" -s h -l help -d 'Print help'
complete -c marc21 -n "__fish_marc21_using_subcommand split" -l filename -d 'Filename template ("{}" is replaced by the chunk number)' -r
complete -c marc21 -n "__fish_marc21_using_subcommand split" -s o -l outdir -d 'Write partitions into <path>' -r -F
complete -c marc21 -n "__fish_marc21_using_subcommand split" -l strsim-threshold -d 'The minimum score for string similarity comparisons (0 <= score <= 100)' -r
complete -c marc21 -n "__fish_marc21_using_subcommand split" -l where -d 'An expression for filtering records' -r
complete -c marc21 -n "__fish_marc21_using_subcommand split" -s c -l compression -d 'Specify compression level' -r
complete -c marc21 -n "__fish_marc21_using_subcommand split" -s s -l skip-invalid -d 'Skip invalid records that can\'t be decoded'
complete -c marc21 -n "__fish_marc21_using_subcommand split" -s p -l progress -d 'If set, show a progress bar'
complete -c marc21 -n "__fish_marc21_using_subcommand split" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c marc21 -n "__fish_marc21_using_subcommand build-completion" -s o -l output -d 'Write output to <filename>' -r -F
complete -c marc21 -n "__fish_marc21_using_subcommand build-completion" -s h -l help -d 'Print help'
complete -c marc21 -n "__fish_marc21_using_subcommand build-man" -s o -l outdir -d 'Write output to <path>' -r -F
complete -c marc21 -n "__fish_marc21_using_subcommand build-man" -s h -l help -d 'Print help'
