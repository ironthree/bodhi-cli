_bodhi-cli() {
    local i cur prev opts cmds
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    cmd=""
    opts=""

    for i in ${COMP_WORDS[@]}
    do
        case "${i}" in
            bodhi-cli)
                cmd="bodhi-cli"
                ;;
            
            comment)
                cmd+="__comment"
                ;;
            compose-info)
                cmd+="__compose__info"
                ;;
            compose-list)
                cmd+="__compose__list"
                ;;
            create-override)
                cmd+="__create__override"
                ;;
            create-update)
                cmd+="__create__update"
                ;;
            create-update-override)
                cmd+="__create__update__override"
                ;;
            edit-override)
                cmd+="__edit__override"
                ;;
            edit-update)
                cmd+="__edit__update"
                ;;
            expire-override)
                cmd+="__expire__override"
                ;;
            query-overrides)
                cmd+="__query__overrides"
                ;;
            query-updates)
                cmd+="__query__updates"
                ;;
            release-info)
                cmd+="__release__info"
                ;;
            release-list)
                cmd+="__release__list"
                ;;
            update-request)
                cmd+="__update__request"
                ;;
            waive-tests)
                cmd+="__waive__tests"
                ;;
            *)
                ;;
        esac
    done

    case "${cmd}" in
        bodhi-cli)
            opts=" -n -k -v -h -V  --staging --no-store-password --ignore-keyring --verbose --help --version --bodhi-url --login-url   comment compose-info compose-list create-override create-update-override create-update edit-override edit-update expire-override query-overrides query-updates release-info release-list update-request waive-tests"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                --bodhi-url)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --login-url)
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
        
        bodhi__cli__comment)
            opts=" -h -V  --help --version --update --text --karma  "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                --update)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --text)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --karma)
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
        bodhi__cli__compose__info)
            opts=" -h -V  --help --version --format  <release> <request> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                --format)
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
        bodhi__cli__compose__list)
            opts=" -h -V  --help --version --format  "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                --format)
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
        bodhi__cli__create__override)
            opts=" -h -V  --help --version --duration --notes  <nvr> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                --duration)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --notes)
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
        bodhi__cli__create__update)
            opts=" -h -V  --help --version --autokarma --autotime --bugs --builds --close-bugs --display-name --from-tag --notes --require-bugs --require-testcases --requirements --severity --stable-days --stable-karma --suggestion --unstable-karma --update-type  "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                --autokarma)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --autotime)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --bugs)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --builds)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --close-bugs)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --display-name)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --from-tag)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --notes)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --require-bugs)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --require-testcases)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --requirements)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --severity)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --stable-days)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --stable-karma)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --suggestion)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --unstable-karma)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --update-type)
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
        bodhi__cli__create__update__override)
            opts=" -h -V  --help --version --duration --notes  <alias> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                --duration)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --notes)
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
        bodhi__cli__edit__override)
            opts=" -h -V  --help --version --duration --notes  <nvr> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                --duration)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --notes)
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
        bodhi__cli__edit__update)
            opts=" -h -V  --help --version --add-bugs --add-builds --autokarma --autotime --close-bugs --display-name --notes --remove-bugs --remove-builds --requirements --severity --stable-days --stable-karma --suggestion --unstable-karma --update-type  <alias> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                --add-bugs)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --add-builds)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --autokarma)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --autotime)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --close-bugs)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --display-name)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --notes)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --remove-bugs)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --remove-builds)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --requirements)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --severity)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --stable-days)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --stable-karma)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --suggestion)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --unstable-karma)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --update-type)
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
        bodhi__cli__expire__override)
            opts=" -h -V  --help --version  <nvr> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
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
        bodhi__cli__query__overrides)
            opts=" -f -h -V  --force --help --version --builds --expired --format --releases --users  "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                --builds)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --expired)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --releases)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --users)
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
        bodhi__cli__query__updates)
            opts=" -f -h -V  --force --help --version --alias --bugs --builds --critpath --content-type --format --locked --modified-before --modified-since --packages --pushed --pushed-before --pushed-since --releases --request --severity --status --submitted-before --submitted-since --suggestion --type --users  "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                --alias)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --bugs)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --builds)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --critpath)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --content-type)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --locked)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --modified-before)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --modified-since)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --packages)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --pushed)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --pushed-before)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --pushed-since)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --releases)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --request)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --severity)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --status)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --submitted-before)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --submitted-since)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --suggestion)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --type)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --users)
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
        bodhi__cli__release__info)
            opts=" -h -V  --help --version --format  <release> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                --format)
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
        bodhi__cli__release__list)
            opts=" -h -V  --help --version --format  "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                --format)
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
        bodhi__cli__update__request)
            opts=" -h -V  --help --version  <alias> <request> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
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
        bodhi__cli__waive__tests)
            opts=" -h -V  --help --version --tests  <alias> <comment> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                --tests)
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

complete -F _bodhi-cli -o bashdefault -o default bodhi-cli
