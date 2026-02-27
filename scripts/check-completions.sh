#!/bin/bash

set -euo pipefail
# set -x

cargo run -q -F build -- build-completion bash -o contrib/completion/marc21-completion.bash
cargo run -q -F build -- build-completion fish -o contrib/completion/marc21-completion.fish
cargo run -q -F build -- build-completion zsh -o contrib/completion/marc21-completion.zsh

if [[ ! -z "$(git status --porcelain --untracked-files)" ]]; then
  echo "completion scripts need to be updated!" >&2
  exit 1;
fi
