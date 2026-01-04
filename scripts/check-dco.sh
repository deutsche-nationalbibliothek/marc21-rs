#!/usr/bin/env bash

set -euo pipefail

commits=$(git log origin/main \
    --grep 'Signed-off-by: ' --invert-grep --format='%H')

if [[ ! -z $commits ]]; then
    for commit in $commits; do
        >&2 echo "Missing 'Signed-off-by' tag for commit '${commit:0:24}'"
    done

    exit 1
fi


