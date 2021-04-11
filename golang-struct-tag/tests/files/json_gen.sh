#!/usr/bin/env bash

set -ex

# ./tests/files/json_gen.sh

script_path=$(cd $(dirname $0) ; pwd -P)
script_path_root="${script_path}/"

go run "${script_path_root}json.go"

cat "${script_path_root}json.go" | sed -n '/^type S struct {$/,/^}$/p' | sed '1d; $d; s/^[ \t]//g; s/[ \t]$//g; s/ int /\t/; /^$/d;' > "${script_path_root}json.txt"
