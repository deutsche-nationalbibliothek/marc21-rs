_marc21() {
    local i cur prev opts cmd
    COMPREPLY=()
    if [[ "${BASH_VERSINFO[0]}" -ge 4 ]]; then
        cur="$2"
    else
        cur="${COMP_WORDS[COMP_CWORD]}"
    fi
    prev="$3"
    cmd=""
    opts=""

    for i in "${COMP_WORDS[@]:0:COMP_CWORD}"
    do
        case "${cmd},${i}" in
            ",$1")
                cmd="marc21"
                ;;
            marc21,build-completion)
                cmd="marc21__subcmd__build__subcmd__completion"
                ;;
            marc21,build-man)
                cmd="marc21__subcmd__build__subcmd__man"
                ;;
            marc21,cat)
                cmd="marc21__subcmd__concat"
                ;;
            marc21,cnt)
                cmd="marc21__subcmd__count"
                ;;
            marc21,concat)
                cmd="marc21__subcmd__concat"
                ;;
            marc21,count)
                cmd="marc21__subcmd__count"
                ;;
            marc21,dedup)
                cmd="marc21__subcmd__dedup"
                ;;
            marc21,describe)
                cmd="marc21__subcmd__describe"
                ;;
            marc21,filter)
                cmd="marc21__subcmd__filter"
                ;;
            marc21,freq)
                cmd="marc21__subcmd__frequency"
                ;;
            marc21,frequency)
                cmd="marc21__subcmd__frequency"
                ;;
            marc21,glimpse)
                cmd="marc21__subcmd__glimpse"
                ;;
            marc21,hash)
                cmd="marc21__subcmd__hash"
                ;;
            marc21,invalid)
                cmd="marc21__subcmd__invalid"
                ;;
            marc21,partition)
                cmd="marc21__subcmd__partition"
                ;;
            marc21,print)
                cmd="marc21__subcmd__print"
                ;;
            marc21,sample)
                cmd="marc21__subcmd__sample"
                ;;
            marc21,select)
                cmd="marc21__subcmd__select"
                ;;
            marc21,split)
                cmd="marc21__subcmd__split"
                ;;
            *)
                ;;
        esac
    done

    case "${cmd}" in
        marc21)
            opts="-h -V --help --version concat cat count cnt dedup describe filter frequency freq glimpse hash invalid partition print sample select split build-completion build-man"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        marc21__subcmd__build__subcmd__completion)
            opts="-o -h --output --help bash elvish fish powershell zsh"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -o)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        marc21__subcmd__build__subcmd__man)
            opts="-o -h --outdir --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --outdir)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -o)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        marc21__subcmd__concat)
            opts="-a -o -s -l -p -c -h --append --tee --output --skip-invalid --limit --strsim-threshold --where --filter-normalization --progress --compression --help [PATH]..."
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --tee)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --output)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -o)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --limit)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -l)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --strsim-threshold)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --where)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --filter-normalization)
                    COMPREPLY=($(compgen -W "nfd nfkd nfc nfkc" -- "${cur}"))
                    return 0
                    ;;
                --compression)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -c)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        marc21__subcmd__count)
            opts="-o -s -l -p -c -h --output --skip-invalid --limit --strsim-threshold --where --filter-normalization --progress --compression --help [PATH]..."
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -o)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --limit)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -l)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --strsim-threshold)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --where)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --filter-normalization)
                    COMPREPLY=($(compgen -W "nfd nfkd nfc nfkc" -- "${cur}"))
                    return 0
                    ;;
                --compression)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -c)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        marc21__subcmd__dedup)
            opts="-o -s -l -p -c -h --output --skip-invalid --limit --strsim-threshold --where --filter-normalization --progress --compression --help [PATH]..."
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -o)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --limit)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -l)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --strsim-threshold)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --where)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --filter-normalization)
                    COMPREPLY=($(compgen -W "nfd nfkd nfc nfkc" -- "${cur}"))
                    return 0
                    ;;
                --compression)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -c)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        marc21__subcmd__describe)
            opts="-o -s -l -p -c -h --tsv --output --skip-invalid --limit --strsim-threshold --where --filter-normalization --progress --compression --help [PATH]..."
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -o)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --limit)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -l)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --strsim-threshold)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --where)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --filter-normalization)
                    COMPREPLY=($(compgen -W "nfd nfkd nfc nfkc" -- "${cur}"))
                    return 0
                    ;;
                --compression)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -c)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        marc21__subcmd__filter)
            opts="-s -v -l -o -p -c -h --skip-invalid --invert-match --limit --strsim-threshold --filter-normalization --output --progress --compression --help <filter> [PATH]..."
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --limit)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -l)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --strsim-threshold)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --filter-normalization)
                    COMPREPLY=($(compgen -W "nfd nfkd nfc nfkc" -- "${cur}"))
                    return 0
                    ;;
                --output)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -o)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --compression)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -c)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        marc21__subcmd__frequency)
            opts="-u -r -t -n -H -o -s -l -p -c -h --unique --reverse --threshold --num --tsv --header --output --skip-invalid --limit --strsim-threshold --where --filter-normalization --progress --compression --help <QUERY> [PATH]..."
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --threshold)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -t)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --num)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -n)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --header)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -H)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --output)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -o)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --limit)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -l)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --strsim-threshold)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --where)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --filter-normalization)
                    COMPREPLY=($(compgen -W "nfd nfkd nfc nfkc" -- "${cur}"))
                    return 0
                    ;;
                --compression)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -c)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        marc21__subcmd__glimpse)
            opts="-n -o -s -l -p -c -h --max-values --output --skip-invalid --limit --strsim-threshold --where --filter-normalization --progress --compression --help <PATH> [INPUT]..."
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --max-values)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -n)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --output)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -o)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --limit)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -l)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --strsim-threshold)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --where)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --filter-normalization)
                    COMPREPLY=($(compgen -W "nfd nfkd nfc nfkc" -- "${cur}"))
                    return 0
                    ;;
                --compression)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -c)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        marc21__subcmd__hash)
            opts="-o -s -l -p -c -h --tsv --output --skip-invalid --limit --strsim-threshold --where --filter-normalization --progress --compression --help [PATH]..."
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -o)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --limit)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -l)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --strsim-threshold)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --where)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --filter-normalization)
                    COMPREPLY=($(compgen -W "nfd nfkd nfc nfkc" -- "${cur}"))
                    return 0
                    ;;
                --compression)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -c)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        marc21__subcmd__invalid)
            opts="-o -p -c -h --output --progress --compression --help [PATH]..."
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -o)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --compression)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -c)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        marc21__subcmd__partition)
            opts="-t -o -s -l -p -c -h --template --output --skip-invalid --limit --strsim-threshold --where --filter-normalization --progress --compression --help <PATH> [FILENAMES]..."
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --template)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -t)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --output)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -o)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --limit)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -l)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --strsim-threshold)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --where)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --filter-normalization)
                    COMPREPLY=($(compgen -W "nfd nfkd nfc nfkc" -- "${cur}"))
                    return 0
                    ;;
                --compression)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -c)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        marc21__subcmd__print)
            opts="-o -s -l -p -c -h --translit --output --skip-invalid --limit --strsim-threshold --where --filter-normalization --progress --compression --help [PATH]..."
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --translit)
                    COMPREPLY=($(compgen -W "nfd nfkd nfc nfkc" -- "${cur}"))
                    return 0
                    ;;
                --output)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -o)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --limit)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -l)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --strsim-threshold)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --where)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --filter-normalization)
                    COMPREPLY=($(compgen -W "nfd nfkd nfc nfkc" -- "${cur}"))
                    return 0
                    ;;
                --compression)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -c)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        marc21__subcmd__sample)
            opts="-o -s -l -p -c -h --seed --output --skip-invalid --limit --strsim-threshold --where --filter-normalization --progress --compression --help <n> [PATH]..."
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --seed)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --output)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -o)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --limit)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -l)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --strsim-threshold)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --where)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --filter-normalization)
                    COMPREPLY=($(compgen -W "nfd nfkd nfc nfkc" -- "${cur}"))
                    return 0
                    ;;
                --compression)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -c)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        marc21__subcmd__select)
            opts="-H -o -s -l -p -c -h --tsv --header --output --skip-invalid --limit --strsim-threshold --where --filter-normalization --progress --compression --help <QUERY> [FILENAMES]..."
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --header)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -H)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --output)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -o)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --limit)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -l)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --strsim-threshold)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --where)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --filter-normalization)
                    COMPREPLY=($(compgen -W "nfd nfkd nfc nfkc" -- "${cur}"))
                    return 0
                    ;;
                --compression)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -c)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        marc21__subcmd__split)
            opts="-o -s -l -p -c -h --filename --outdir --skip-invalid --limit --strsim-threshold --where --filter-normalization --progress --compression --help <CHUNK_SIZE> [PATHS]..."
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --filename)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --outdir)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -o)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --limit)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -l)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --strsim-threshold)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --where)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --filter-normalization)
                    COMPREPLY=($(compgen -W "nfd nfkd nfc nfkc" -- "${cur}"))
                    return 0
                    ;;
                --compression)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -c)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
    esac
}

if [[ "${BASH_VERSINFO[0]}" -eq 4 && "${BASH_VERSINFO[1]}" -ge 4 || "${BASH_VERSINFO[0]}" -gt 4 ]]; then
    complete -F _marc21 -o nosort -o bashdefault -o default marc21
else
    complete -F _marc21 -o bashdefault -o default marc21
fi
