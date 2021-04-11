#!/usr/bin/env bash

set -ex

# ./tests/files/type_names_gen.sh

script_path=$(cd $(dirname $0) ; pwd -P)
script_path_root="${script_path}/"

go run "${script_path_root}type_names.go"

cat "${script_path_root}type_names.go" | sed -n '/^[ \t]*var _ /p' | sed 's/^[ \t]//g; s/[ \t]$//g; s/^[ \t]*var _ //; /^$/d;' > "${script_path_root}type_names.txt"
