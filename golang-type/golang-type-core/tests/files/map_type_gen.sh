#!/usr/bin/env bash

# ./tests/files/map_type_gen.sh

set -ex

script_path=$(cd $(dirname $0) ; pwd -P)
script_path_root="${script_path}/"

go run "${script_path_root}map_type.go"

cat "${script_path_root}map_type.go" | sed -n '/^[ \t]*var _ /p' | sed 's/^[ \t]//g; s/[ \t]$//g; s/^[ \t]*var _ //; /^$/d;' > "${script_path_root}map_type.txt"
